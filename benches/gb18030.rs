use criterion::{Criterion, black_box, criterion_group, criterion_main};

use Janus::gb18030::{is_gb18030_fsm, is_gb18030_iconv, is_gb18030_rs, is_gb18030_simd};
use std::fs;

fn gb18030_benchmark(c: &mut Criterion) {
    let bytes = fs::read("dream-gb2312.txt").unwrap();

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

criterion_group!(benches, gb18030_benchmark);
criterion_main!(benches);
