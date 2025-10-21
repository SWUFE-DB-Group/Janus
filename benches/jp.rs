use criterion::{Criterion, black_box, criterion_group, criterion_main};

use Janus::euc_jp::{is_jp_iconv, is_jp_lookup, is_jp_range, is_jp_rs, is_jp_simd};
use Janus::{euc_jp, tools};
use std::fs;

fn jp_benchmark(c: &mut Criterion) {
    let bytes = fs::read("meian_eucjp.txt").unwrap();
    let table = euc_jp::build_jp_table();
    let i32_table = tools::build_i32_table_from_bool(&table);

    c.bench_function("jp::is_jp_iconv", |b| {
        b.iter(|| is_jp_iconv(black_box(&bytes)))
    });

    c.bench_function("jp::is_jp_rs", |b| b.iter(|| is_jp_rs(black_box(&bytes))));

    c.bench_function("jp::is_js_range", |b| {
        b.iter(|| is_jp_range(black_box(&bytes)))
    });

    c.bench_function("jp::is_jp_lookup", |b| {
        b.iter(|| is_jp_lookup(black_box(&bytes), black_box(&table)))
    });

    c.bench_function("jp::is_jp_simd", |b| {
        b.iter(|| is_jp_simd(black_box(&bytes), black_box(&i32_table)))
    });
}

criterion_group!(benches, jp_benchmark);
criterion_main!(benches);
