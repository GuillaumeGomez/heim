use criterion::{criterion_group, Criterion};
use futures::prelude::*;

pub fn inner(c: &mut Criterion) {
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("platform", |b| {
        b.iter(|| rt.block_on(heim::host::platform()))
    });

    c.bench_function("uptime", |b| b.iter(|| rt.block_on(heim::host::uptime())));

    c.bench_function("boot_time", |b| {
        b.iter(|| rt.block_on(heim::host::boot_time()))
    });

    c.bench_function("users", |b| {
        b.iter(|| {
            let stream = heim::host::users().for_each(|_| async {});
            rt.block_on(stream)
        })
    });
}

criterion_group!(bench, inner);
