# buffer-queue

A lock-free, fixed-capacity, async-aware channel library for Rust, supporting all four
producer/consumer topologies: SPSC, SPMC, MPSC, and MPMC.

The buffer core is fully heapless and stack-allocatable. Async wakeup is layered on top
via `AtomicWaker` (single-waiter sides) and `parking_lot::Mutex<VecDeque<Waker>>`
(multi-waiter sides), keeping the hot push/pop path free of any locking.

---

## Supported Modes

| Mode | Senders | Receivers | Buffer Heapless | Waker (recv side) | Waker (send side) |
|------|---------|-----------|-----------------|-------------------|-------------------|
| SPSC | 1       | 1         | ✓               | `AtomicWaker`     | `AtomicWaker`     |
| SPMC | 1       | many      | ✓               | `MultiWaker`      | `AtomicWaker`     |
| MPSC | many    | 1         | ✓               | `AtomicWaker`     | `MultiWaker`      |
| MPMC | many    | many      | ✓               | `MultiWaker`      | `MultiWaker`      |

`MultiWaker` = `parking_lot::Mutex<VecDeque<Waker>>` — heap-backed, only allocated for
modes that need it, and only touched during wakeup/registration, never on the data path.

---

## Architecture

### Buffer Core — `BufferQueue<T, Mode, N>`

A fixed-size ring buffer of `MaybeUninit<T>` slots. All synchronization is done via
atomics — no locks, no heap, no per-element metadata.

```
BufferQueue<T, Mode, N>
├── buf: UnsafeCell<[MaybeUninit<T>; N]>   — the actual data slots
└── markers: BufferMarkers<MarkerType, MASK_SIZE>
    ├── head: MarkerType         — where the next WRITE goes (atomic)
    ├── tail: MarkerType         — where the next READ comes from (atomic)
    ├── invalidated: AtomicBool  — signals shutdown
    ├── write_mask: [AtomicU8]   — one bit per slot: 1 = write in progress
    └── read_mask:  [AtomicU8]   — one bit per slot: 1 = read in progress
```

`MarkerType` is selected at compile time based on `N`:
- `N < 256`   → `AtomicU8`  markers
- `N < 65536` → `AtomicU16` markers
- etc.

Mask arrays are sized as `ceil(N / 8)` bytes each — total mask overhead is `2 * ceil(N/8)` bytes
regardless of `T`. For `N=64` that is 16 bytes total.

### Slot Lifecycle

Two independent bitmasks govern each slot:

```
write_mask bit = 1  →  pusher is actively writing this slot
read_mask  bit = 1  →  popper is actively reading this slot
```

The four valid states per slot:

| write | read | meaning                              |
|-------|------|--------------------------------------|
| 0     | 0    | idle — free to push or fully consumed |
| 1     | 0    | write in progress — poppers spin      |
| 0     | 1    | read in progress — wrap-around pushers spin |
| 1     | 1    | impossible by construction            |

**Push protocol:**
1. Atomically claim `head` slot (SP: relaxed load + store; MP: `fetch_update`)
2. Spin on `read_mask` until slot is not being read (wrap-around protection)
3. `Register` `write_mask` bit
4. `ptr::write` into the slot
5. `Unregister` `write_mask` bit with `Release` ordering
6. Advance `head` (SP only — MP already advanced it in step 1)

**Pop protocol:**
1. Atomically claim `tail` slot (SC: relaxed load + store; MC: `fetch_update`)
2. Spin on `write_mask` until slot is not being written
3. `Register` `read_mask` bit
4. `ptr::read` from the slot, wipe with `MaybeUninit::uninit()`
5. `Unregister` `read_mask` bit with `Release` ordering
6. Advance `tail` (SC only — MC already advanced it in step 1)

### Memory Ordering

| Operation | Ordering | Reason |
|-----------|----------|--------|
| `head`/`tail` load (own side, SP/SC) | `Relaxed` | No contention, single owner |
| `head`/`tail` load (other side) | `Acquire` | Must see latest advances |
| `fetch_update` on head/tail (MP/MC) | `AcqRel` / `Acquire` | RMW — read+write in one |
| `write_mask` / `read_mask` Register | `Relaxed` | Staking a claim, no data yet |
| `write_mask` / `read_mask` Unregister | `Release` | Publishes the completed operation |

### Channel Layer

Each mode has a concrete channel struct pairing the buffer with its wakers:

```rust
pub struct SpscChannel<T, const N: usize> {
    buf:        BufferQueue<T, SPSC, N>,
    recv_waker: SingleWaker,   // AtomicWaker
    send_waker: SingleWaker,   // AtomicWaker
}

pub struct SpmcChannel<T, const N: usize> {
    buf:        BufferQueue<T, SPMC, N>,
    recv_waker: MultiWaker,    // Mutex<VecDeque<Waker>>
    send_waker: SingleWaker,
}

pub struct MpscChannel<T, const N: usize> {
    buf:        BufferQueue<T, MPSC, N>,
    recv_waker: SingleWaker,
    send_waker: MultiWaker,    // Mutex<VecDeque<Waker>>
}

pub struct MpmcChannel<T, const N: usize> {
    buf:        BufferQueue<T, MPMC, N>,
    recv_waker: MultiWaker,
    send_waker: MultiWaker,
}
```

All channel structs are wrapped in `triomphe::Arc` (no weak ref count overhead).
Senders/receivers are `#[repr(transparent)]` wrappers over `Arc<XxxxChannel<T, N>>`.

### Message Flow

```
send(val).await
    │
    ▼
SendFuture::poll()
    ├─ buf.push(val)
    │       ├─ success → recv_waker.notify() → Poll::Ready(())
    │       └─ full    → send_waker.register(cx)
    │                    buf.push(val) [re-check, closes race window]
    │                       ├─ success → recv_waker.notify() → Poll::Ready(())
    │                       └─ full    → Poll::Pending [parked]
    │
    │   ... a pop happens elsewhere ...
    │   pop calls send_waker.notify() → executor re-polls SendFuture
    │
    └─ [re-polled] → buf.push(val) succeeds → Poll::Ready(())


recv().await
    │
    ▼
RecvFuture::poll()
    ├─ buf.pop()
    │       ├─ success → send_waker.notify() → Poll::Ready(val)
    │       └─ empty   → recv_waker.register(cx)
    │                    buf.pop() [re-check, closes race window]
    │                       ├─ success → send_waker.notify() → Poll::Ready(val)
    │                       └─ empty   → Poll::Pending [parked]
    │
    │   ... a push happens elsewhere ...
    │   push calls recv_waker.notify() → executor re-polls RecvFuture
    │
    └─ [re-polled] → buf.pop() succeeds → Poll::Ready(val)
```

The double-check after `register` closes the race window where data/space arrives
between the failed push/pop and the waker registration.

---

## API

```rust
// Construction
let (tx, rx) = spsc_channel::<T, N>();
let (tx, rx) = spmc_channel::<T, N>();  // rx: SpmcReceiver — implements Clone
let (tx, rx) = mpsc_channel::<T, N>();  // tx: MpscSender   — implements Clone
let (tx, rx) = mpmc_channel::<T, N>();  // both implement Clone

// Sync (non-blocking)
tx.try_send(val) -> Result<(), T>   // Err(val) if full or invalidated
rx.try_recv()    -> Option<T>       // None if empty or invalidated

// Async (backpressure)
tx.send(val).await                  // suspends if buffer full
rx.recv().await   -> T              // suspends if buffer empty
```

---

## Known Issues & TODOs

### 1. `MultiWaker` Duplicate Registration
`MultiWaker::register` pushes a new `Waker` into the queue on every call. If an
executor re-polls a future without an intervening wakeup (valid behavior), the same
task's waker is pushed twice. This causes a spurious double-wakeup — harmless but
wasteful. Fix: deduplicate via `Waker::will_wake` before pushing.

```rust
// TODO in MultiWaker::register:
let mut wakers = self.wakers.lock();
if !wakers.iter().any(|w| w.will_wake(cx.waker())) {
    wakers.push_back(cx.waker().clone());
}
```

### 2. No Disconnect Detection
When all senders drop, receivers have no way to know the channel is closed — `recv()`
will park forever on an empty buffer. Same in reverse (all receivers drop, sender parks
forever on a full buffer). Needs a sender/receiver refcount tracked separately from the
`Arc` refcount, and a check in `poll()` to return an error variant instead of
`Poll::Pending` when the other side is gone.

### 3. No Cancellation Safety Audit
`SendFuture` holds a value in `Option<T>`. If the future is dropped while `Pending`
(i.e. the `.await` is cancelled), the value is dropped with it — not pushed, not
returned to the caller. Depending on use case this may be acceptable, but it has not
been formally documented or tested.

### 4. `_sp_push` / `_sc_pop` Race On Wrap-Around (Partially Fixed)
The `write_mask` / `read_mask` two-mask protocol was introduced to prevent simultaneous
read and write on the same slot during wrap-around. The protocol is implemented and
correct for the push and pop sides independently. However, **this has not yet been stress-tested
under high concurrency** — formal verification or a loom-based concurrency test is needed
before marking this production-ready.

### 5. No `loom` Tests
The lock-free core has not been tested under `loom` (a model checker for concurrent Rust
code). All orderings have been reasoned about manually but have not been formally
checked. This is the most critical gap before any production use.

### 6. `BufferOperation<T>` Uses `&self` — `UnsafeCell` Soundness
The `push` and `pop` methods take `&self` rather than `&mut self`, relying on
`UnsafeCell` inside `BufferQueue` for interior mutability. This is intentional (required
for shared concurrent access through `Arc`) but the `unsafe` contracts — particularly
around `ptr::read`, `ptr::write`, and the mask protocol — have not been formally
audited for soundness under all possible interleavings.

---

## Dependencies

| Crate | Use |
|-------|-----|
| `triomphe` | `Arc` without weak reference count overhead |
| `atomic_waker` | Single-slot waker storage for SPSC/single-sided wakeup |
| `parking_lot` | Fast userspace `Mutex` for `MultiWaker` (no poisoning, smaller than `std::Mutex`) |

---

## Not Yet Implemented

- Timeout variants (`send_timeout`, `recv_timeout`)
- `select!`-compatible API
- Metrics / instrumentation hooks
- `no_std` support (currently depends on `std` via `VecDeque`, `parking_lot`)
- Benchmarks against `crossbeam-channel`, `tokio::sync::mpsc`, `flume`
