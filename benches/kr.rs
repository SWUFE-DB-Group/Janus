use criterion::{Criterion, black_box, criterion_group, criterion_main};

use Janus::{euc_kr, tools};
use std::fs;

fn kr_benchmark(c: &mut Criterion) {
    let bytes = fs::read("heartless_euckr.txt").unwrap();
    let table = euc_kr::build_kr_table();
    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("kr::is_kr_iconv", |b| {
        b.iter(|| euc_kr::is_kr_iconv(black_box(&bytes)))
    });

    c.bench_function("kr::is_kr_rs", |b| {
        b.iter(|| euc_kr::is_kr_rs(black_box(&bytes)))
    });

    c.bench_function("kr::is_kr_range", |b| {
        b.iter(|| euc_kr::is_kr_range(black_box(&bytes)))
    });

    c.bench_function("kr::is_kr_lookup", |b| {
        b.iter(|| euc_kr::is_kr_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("kr::is_kr_lookup", |b| {
        b.iter(|| euc_kr::is_kr_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

criterion_group!(benches, kr_benchmark);
criterion_main!(benches);
