use crate::tools;
use encoding_rs::EUC_KR;
use std::arch::x86_64::*;
pub fn is_kr_iconv(data: &[u8]) -> bool {
    tools::is_encoding_iconv(data, "EUC-KR")
}

pub fn is_kr_rs(data: &[u8]) -> bool {
    tools::is_encoding_rs(data, EUC_KR)
}

pub fn is_kr_range(data: &[u8]) -> bool {
    let mut i = 0;
    while i < data.len() {
        let lead = data[i];
        match lead {
            0x00..=0x7F => {
                i += 1;
            }
            0xA1..=0xFE => {
                if i + 1 >= data.len() {
                    return false;
                }
                let trail = data[i + 1];
                if !(0xA1..=0xFE).contains(&trail) {
                    return false;
                }
                if (0xAD..=0xAF).contains(&lead) || lead == 0xC9 || lead == 0xFE {
                    return false;
                }
                match (lead, trail) {
                    (0xA2, 0xE8..=0xFE)
                    | (0xA4, 0xD4)
                    | (0xA5, 0xAB..=0xAF)
                    | (0xA5, 0xBA..=0xC0)
                    | (0xA5, 0xD9..=0xE0)
                    | (0xA5, 0xF9..=0xFE)
                    | (0xA6, 0xE5..=0xFE)
                    | (0xA7, 0xF0..=0xFE)
                    | (0xA8, 0xA5)
                    | (0xA8, 0xA7)
                    | (0xA8, 0xB0)
                    | (0xAA, 0xF4..=0xFE)
                    | (0xAB, 0xF7..=0xFE)
                    | (0xAC, 0xC2..=0xD0)
                    | (0xAC, 0xF2..=0xFE) => {
                        return false;
                    }
                    _ => {
                        i += 2;
                    }
                }
            }
            _ => return false,
        }
    }
    true
}

static VALID_KR_RANGES: [&[(u8, u8)]; 128] = [
    // 0x80-0xA0, not any valid bytes
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    // 0xA1
    &[(0xA1, 0xFE)],
    // 0xA2
    &[(0xA1, 0xE7)],
    // 0xA3
    &[(0xA1, 0xFE)],
    // 0xA4
    &[(0xA1, 0xD3), (0xD5, 0xFE)],
    // 0xA5
    &[(0xA1, 0xAA), (0xB1, 0xB9), (0xC1, 0xD8), (0xE1, 0xF8)],
    // 0xA6
    &[(0xA1, 0xE4)],
    // 0xA7
    &[(0xA1, 0xEF)],
    // 0xA8
    &[(0xA1, 0xA4), (0xA6, 0xA6), (0xA8, 0xAF), (0xB1, 0xFE)],
    // 0xA9
    &[(0xA1, 0xFE)],
    // 0xAA
    &[(0xA1, 0xF3)],
    // 0xAB
    &[(0xA1, 0xF6)],
    // 0xAC
    &[(0xA1, 0xC1), (0xD1, 0xF1)],
    // 0xAD
    &[],
    // 0xAE
    &[],
    // 0xAF
    &[],
    // 0xB0
    &[(0xA1, 0xFE)],
    // 0xB1
    &[(0xA1, 0xFE)],
    // 0xB2
    &[(0xA1, 0xFE)],
    // 0xB3
    &[(0xA1, 0xFE)],
    // 0xB4
    &[(0xA1, 0xFE)],
    // 0xB5
    &[(0xA1, 0xFE)],
    // 0xB6
    &[(0xA1, 0xFE)],
    // 0xB7
    &[(0xA1, 0xFE)],
    // 0xB8
    &[(0xA1, 0xFE)],
    // 0xB9
    &[(0xA1, 0xFE)],
    // 0xBA
    &[(0xA1, 0xFE)],
    // 0xBB
    &[(0xA1, 0xFE)],
    // 0xBC
    &[(0xA1, 0xFE)],
    // 0xBD
    &[(0xA1, 0xFE)],
    // 0xBE
    &[(0xA1, 0xFE)],
    // 0xBF
    &[(0xA1, 0xFE)],
    // 0xC0
    &[(0xA1, 0xFE)],
    // 0xC1
    &[(0xA1, 0xFE)],
    // 0xC2
    &[(0xA1, 0xFE)],
    // 0xC3
    &[(0xA1, 0xFE)],
    // 0xC4
    &[(0xA1, 0xFE)],
    // 0xC5
    &[(0xA1, 0xFE)],
    // 0xC6
    &[(0xA1, 0xFE)],
    // 0xC7
    &[(0xA1, 0xFE)],
    // 0xC8
    &[(0xA1, 0xFE)],
    // 0xC9
    &[],
    // 0xCA
    &[(0xA1, 0xFE)],
    // 0xCB
    &[(0xA1, 0xFE)],
    // 0xCC
    &[(0xA1, 0xFE)],
    // 0xCD
    &[(0xA1, 0xFE)],
    // 0xCE
    &[(0xA1, 0xFE)],
    // 0xCF
    &[(0xA1, 0xFE)],
    // 0xD0
    &[(0xA1, 0xFE)],
    // 0xD1
    &[(0xA1, 0xFE)],
    // 0xD2
    &[(0xA1, 0xFE)],
    // 0xD3
    &[(0xA1, 0xFE)],
    // 0xD4
    &[(0xA1, 0xFE)],
    // 0xD5
    &[(0xA1, 0xFE)],
    // 0xD6
    &[(0xA1, 0xFE)],
    // 0xD7
    &[(0xA1, 0xFE)],
    // 0xD8
    &[(0xA1, 0xFE)],
    // 0xD9
    &[(0xA1, 0xFE)],
    // 0xDA
    &[(0xA1, 0xFE)],
    // 0xDB
    &[(0xA1, 0xFE)],
    // 0xDC
    &[(0xA1, 0xFE)],
    // 0xDD
    &[(0xA1, 0xFE)],
    // 0xDE
    &[(0xA1, 0xFE)],
    // 0xDF
    &[(0xA1, 0xFE)],
    // 0xE0
    &[(0xA1, 0xFE)],
    // 0xE1
    &[(0xA1, 0xFE)],
    // 0xE2
    &[(0xA1, 0xFE)],
    // 0xE3
    &[(0xA1, 0xFE)],
    // 0xE4
    &[(0xA1, 0xFE)],
    // 0xE5
    &[(0xA1, 0xFE)],
    // 0xE6
    &[(0xA1, 0xFE)],
    // 0xE7
    &[(0xA1, 0xFE)],
    // 0xE8
    &[(0xA1, 0xFE)],
    // 0xE9
    &[(0xA1, 0xFE)],
    // 0xEA
    &[(0xA1, 0xFE)],
    // 0xEB
    &[(0xA1, 0xFE)],
    // 0xEC
    &[(0xA1, 0xFE)],
    // 0xED
    &[(0xA1, 0xFE)],
    // 0xEE
    &[(0xA1, 0xFE)],
    // 0xEF
    &[(0xA1, 0xFE)],
    // 0xF0
    &[(0xA1, 0xFE)],
    // 0xF1
    &[(0xA1, 0xFE)],
    // 0xF2
    &[(0xA1, 0xFE)],
    // 0xF3
    &[(0xA1, 0xFE)],
    // 0xF4
    &[(0xA1, 0xFE)],
    // 0xF5
    &[(0xA1, 0xFE)],
    // 0xF6
    &[(0xA1, 0xFE)],
    // 0xF7
    &[(0xA1, 0xFE)],
    // 0xF8
    &[(0xA1, 0xFE)],
    // 0xF9
    &[(0xA1, 0xFE)],
    // 0xFA
    &[(0xA1, 0xFE)],
    // 0xFB
    &[(0xA1, 0xFE)],
    // 0xFC
    &[(0xA1, 0xFE)],
    // 0xFD
    &[(0xA1, 0xFE)],
    // 0xFE
    &[],
    // 0xFF
    &[],
];

pub fn build_kr_table() -> [bool; 32768] {
    tools::build_table(&VALID_KR_RANGES)
}

pub fn is_kr_lookup(data: &[u8], table: &[bool; 32768]) -> bool {
    let mut i = 0;
    while i < data.len() {
        let lead = data[i];
        match lead {
            0x00..=0x7F => {
                i += 1;
            }
            0xA1..=0xFE => {
                if i + 1 >= data.len() {
                    return false;
                }
                let trail = data[i + 1];
                let index = (lead - 0x80) as usize * 256 + trail as usize;
                if !table[index] {
                    return false;
                }
                i += 2;
            }
            _ => return false,
        }
    }
    true
}

#[inline]
fn kr_fallback(slice: &[u8], table: &[i32; 32768]) -> i32 {
    let mut i = 0;
    while i < slice.len() {
        let lead = slice[i];
        match lead {
            0x00..=0x7F => {
                i += 1;
            }
            0xA1..=0xFE => {
                if i + 1 >= slice.len() {
                    return 1;
                }
                let trail = slice[i + 1];
                if !(0xA1..=0xFE).contains(&trail) {
                    return -1;
                }
                let index = (lead - 0x80) as usize * 256 + trail as usize;
                if table[index] != 1 {
                    return -1;
                }
                i += 2;
            }
            _ => return -1,
        }
    }
    0
}

pub fn is_kr_simd(data: &[u8], table: &[i32; 32768]) -> bool {
    let len = data.len();
    let mut i = 0;

    unsafe {
        while i + 15 < len {
            let chunk = _mm_loadu_si128(data[i..].as_ptr() as *const __m128i);
            let mask = _mm_movemask_epi8(chunk);
            if mask == 0xFFFF {
                // all two-byte characters
                let leads = _mm_shuffle_epi8(chunk, tools::LEAD_BYTE_MASK);
                let leads = _mm_cvtepu8_epi16(leads);
                let trails = _mm_shuffle_epi8(chunk, tools::TRAIL_BYTE_MASK);
                let trails = _mm_cvtepu8_epi16(trails);
                let sub_leads = _mm_sub_epi16(leads, tools::EIGHTY);
                let shift_leads = _mm_slli_epi16::<8>(sub_leads);
                let indices = _mm_or_si128(shift_leads, trails);
                let indices = _mm256_cvtepu16_epi32(indices);
                let gathers = _mm256_i32gather_epi32::<4>(table.as_ptr(), indices);
                let cmp = _mm256_cmpeq_epi32(gathers, tools::ONE);
                let mask = _mm256_movemask_epi8(cmp);
                if mask != -1 {
                    return false;
                }
                i += 16;
            } else if mask == 0 {
                // all single-byte characters
                i += 16;
            } else {
                // mixed single-byte and two-byte characters
                let fallback_result = kr_fallback(&data[i..i + 16], &table);
                if fallback_result == -1 {
                    return false;
                } else {
                    i += 16 - fallback_result as usize;
                }
            }
        }
    }
    if i < len {
        let fallback_result = kr_fallback(&data[i..], &table);
        if fallback_result == -1 {
            return false;
        }
    }
    true
}
