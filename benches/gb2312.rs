use criterion::{Criterion, black_box, criterion_group, criterion_main};

use Janus::gb18030::{is_gb18030_fsm, is_gb18030_iconv, is_gb18030_rs, is_gb18030_simd};
use Janus::{gb2312, tools};
use std::fs;

fn gb2312_benchmark(c: &mut Criterion) {
    let bytes = fs::read("dream_gb2312.txt").unwrap();

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

fn random_0_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_0.txt").unwrap();

    let table = gb2312::build_gb2312_table();

    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("random0::is_iconv", |b| {
        b.iter(|| gb2312::is_gb2312_iconv(black_box(&bytes)))
    });

    c.bench_function("random0::is_rs", |b| {
        b.iter(|| gb2312::is_gb2312_rs(black_box(&bytes)))
    });

    c.bench_function("random0::is_range", |b| {
        b.iter(|| gb2312::is_gb2312_range(black_box(&bytes)))
    });

    c.bench_function("random0::is_lookup", |b| {
        b.iter(|| gb2312::is_gb2312_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("random0::is_simd", |b| {
        b.iter(|| gb2312::is_gb2312_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

fn random_10_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_10.txt").unwrap();

    let table = gb2312::build_gb2312_table();

    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("random10::is_iconv", |b| {
        b.iter(|| gb2312::is_gb2312_iconv(black_box(&bytes)))
    });

    c.bench_function("random10::is_rs", |b| {
        b.iter(|| gb2312::is_gb2312_rs(black_box(&bytes)))
    });

    c.bench_function("random10::is_range", |b| {
        b.iter(|| gb2312::is_gb2312_range(black_box(&bytes)))
    });

    c.bench_function("random10::is_lookup", |b| {
        b.iter(|| gb2312::is_gb2312_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("random10::is_simd", |b| {
        b.iter(|| gb2312::is_gb2312_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

fn random_20_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_20.txt").unwrap();

    let table = gb2312::build_gb2312_table();

    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("random20::is_iconv", |b| {
        b.iter(|| gb2312::is_gb2312_iconv(black_box(&bytes)))
    });

    c.bench_function("random20::is_rs", |b| {
        b.iter(|| gb2312::is_gb2312_rs(black_box(&bytes)))
    });

    c.bench_function("random20::is_range", |b| {
        b.iter(|| gb2312::is_gb2312_range(black_box(&bytes)))
    });

    c.bench_function("random20::is_lookup", |b| {
        b.iter(|| gb2312::is_gb2312_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("random20::is_simd", |b| {
        b.iter(|| gb2312::is_gb2312_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

fn random_50_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_50.txt").unwrap();

    let table = gb2312::build_gb2312_table();

    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("random50::is_iconv", |b| {
        b.iter(|| gb2312::is_gb2312_iconv(black_box(&bytes)))
    });

    c.bench_function("random50::is_rs", |b| {
        b.iter(|| gb2312::is_gb2312_rs(black_box(&bytes)))
    });

    c.bench_function("random50::is_range", |b| {
        b.iter(|| gb2312::is_gb2312_range(black_box(&bytes)))
    });

    c.bench_function("random50::is_lookup", |b| {
        b.iter(|| gb2312::is_gb2312_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("random50::is_simd", |b| {
        b.iter(|| gb2312::is_gb2312_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

fn random_70_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_70.txt").unwrap();

    let table = gb2312::build_gb2312_table();

    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("random70::is_iconv", |b| {
        b.iter(|| gb2312::is_gb2312_iconv(black_box(&bytes)))
    });

    c.bench_function("random70::is_rs", |b| {
        b.iter(|| gb2312::is_gb2312_rs(black_box(&bytes)))
    });

    c.bench_function("random70::is_range", |b| {
        b.iter(|| gb2312::is_gb2312_range(black_box(&bytes)))
    });

    c.bench_function("random70::is_lookup", |b| {
        b.iter(|| gb2312::is_gb2312_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("random70::is_simd", |b| {
        b.iter(|| gb2312::is_gb2312_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

fn random_100_gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_100.txt").unwrap();

    c.bench_function("random-gb-full::is_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("random-gb-full::is_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("random-gb-full::is_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("random-gb-full::is_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

criterion_group!(benches, gb2312_benchmark);
criterion_main!(benches);
