# Janus

> A Two-Level Character Validation Framework
for Legacy CJK Encodings and Short-Texts


This repo is mainly used for structural
encoding validation for CJK (`StruVal-C`) in Rust. And `SemVal-S` can be found at [LGBench](https://github.com/SWUFE-DB-Group/LGBench).

## How to benchmark it

```shell
cargo bench 
```

If you want to evaluate a specific encoding, you can use `cargo bench -- <name>`, in which `<name>` can be `gb2312`,
`gb18030`, `jp` and `kr`.

## Performance reports

Three real-world datasets are from public domain novels in CJK literary world.

| Dataset  | Novel              | Original Title | File                  | Size   | Encoding |
|-----------|-----------------------------|----------------|-----------------------|--------|-----------|
| Dream-C   | Dream of the Red Chamber    | 红楼梦            | `dream_gb2312.txt`    | 1.75 MB | GB2312    |
| Light-J   | Light and Darkness          | 明暗             | `meian_eucjp.txt`     | 744 KB  | EUC-JP    |
| Heart-K   | The Heartless               | 무정             | `heartless_euckr.txt` | 555 KB  | EUC-KR    |

> [!NOTE]
> Since the files are NOT encoded with UTF-8, your editor may display them with *mojibake*.

Take `GB2312` for example, when running on a Linux machine with 64GB memory and Intel i9-12900K CPU, 
the results are summerized:

- [iconv](https://man7.org/linux/man-pages/man1/iconv.1.html): 0.65 GiB/s
- [encoding_rs](https://github.com/hsivonen/encoding_rs): 0.58 GiB/s
- `range`: 1.04 GiB/s
- `lookup`: 3.36 GiB/s
- `simd`: 9.01 GiB/s

### How to run benchmarks over random inputs

First generate the random data:

```shell
python3 random-data-generate.py
```

By default, the benchmarks over random inputs are not enabled, and currently you have to add them manually. For example,

```rust
criterion_group!(benches, random_0_benchmark);
```
