use criterion::{Criterion, black_box, criterion_group, criterion_main};

use Janus::gb18030::{is_gb18030_fsm, is_gb18030_iconv, is_gb18030_rs, is_gb18030_simd};
use std::fs;

fn gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("dream_gb2312.txt").unwrap();

    c.bench_function("gb18030::is_gb18030_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("gb18030::is_gb18030_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("gb18030::is_gb18030_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("gb18030::is_gb18030_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

fn random_0_gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_0.txt").unwrap();

    c.bench_function("random-gb-0::is_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("random-gb-0::is_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("random-gb-0::is_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("random-gb-0::is_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

fn random_10_gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_10.txt").unwrap();

    c.bench_function("random-gb-10::is_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("random-gb-10::is_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("random-gb-10::is_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("random-gb-10::is_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

fn random_20_gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_20.txt").unwrap();

    c.bench_function("random-gb-20::is_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("random-gb-20::is_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("random-gb-20::is_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("random-gb-20::is_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

fn random_50_gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_50.txt").unwrap();

    c.bench_function("random-gb-50::is_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("random-gb-50::is_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("random-gb-50::is_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("random-gb-50::is_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

fn random_70_gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("random/gb2312_70.txt").unwrap();

    c.bench_function("random-gb-70::is_iconv", |b| {
        b.iter(|| is_gb18030_iconv(black_box(&bytes)))
    });

    c.bench_function("random-gb-70::is_rs", |b| {
        b.iter(|| is_gb18030_rs(black_box(&bytes)))
    });

    c.bench_function("random-gb-70::is_fsm", |b| {
        b.iter(|| is_gb18030_fsm(black_box(&bytes)))
    });

    c.bench_function("random-gb-70::is_simd", |b| {
        b.iter(|| is_gb18030_simd(black_box(&bytes)))
    });
}

criterion_group!(benches, gb18030_benchmark);
criterion_main!(benches);
