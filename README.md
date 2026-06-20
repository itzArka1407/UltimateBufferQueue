## KNOWN ISSUE — Notifier correctness for multi-receiver modes (SPMC, MPMC)

`AtomicWaker` only stores a single registered `Waker`. It is only safe to use
on a side of the channel where **at most one task at a time** will ever call
`register()`.

Safe as-is:
- SPSC: one sender, one receiver — single registrant on both sides.
- MPSC: many senders, ONE receiver — recv-side waker has a single registrant.

BROKEN as currently designed:
- SPMC / MPMC: multiple receivers can be parked waiting simultaneously.
  Each call to `register()` overwrites the previous receiver's stored Waker.
  `notify_one()` only wakes the most recently registered receiver — any
  earlier-registered receiver is silently forgotten and may hang forever,
  even if data becomes available, unless a later unrelated push happens to
  wake it.

This is a correctness bug (missed wakeups / potential permanent stalls),
not a performance issue.

### Fix required
Multi-receiver modes need a FIFO multi-waker structure instead of
`AtomicWaker`, e.g.:

    pub struct WakerQueue {
        inner: Mutex<VecDeque<Waker>>,
    }
    // register() -> push_back
    // notify_one() -> pop_front + wake()

This must be heap-backed (`VecDeque`) since the number of simultaneously
parked receivers is unbounded at compile time — this is unavoidable for
any correct multi-waiter design, not specific to this implementation.

Net effect: `BufferQueue` itself stays fully heapless in all modes.
Only the notifier for SPMC/MPMC needs heap allocation + a Mutex
(touched only on register/wake, never on the hot push/pop path).

### Design decision still open
Decide whether `send()` needs backpressure (suspend when buffer is full)
or whether senders should always be non-blocking (fail-fast like `try_send`,
return Err(Full) instead of awaiting space).

- If senders never need to wait → only ONE notifier per channel needed
  (recv-side: registered on empty, woken on push).
- If senders DO need to wait on a full buffer → a SECOND notifier is needed
  on the send side (registered on full, woken on pop), and the same
  single-vs-multi-registrant rule applies there too:
    - SPMC (single sender) → AtomicWaker is fine for the send-side waker.
    - MPSC/MPMC (multiple senders) → send-side waker also needs the
      multi-waker fix above.

TODO: pick one of the above before implementing `send()`/`recv()` futures
for SPMC and MPMC. SPSC and MPSC can proceed with AtomicWaker as-is.
