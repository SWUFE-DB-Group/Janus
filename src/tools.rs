use libc::{c_char, c_void, size_t};
use std::arch::x86_64::{__m128i, __m256i};
use std::ffi::CString;

use encoding_rs::Encoding;

type IconvT = *mut c_void;

unsafe extern "C" {
    fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> IconvT;
    fn iconv(
        cd: IconvT,
        inbuf: *mut *mut c_char,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut c_char,
        outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(cd: IconvT) -> i32;
}

pub fn is_encoding_iconv(data: &[u8], targeted_encoding: &str) -> bool {
    let len = data.len();
    let utf8_len = (len as f32 * 2.5).ceil() as usize;
    let mut utf8_str = vec![0u8; utf8_len];

    let cd = unsafe {
        let tocode = CString::new("UTF-8").unwrap();
        let fromcode = CString::new(targeted_encoding).unwrap();
        iconv_open(tocode.as_ptr(), fromcode.as_ptr())
    };

    let mut inbuf = data.as_ptr() as *mut c_char;
    let mut outbuf = utf8_str.as_mut_ptr() as *mut c_char;
    let mut inbytesleft = len;
    let mut outbytesleft = utf8_len;

    unsafe {
        iconv(
            cd,
            &mut inbuf,
            &mut inbytesleft,
            &mut outbuf,
            &mut outbytesleft,
        );
    }

    unsafe {
        iconv_close(cd);
    }
    inbytesleft == 0
}

pub fn is_encoding_rs(data: &[u8], targeted_encoding: &'static Encoding) -> bool {
    let (cow, _, had_errors) = targeted_encoding.decode(data);
    !had_errors && cow.len() > 0
}

pub fn build_table(valid_ranges: &[&[(u8, u8)]; 128]) -> [bool; 32768] {
    let mut table = [false; 32768];

    for lead in 0x80u8..=0xFF {
        let row = (lead - 0x80) as usize;
        let ranges = valid_ranges[row];
        for &(start, end) in ranges {
            for trail in start..=end {
                let idx = row * 256 + trail as usize;
                table[idx] = true;
            }
        }
    }
    table
}

pub fn build_i32_table_from_bool(table: &[bool; 32768]) -> [i32; 32768] {
    let mut i32_table = [0; 32768];
    for i in 0..32768 {
        i32_table[i] = if table[i] { 1 } else { 0 };
    }
    i32_table
}

pub static LEAD_BYTE_MASK: __m128i =
    unsafe { std::mem::transmute([0i8, 2, 4, 6, 8, 10, 12, 14, -1, -1, -1, -1, -1, -1, -1, -1]) };

pub static TRAIL_BYTE_MASK: __m128i =
    unsafe { std::mem::transmute([1i8, 3, 5, 7, 9, 11, 13, 15, -1, -1, -1, -1, -1, -1, -1, -1]) };

pub static ONE: __m256i = unsafe { std::mem::transmute([0x01i32; 8]) };

pub static EIGHTY: __m128i = unsafe { std::mem::transmute([0x80u8 as i8; 16]) };
