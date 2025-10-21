use crate::tools;
use encoding_rs::GB18030;
use std::arch::x86_64::*;
pub fn is_gb18030_iconv(data: &[u8]) -> bool {
    tools::is_encoding_iconv(data, "GB18030")
}

pub fn is_gb18030_rs(data: &[u8]) -> bool {
    tools::is_encoding_rs(data, GB18030)
}

#[derive(PartialEq)]
enum State {
    Start,
    // Holds the first byte of a potential multi-byte sequence
    ExpectingSecondByte(u8),
    // Holds the first two bytes of a potential 4-byte sequence
    ExpectingThirdByte(u8, u8),
    // Holds the first three bytes of a potential 4-byte sequence
    ExpectingFourthByte(u8, u8, u8),
}

pub fn is_gb18030_fsm(data: &[u8]) -> bool {
    let mut state = State::Start;
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        match state {
            State::Start => {
                match byte {
                    // 1-byte sequence (ASCII)
                    0x00..=0x7F => {
                        i += 1;
                    }
                    // Potential start of a 2-byte or 4-byte sequence
                    0x81..=0xFE => {
                        state = State::ExpectingSecondByte(byte);
                        i += 1;
                    }
                    // Any other byte is invalid as a starting byte
                    _ => return false,
                }
            }
            State::ExpectingSecondByte(b1) => {
                match byte {
                    // Completes a 2-byte sequence
                    0x40..=0x7E | 0x80..=0xFE => {
                        state = State::Start;
                        i += 1;
                    }
                    // Continues a 4-byte sequence
                    0x30..=0x39 => {
                        state = State::ExpectingThirdByte(b1, byte);
                        i += 1;
                    }
                    // Invalid second byte
                    _ => return false,
                }
            }
            State::ExpectingThirdByte(b1, b2) => {
                match byte {
                    // Continues a 4-byte sequence
                    0x81..=0xFE => {
                        state = State::ExpectingFourthByte(b1, b2, byte);
                        i += 1;
                    }
                    // Invalid third byte
                    _ => return false,
                }
            }
            State::ExpectingFourthByte(b1, b2, b3) => {
                match byte {
                    // Completes a 4-byte sequence, now needs validation
                    0x30..=0x39 => {
                        let b4 = byte;
                        let val = u32::from_be_bytes([b1, b2, b3, b4]);

                        // Check against the defined upper bounds for 4-byte sequences.
                        // The valid 4-byte ranges are 0x81308130-0x8431A439 and 0x90308130-0xE3329A35.
                        if (val > 0x8431A439 && val < 0x90308130) || val > 0xE3329A35 {
                            return false;
                        }

                        state = State::Start;
                        i += 1;
                    }
                    // Invalid fourth byte
                    _ => return false,
                }
            }
        }
    }

    // The final state must be Start, otherwise the sequence is truncated.
    state == State::Start
}

#[inline]
fn mm_compge_epu8(a: __m128i, b: __m128i) -> __m128i {
    unsafe { _mm_cmpeq_epi8(_mm_max_epu8(a, b), a) }
}

#[inline]
fn mm_comple_epu8(a: __m128i, b: __m128i) -> __m128i {
    unsafe { mm_compge_epu8(b, a) }
}

pub static FORTY: __m128i = unsafe { std::mem::transmute([0x40u8 as i8; 16]) };
pub static SEVEN_F: __m128i = unsafe { std::mem::transmute([0x7Fu8 as i8; 16]) };
pub static EIGHT_ONE: __m128i = unsafe { std::mem::transmute([0x81u8 as i8; 16]) };
pub static FE: __m128i = unsafe { std::mem::transmute([0xFEu8 as i8; 16]) };

#[inline]
unsafe fn validate_byte_pairs_128(data: __m128i) -> bool {
    let lead_in_range = _mm_and_si128(mm_compge_epu8(data, EIGHT_ONE), mm_comple_epu8(data, FE));

    let tail_in_range = _mm_and_si128(mm_compge_epu8(data, FORTY), mm_comple_epu8(data, FE));
    let seven_in_range = _mm_cmpeq_epi8(data, SEVEN_F);
    let seven_mask = _mm_and_si128(seven_in_range, tools::TRAIL_BYTE_MASK);

    _mm_testc_si128(lead_in_range, tools::LEAD_BYTE_MASK) != 0
        && _mm_testc_si128(tail_in_range, tools::TRAIL_BYTE_MASK) != 0
        && _mm_movemask_epi8(seven_mask) == 0
}

fn gb18030_fallback(data: &[u8], current: usize) -> i32 {
    let mut state = State::Start;
    let mut j = current;
    let mut i = current;
    while j < data.len() {
        let byte = data[j];
        match state {
            State::Start => {
                match byte {
                    // 1-byte sequence (ASCII)
                    0x00..=0x7F => {
                        i += 1;
                        return i as i32;
                    }
                    // Potential start of a 2-byte or 4-byte sequence
                    0x81..=0xFE => {
                        state = State::ExpectingSecondByte(byte);
                        j += 1;
                    }
                    // Any other byte is invalid as a starting byte
                    _ => return -1,
                }
            }
            State::ExpectingSecondByte(b1) => {
                match byte {
                    // Completes a 2-byte sequence
                    0x40..=0x7E | 0x80..=0xFE => {
                        i += 2;
                        return i as i32;
                    }
                    // Continues a 4-byte sequence
                    0x30..=0x39 => {
                        state = State::ExpectingThirdByte(b1, byte);
                        j += 1;
                    }
                    // Invalid second byte
                    _ => return -1,
                }
            }
            State::ExpectingThirdByte(b1, b2) => {
                match byte {
                    // Continues a 4-byte sequence
                    0x81..=0xFE => {
                        state = State::ExpectingFourthByte(b1, b2, byte);
                        j += 1;
                    }
                    // Invalid third byte
                    _ => return -1,
                }
            }
            State::ExpectingFourthByte(b1, b2, b3) => {
                match byte {
                    // Completes a 4-byte sequence, now needs validation
                    0x30..=0x39 => {
                        let b4 = byte;
                        let val = u32::from_be_bytes([b1, b2, b3, b4]);

                        // Check against the defined upper bounds for 4-byte sequences.
                        // The valid 4-byte ranges are 0x81308130-0x8431A439 and 0x90308130-0xE3329A35.
                        if (val > 0x8431A439 && val < 0x90308130) || val > 0xE3329A35 {
                            return -1;
                        }
                        i += 4;
                        return i as i32;
                    }
                    // Invalid fourth byte
                    _ => return -1,
                }
            }
        }
    }
    i as i32
}

pub fn is_gb18030_simd(data: &[u8]) -> bool {
    let len = data.len();
    let mut i = 0;
    while i + 15 < len {
        let chunk = unsafe { _mm_loadu_si128(data[i..].as_ptr() as *const __m128i) };
        let mask = unsafe { _mm_movemask_epi8(chunk) };
        if (mask == 0) || (mask == 0xFFFF && unsafe { validate_byte_pairs_128(chunk) }) {
            i += 16;
            continue;
        }
        let fallback_result = gb18030_fallback(data, i);
        if fallback_result == -1 {
            return false;
        } else {
            i = fallback_result as usize;
        }
    }
    if i < len {
        let slice = &data[i..];
        return is_gb18030_fsm(slice);
    }
    true
}
