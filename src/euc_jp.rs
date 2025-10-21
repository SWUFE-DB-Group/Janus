use crate::tools;
use encoding_rs::EUC_JP;
use std::arch::x86_64::*;
pub fn is_jp_iconv(data: &[u8]) -> bool {
    tools::is_encoding_iconv(data, "EUC-JP")
}

pub fn is_jp_rs(data: &[u8]) -> bool {
    tools::is_encoding_rs(data, EUC_JP)
}

pub fn is_jp_range(data: &[u8]) -> bool {
    let mut i = 0;
    while i < data.len() {
        let lead = data[i];
        if lead <= 0x7F {
            // ASCII
            i += 1;
        } else if lead == 0x8E {
            // half-width katakana
            if i + 1 > data.len() {
                return false;
            }
            let trail = data[i + 1];
            if !(0xA1..=0xDF).contains(&trail) {
                return false;
            }
            i += 2;
        } else if lead == 0x8F {
            // JIS X 0212
            if i + 2 >= data.len() {
                return false;
            }
            let second = data[i + 1];
            let third = data[i + 2];

            if !(0xA1..=0xFE).contains(&second) {
                return false;
            }
            if !(0xA1..=0xFE).contains(&third) {
                return false;
            }

            if second == 0xA1
                || (0xA3..=0xA5).contains(&second)
                || second == 0xA8
                || (0xAC..=0xAF).contains(&second)
                || (0xEE..=0xFE).contains(&second)
            {
                return false;
            }

            match (second, third) {
                (0xA2, 0xA1..=0xAE)
                | (0xA2, 0xBA..=0xC1)
                | (0xA2, 0xC5..=0xEA)
                | (0xA2, 0xF2..=0xFE)
                | (0xA6, 0xA1..=0xE0)
                | (0xA6, 0xE6)
                | (0xA6, 0xE8)
                | (0xA6, 0xEB)
                | (0xA6, 0xED..=0xF0)
                | (0xA6, 0xFD..=0xFE)
                | (0xA7, 0xA1..=0xC1)
                | (0xA7, 0xCF..=0xF1)
                | (0xA9, 0xA3)
                | (0xA9, 0xA5)
                | (0xA9, 0xA7)
                | (0xA9, 0xAA)
                | (0xA9, 0xAE)
                | (0xA9, 0xB1..=0xC0)
                | (0xA9, 0xD1..=0xFE)
                | (0xAA, 0xB9)
                | (0xAA, 0xF8..=0xFE)
                | (0xAB, 0xBC)
                | (0xAB, 0xC4)
                | (0xAB, 0xF8..=0xFE)
                | (0xED, 0xE5..=0xFE) => {
                    return false;
                }
                _ => {
                    i += 3;
                }
            }
        } else if (0xA1..=0xFE).contains(&lead) {
            // JIS X 0208
            if i + 1 >= data.len() {
                return false;
            }
            let trail = data[i + 1];
            if !(0xA1..=0xFE).contains(&trail) {
                return false;
            }
            if (0xA9..=0xAF).contains(&lead) {
                return false;
            }
            if (0xF5..=0xFE).contains(&lead) {
                return false;
            }
            match (lead, trail) {
                (0xA2, 0xAF..=0xB9)
                | (0xA2, 0xC2..=0xC9)
                | (0xA2, 0xD1..=0xDB)
                | (0xA2, 0xEB..=0xF1)
                | (0xA2, 0xFA..=0xFD)
                | (0xA3, 0xA1..=0xAF)
                | (0xA3, 0xBA..=0xC0)
                | (0xA3, 0xDB..=0xE0)
                | (0xA3, 0xFB..=0xFE)
                | (0xA4, 0xF4..=0xFE)
                | (0xA5, 0xF7..=0xFE)
                | (0xA6, 0xB9..=0xC0)
                | (0xA6, 0xD9..=0xFE)
                | (0xA7, 0xC2..=0xD0)
                | (0xA7, 0xF2..=0xFE)
                | (0xA8, 0xC1..=0xFE)
                | (0xCF, 0xD4..=0xFE)
                | (0xF4, 0xA7..=0xFE) => {
                    return false;
                }
                _ => {
                    i += 2;
                }
            }
        } else {
            return false;
        }
    }
    true
}

static VALID_JP_RANGES: [&[(u8, u8)]; 128] = [
    // 0x80 (index 0)
    &[],
    // 0x81 (index 1)
    &[],
    // 0x82 (index 2)
    &[],
    // 0x83 (index 3)
    &[],
    // 0x84 (index 4)
    &[],
    // 0x85 (index 5)
    &[],
    // 0x86 (index 6)
    &[],
    // 0x87 (index 7)
    &[],
    // 0x88 (index 8)
    &[],
    // 0x89 (index 9)
    &[],
    // 0x8A (index 10)
    &[],
    // 0x8B (index 11)
    &[],
    // 0x8C (index 12)
    &[],
    // 0x8D (index 13)
    &[],
    // 0x8E (index 14)
    &[],
    // 0x8F (index 15)
    &[],
    // 0x90 (index 16)
    &[],
    // 0x91 (index 17)
    &[],
    // 0x92 (index 18)
    &[],
    // 0x93 (index 19)
    &[],
    // 0x94 (index 20)
    &[],
    // 0x95 (index 21)
    &[],
    // 0x96 (index 22)
    &[],
    // 0x97 (index 23)
    &[],
    // 0x98 (index 24)
    &[],
    // 0x99 (index 25)
    &[],
    // 0x9A (index 26)
    &[],
    // 0x9B (index 27)
    &[],
    // 0x9C (index 28)
    &[],
    // 0x9D (index 29)
    &[],
    // 0x9E (index 30)
    &[],
    // 0x9F (index 31)
    &[],
    // 0xA0 (index 32)
    &[],
    // 0xA1 (index 33)
    &[(0xA1, 0xFE)],
    // 0xA2 (index 34)
    &[
        (0xA1, 0xAE),
        (0xBA, 0xC1),
        (0xCA, 0xD0),
        (0xDC, 0xEA),
        (0xF2, 0xF9),
        (0xFE, 0xFE),
    ],
    // 0xA3 (index 35)
    &[(0xB0, 0xB9), (0xC1, 0xDA), (0xE1, 0xFA)],
    // 0xA4 (index 36)
    &[(0xA1, 0xF3)],
    // 0xA5 (index 37)
    &[(0xA1, 0xF6)],
    // 0xA6 (index 38)
    &[(0xA1, 0xB8), (0xC1, 0xD8)],
    // 0xA7 (index 39)
    &[(0xA1, 0xC1), (0xD1, 0xF1)],
    // 0xA8 (index 40)
    &[(0xA1, 0xC0)],
    // 0xA9 (index 41)
    &[],
    // 0xAA (index 42)
    &[],
    // 0xAB (index 43)
    &[],
    // 0xAC (index 44)
    &[],
    // 0xAD (index 45)
    &[],
    // 0xAE (index 46)
    &[],
    // 0xAF (index 47)
    &[],
    // 0xB0 (index 48)
    &[(0xA1, 0xFE)],
    // 0xB1 (index 49)
    &[(0xA1, 0xFE)],
    // 0xB2 (index 50)
    &[(0xA1, 0xFE)],
    // 0xB3 (index 51)
    &[(0xA1, 0xFE)],
    // 0xB4 (index 52)
    &[(0xA1, 0xFE)],
    // 0xB5 (index 53)
    &[(0xA1, 0xFE)],
    // 0xB6 (index 54)
    &[(0xA1, 0xFE)],
    // 0xB7 (index 55)
    &[(0xA1, 0xFE)],
    // 0xB8 (index 56)
    &[(0xA1, 0xFE)],
    // 0xB9 (index 57)
    &[(0xA1, 0xFE)],
    // 0xBA (index 58)
    &[(0xA1, 0xFE)],
    // 0xBB (index 59)
    &[(0xA1, 0xFE)],
    // 0xBC (index 60)
    &[(0xA1, 0xFE)],
    // 0xBD (index 61)
    &[(0xA1, 0xFE)],
    // 0xBE (index 62)
    &[(0xA1, 0xFE)],
    // 0xBF (index 63)
    &[(0xA1, 0xFE)],
    // 0xC0 (index 64)
    &[(0xA1, 0xFE)],
    // 0xC1 (index 65)
    &[(0xA1, 0xFE)],
    // 0xC2 (index 66)
    &[(0xA1, 0xFE)],
    // 0xC3 (index 67)
    &[(0xA1, 0xFE)],
    // 0xC4 (index 68)
    &[(0xA1, 0xFE)],
    // 0xC5 (index 69)
    &[(0xA1, 0xFE)],
    // 0xC6 (index 70)
    &[(0xA1, 0xFE)],
    // 0xC7 (index 71)
    &[(0xA1, 0xFE)],
    // 0xC8 (index 72)
    &[(0xA1, 0xFE)],
    // 0xC9 (index 73)
    &[(0xA1, 0xFE)],
    // 0xCA (index 74)
    &[(0xA1, 0xFE)],
    // 0xCB (index 75)
    &[(0xA1, 0xFE)],
    // 0xCC (index 76)
    &[(0xA1, 0xFE)],
    // 0xCD (index 77)
    &[(0xA1, 0xFE)],
    // 0xCE (index 78)
    &[(0xA1, 0xFE)],
    // 0xCF (index 79)
    &[(0xA1, 0xD3)],
    // 0xD0 (index 80)
    &[(0xA1, 0xFE)],
    // 0xD1 (index 81)
    &[(0xA1, 0xFE)],
    // 0xD2 (index 82)
    &[(0xA1, 0xFE)],
    // 0xD3 (index 83)
    &[(0xA1, 0xFE)],
    // 0xD4 (index 84)
    &[(0xA1, 0xFE)],
    // 0xD5 (index 85)
    &[(0xA1, 0xFE)],
    // 0xD6 (index 86)
    &[(0xA1, 0xFE)],
    // 0xD7 (index 87)
    &[(0xA1, 0xFE)],
    // 0xD8 (index 88)
    &[(0xA1, 0xFE)],
    // 0xD9 (index 89)
    &[(0xA1, 0xFE)],
    // 0xDA (index 90)
    &[(0xA1, 0xFE)],
    // 0xDB (index 91)
    &[(0xA1, 0xFE)],
    // 0xDC (index 92)
    &[(0xA1, 0xFE)],
    // 0xDD (index 93)
    &[(0xA1, 0xFE)],
    // 0xDE (index 94)
    &[(0xA1, 0xFE)],
    // 0xDF (index 95)
    &[(0xA1, 0xFE)],
    // 0xE0 (index 96)
    &[(0xA1, 0xFE)],
    // 0xE1 (index 97)
    &[(0xA1, 0xFE)],
    // 0xE2 (index 98)
    &[(0xA1, 0xFE)],
    // 0xE3 (index 99)
    &[(0xA1, 0xFE)],
    // 0xE4 (index 100)
    &[(0xA1, 0xFE)],
    // 0xE5 (index 101)
    &[(0xA1, 0xFE)],
    // 0xE6 (index 102)
    &[(0xA1, 0xFE)],
    // 0xE7 (index 103)
    &[(0xA1, 0xFE)],
    // 0xE8 (index 104)
    &[(0xA1, 0xFE)],
    // 0xE9 (index 105)
    &[(0xA1, 0xFE)],
    // 0xEA (index 106)
    &[(0xA1, 0xFE)],
    // 0xEB (index 107)
    &[(0xA1, 0xFE)],
    // 0xEC (index 108)
    &[(0xA1, 0xFE)],
    // 0xED (index 109)
    &[(0xA1, 0xFE)],
    // 0xEE (index 110)
    &[(0xA1, 0xFE)],
    // 0xEF (index 111)
    &[(0xA1, 0xFE)],
    // 0xF0 (index 112)
    &[(0xA1, 0xFE)],
    // 0xF1 (index 113)
    &[(0xA1, 0xFE)],
    // 0xF2 (index 114)
    &[(0xA1, 0xFE)],
    // 0xF3 (index 115)
    &[(0xA1, 0xFE)],
    // 0xF4 (index 116)
    &[(0xA1, 0xA6)],
    // 0xF5 (index 117)
    &[],
    // 0xF6 (index 118)
    &[],
    // 0xF7 (index 119)
    &[],
    // 0xF8 (index 120)
    &[],
    // 0xF9 (index 121)
    &[],
    // 0xFA (index 122)
    &[],
    // 0xFB (index 123)
    &[],
    // 0xFC (index 124)
    &[],
    // 0xFD (index 125)
    &[],
    // 0xFE (index 126)
    &[],
    // 0xFF (index 127)
    &[],
];

pub fn build_jp_table() -> [bool; 32768] {
    tools::build_table(&VALID_JP_RANGES)
}

pub fn is_jp_lookup(data: &[u8], table: &[bool; 32768]) -> bool {
    let mut i = 0;
    while i < data.len() {
        let lead = data[i];
        if lead <= 0x7F {
            // ASCII
            i += 1;
        } else if lead == 0x8F {
            // JIS X 0212
            if i + 2 >= data.len() {
                return false;
            }
            let second = data[i + 1];
            let third = data[i + 2];

            if !(0xA1..=0xFE).contains(&second) {
                return false;
            }
            if !(0xA1..=0xFE).contains(&third) {
                return false;
            }

            if second == 0xA1
                || (0xA3..=0xA5).contains(&second)
                || second == 0xA8
                || (0xAC..=0xAF).contains(&second)
                || (0xEE..=0xFE).contains(&second)
            {
                return false;
            }

            match (second, third) {
                (0xA2, 0xA1..=0xAE)
                | (0xA2, 0xBA..=0xC1)
                | (0xA2, 0xC5..=0xEA)
                | (0xA2, 0xF2..=0xFE)
                | (0xA6, 0xA1..=0xE0)
                | (0xA6, 0xE6)
                | (0xA6, 0xE8)
                | (0xA6, 0xEB)
                | (0xA6, 0xED..=0xF0)
                | (0xA6, 0xFD..=0xFE)
                | (0xA7, 0xA1..=0xC1)
                | (0xA7, 0xCF..=0xF1)
                | (0xA9, 0xA3)
                | (0xA9, 0xA5)
                | (0xA9, 0xA7)
                | (0xA9, 0xAA)
                | (0xA9, 0xAE)
                | (0xA9, 0xB1..=0xC0)
                | (0xA9, 0xD1..=0xFE)
                | (0xAA, 0xB9)
                | (0xAA, 0xF8..=0xFE)
                | (0xAB, 0xBC)
                | (0xAB, 0xC4)
                | (0xAB, 0xF8..=0xFE)
                | (0xED, 0xE5..=0xFE) => {
                    return false;
                }
                _ => {
                    i += 3;
                }
            }
        } else if (0xA1..=0xFE).contains(&lead) || lead == 0x8E {
            // JIS X 0208
            if i + 1 >= data.len() {
                return false;
            }
            let trail = data[i + 1];

            let index = (lead as usize - 0x80) * 256 + (trail as usize);
            if !table[index] {
                return false;
            }

            i += 2;
        }
    }
    true
}

fn jp_fallback(slice: &[u8], table: &[i32; 32768]) -> i32 {
    let mut i = 0;
    while i < slice.len() {
        let lead = slice[i];
        if lead <= 0x7F {
            i += 1;
        } else if lead == 0x8F {
            if i + 2 >= slice.len() {
                return (slice.len() - i) as i32;
            }
            let second = slice[i + 1];
            let third = slice[i + 2];

            if !(0xA1..=0xFE).contains(&second) {
                return -1;
            }
            if !(0xA1..=0xFE).contains(&third) {
                return -1;
            }

            if second == 0xA1
                || (0xA3..=0xA5).contains(&second)
                || second == 0xA8
                || (0xAC..=0xAF).contains(&second)
                || (0xEE..=0xFE).contains(&second)
            {
                return -1;
            }

            match (second, third) {
                (0xA2, 0xA1..=0xAE)
                | (0xA2, 0xBA..=0xC1)
                | (0xA2, 0xC5..=0xEA)
                | (0xA2, 0xF2..=0xFE)
                | (0xA6, 0xA1..=0xE0)
                | (0xA6, 0xE6)
                | (0xA6, 0xE8)
                | (0xA6, 0xEB)
                | (0xA6, 0xED..=0xF0)
                | (0xA6, 0xFD..=0xFE)
                | (0xA7, 0xA1..=0xC1)
                | (0xA7, 0xCF..=0xF1)
                | (0xA9, 0xA3)
                | (0xA9, 0xA5)
                | (0xA9, 0xA7)
                | (0xA9, 0xAA)
                | (0xA9, 0xAE)
                | (0xA9, 0xB1..=0xC0)
                | (0xA9, 0xD1..=0xFE)
                | (0xAA, 0xB9)
                | (0xAA, 0xF8..=0xFE)
                | (0xAB, 0xBC)
                | (0xAB, 0xC4)
                | (0xAB, 0xF8..=0xFE)
                | (0xED, 0xE5..=0xFE) => {
                    return -1;
                }
                _ => {
                    i += 3;
                }
            }
        } else if (0xA1..=0xFE).contains(&lead) || lead == 0x8E {
            if i + 1 >= slice.len() {
                return (slice.len() - i) as i32;
            }
            let trail = slice[i + 1];

            let index = (lead as usize - 0x80) * 256 + (trail as usize);
            if table[index] != 1 {
                return -1;
            }

            i += 2;
        } else {
            return -1;
        }
    }
    0
}

static EIGHT_F: __m128i = unsafe { std::mem::transmute([0x8Fu8 as i8; 16]) };

pub fn is_jp_simd(data: &[u8], bitmap: &[i32; 32768]) -> bool {
    let len = data.len();
    let mut i = 0;
    while i + 15 < len {
        let chunk = unsafe { _mm_loadu_si128(data[i..].as_ptr() as *const __m128i) };
        let mask = unsafe { _mm_movemask_epi8(chunk) };
        if mask == 0xFFFF {
            unsafe {
                let cmp_8f = _mm_cmpeq_epi8(chunk, EIGHT_F);
                let test_8f = _mm_movemask_epi8(cmp_8f);
                if test_8f == 0 {
                    let leads = _mm_shuffle_epi8(chunk, tools::LEAD_BYTE_MASK);
                    let leads = _mm_cvtepu8_epi16(leads);
                    let trails = _mm_shuffle_epi8(chunk, tools::TRAIL_BYTE_MASK);
                    let trails = _mm_cvtepu8_epi16(trails);
                    let sub_leads = _mm_sub_epi16(leads, tools::EIGHTY);
                    let shift_leads = _mm_slli_epi16::<8>(sub_leads);
                    let indices = _mm_or_si128(shift_leads, trails);
                    let indices = _mm256_cvtepu16_epi32(indices);
                    let gathers = _mm256_i32gather_epi32::<4>(bitmap.as_ptr(), indices);
                    let cmp = _mm256_cmpeq_epi32(gathers, tools::ONE);
                    let mask = _mm256_movemask_epi8(cmp);
                    if mask != -1 {
                        return false;
                    }
                    i += 16;
                } else {
                    let fallback_result = jp_fallback(&data[i..i + 16], bitmap);
                    if fallback_result == -1 {
                        return false;
                    } else {
                        i += 16 - fallback_result as usize;
                    }
                }
            }
        } else if mask == 0 {
            // all ASCII
            i += 16;
        } else {
            let fallback_result = jp_fallback(&data[i..i + 16], bitmap);
            if fallback_result == -1 {
                return false;
            } else {
                i += 16 - fallback_result as usize;
            }
        }
    }
    if i < len {
        let fallback_result = jp_fallback(&data[i..], bitmap);
        if fallback_result != 0 {
            return false;
        }
    }
    true
}
