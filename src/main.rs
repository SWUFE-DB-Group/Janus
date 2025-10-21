use std::fs;

use Janus::gb2312;
use Janus::tools;
fn run_gb2312() {
    let bytes = fs::read("dream-gb2312.txt").unwrap();
    let result = gb2312::is_gb2312_iconv(&bytes);
    println!("is_gb2312_iconv: {:?}", result);

    let result = gb2312::is_gb2312_rs(&bytes);
    println!("is_gb2312_rs: {:?}", result);

    let result = gb2312::is_gb2312_range(&bytes);
    println!("is_gb2312_range: {:?}", result);

    let table = gb2312::build_gb2312_table();
    let result = gb2312::is_gb2312_lookup(&bytes, &table);
    println!("is_gb2312_lookup: {:?}", result);

    let i32_table = tools::build_i32_table_from_bool(&table);
    let result = gb2312::is_gb2312_simd(&bytes, &i32_table);
    println!("is_gb2312_simd: {:?}", result);
}
fn main() {
    run_gb2312();
}
