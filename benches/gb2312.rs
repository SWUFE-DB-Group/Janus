use criterion::{Criterion, black_box, criterion_group, criterion_main};

use Janus::{gb2312, tools};
use std::fs;

fn gb2312_benchmark(c: &mut Criterion) {
    let bytes = fs::read("dream-gb2312.txt").unwrap();

    let table = gb2312::build_gb2312_table();

    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("gb2312::is_gb2312_iconv", |b| {
        b.iter(|| gb2312::is_gb2312_iconv(black_box(&bytes)))
    });

    c.bench_function("gb2312::is_gb2312_rs", |b| {
        b.iter(|| gb2312::is_gb2312_rs(black_box(&bytes)))
    });

    c.bench_function("gb2312::is_gb2312_range", |b| {
        b.iter(|| gb2312::is_gb2312_range(black_box(&bytes)))
    });

    c.bench_function("gb2312::is_gb2312_lookup", |b| {
        b.iter(|| gb2312::is_gb2312_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("gb2312::is_gb2312_simd", |b| {
        b.iter(|| gb2312::is_gb2312_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

criterion_group!(benches, gb2312_benchmark);
criterion_main!(benches);
