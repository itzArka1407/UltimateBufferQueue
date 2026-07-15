use buffer_queue::{mpsc_channel, spsc_channel};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use tokio::runtime::Runtime;

const CAP: usize = 1024;
const MESSAGES: usize = 10_000;

/// MPSC throughput: 1 producer, 1 consumer, MESSAGES u64s pushed through a
/// bounded channel of capacity CAP. Compares your MPSC against tokio's and flume's.
fn bench_mpsc_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("mpsc_throughput_10k_msgs");

    group.bench_function("buffer_queue::mpsc", |b| {
        b.to_async(&rt).iter_batched(
            || mpsc_channel::<u64, CAP>(),
            |(tx, rx)| async move {
                let sender = tokio::spawn(async move {
                    for i in 0..MESSAGES as u64 {
                        tx.send(i).await;
                    }
                });
                let receiver = tokio::spawn(async move {
                    for _ in 0..MESSAGES {
                        black_box(rx.recv().await);
                    }
                });
                let _ = tokio::join!(sender, receiver);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("tokio::sync::mpsc", |b| {
        b.to_async(&rt).iter_batched(
            || tokio::sync::mpsc::channel::<u64>(CAP),
            |(tx, mut rx)| async move {
                let sender = tokio::spawn(async move {
                    for i in 0..MESSAGES as u64 {
                        let _ = tx.send(i).await;
                    }
                });
                let receiver = tokio::spawn(async move {
                    for _ in 0..MESSAGES {
                        black_box(rx.recv().await);
                    }
                });
                let _ = tokio::join!(sender, receiver);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("flume::bounded", |b| {
        b.to_async(&rt).iter_batched(
            || flume::bounded::<u64>(CAP),
            |(tx, rx)| async move {
                let sender = tokio::spawn(async move {
                    for i in 0..MESSAGES as u64 {
                        let _ = tx.send_async(i).await;
                    }
                });
                let receiver = tokio::spawn(async move {
                    for _ in 0..MESSAGES {
                        black_box(rx.recv_async().await.ok());
                    }
                });
                let _ = tokio::join!(sender, receiver);
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

/// SPSC ping-pong latency: round-trip time for a single request/response pair,
/// repeated. This is the metric that matters most for "ultra-low-latency"
/// claims, since throughput can hide per-message latency under batching.
fn bench_spsc_ping_pong(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("spsc_ping_pong_1k_roundtrips");
    const ROUNDTRIPS: usize = 1_000;

    group.bench_function("buffer_queue::spsc", |b| {
        b.to_async(&rt).iter_batched(
            || {
                let (tx_a, rx_a) = spsc_channel::<u64, CAP>();
                let (tx_b, rx_b) = spsc_channel::<u64, CAP>();
                (tx_a, rx_a, tx_b, rx_b)
            },
            |(tx_a, rx_a, tx_b, rx_b)| async move {
                let ping = tokio::spawn(async move {
                    for i in 0..ROUNDTRIPS as u64 {
                        tx_a.send(i).await;
                        black_box(rx_b.recv().await);
                    }
                });
                let pong = tokio::spawn(async move {
                    for _ in 0..ROUNDTRIPS {
                        let v = rx_a.recv().await;
                        tx_b.send(v).await;
                    }
                });
                let _ = tokio::join!(ping, pong);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("tokio::sync::mpsc (as spsc)", |b| {
        b.to_async(&rt).iter_batched(
            || {
                let (tx_a, rx_a) = tokio::sync::mpsc::channel::<u64>(CAP);
                let (tx_b, rx_b) = tokio::sync::mpsc::channel::<u64>(CAP);
                (tx_a, rx_a, tx_b, rx_b)
            },
            |(tx_a, mut rx_a, tx_b, mut rx_b)| async move {
                let ping = tokio::spawn(async move {
                    for i in 0..ROUNDTRIPS as u64 {
                        let _ = tx_a.send(i).await;
                        black_box(rx_b.recv().await);
                    }
                });
                let pong = tokio::spawn(async move {
                    for _ in 0..ROUNDTRIPS {
                        if let Some(v) = rx_a.recv().await {
                            let _ = tx_b.send(v).await;
                        }
                    }
                });
                let _ = tokio::join!(ping, pong);
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

/// Sync-only throughput via try_send/try_recv, no async runtime overhead at all.
/// This isolates the lock-free core's raw performance against crossbeam's ArrayQueue,
/// which is the fairest possible comparison since neither pays waker/executor cost here.
fn bench_sync_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("sync_try_send_recv_1024_items");

    group.bench_function("buffer_queue::mpsc (try_send/try_recv)", |b| {
        b.iter_batched(
            || mpsc_channel::<u64, CAP>(),
            |(tx, rx)| {
                for i in 0..CAP as u64 {
                    let _ = tx.try_send(i);
                }
                for _ in 0..CAP {
                    black_box(rx.try_recv());
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("crossbeam::ArrayQueue", |b| {
        b.iter_batched(
            || crossbeam_queue::ArrayQueue::<u64>::new(CAP),
            |q| {
                for i in 0..CAP as u64 {
                    let _ = q.push(i);
                }
                for _ in 0..CAP {
                    black_box(q.pop());
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_mpsc_throughput,
    bench_spsc_ping_pong,
    bench_sync_throughput
);
criterion_main!(benches);
