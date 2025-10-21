use libc::{c_char, c_void, size_t};
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