use super::*;

use std::hash::{Hash, Hasher};

// Hash just the bytes of the slice, without length prefix
struct Bytes<'a>(&'a [u8]);

impl<'a> Hash for Bytes<'a> {
    #[allow(unused_must_use)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.0 {
            state.write_u8(*byte);
        }
    }
}

fn hash_with<T: Hash>(mut st: SipHasher128, x: &T) -> (u64, u64) {
    x.hash(&mut st);
    st.finish128()
}

fn hash<T: Hash>(x: &T) -> (u64, u64) {
    hash_with(SipHasher128::new_with_keys(0, 0), x)
}
#[rustfmt::skip]
const TEST_VECTOR: [[u8; 16]; 64] = [
    [0xe7, 0x7e, 0xbc, 0xb2, 0x27, 0x88, 0xa5, 0xbe, 0xfd, 0x62, 0xdb, 0x6a, 0xdd, 0x30, 0x30, 0x01],
    [0xfc, 0x6f, 0x37, 0x04, 0x60, 0xd3, 0xed, 0xa8, 0x5e, 0x05, 0x73, 0xcc, 0x2b, 0x2f, 0xf0, 0x63],
    [0x75, 0x78, 0x7f, 0x09, 0x05, 0x69, 0x83, 0x9b, 0x85, 0x5b, 0xc9, 0x54, 0x8c, 0x6a, 0xea, 0x95],
    [0x6b, 0xc5, 0xcc, 0xfa, 0x1e, 0xdc, 0xf7, 0x9f, 0x48, 0x23, 0x18, 0x77, 0x12, 0xeb, 0xd7, 0x43],
    [0x0c, 0x78, 0x4e, 0x71, 0xac, 0x2b, 0x28, 0x5a, 0x9f, 0x8e, 0x92, 0xe7, 0x8f, 0xbf, 0x2c, 0x25],
    [0xf3, 0x28, 0xdb, 0x89, 0x34, 0x5b, 0x62, 0x0c, 0x79, 0x52, 0x29, 0xa4, 0x26, 0x95, 0x84, 0x3e],
    [0xdc, 0xd0, 0x3d, 0x29, 0xf7, 0x43, 0xe7, 0x10, 0x09, 0x51, 0xb0, 0xe8, 0x39, 0x85, 0xa6, 0xf8],
    [0x10, 0x84, 0xb9, 0x23, 0xf2, 0xaa, 0xe0, 0xc3, 0xa6, 0x2f, 0x2e, 0xc8, 0x08, 0x48, 0xab, 0x77],
    [0xaa, 0x12, 0xfe, 0xe1, 0xd5, 0xe3, 0xda, 0xb4, 0x72, 0x4f, 0x16, 0xab, 0x35, 0xf9, 0xc7, 0x99],
    [0x81, 0xdd, 0xb8, 0x04, 0x2c, 0xf3, 0x39, 0x94, 0xf4, 0x72, 0x0e, 0x00, 0x94, 0x13, 0x7c, 0x42],
    [0x4f, 0xaa, 0x54, 0x1d, 0x5d, 0x49, 0x8e, 0x89, 0xba, 0x0e, 0xa4, 0xc3, 0x87, 0xb2, 0x2f, 0xb4],
    [0x72, 0x3b, 0x9a, 0xf3, 0x55, 0x44, 0x91, 0xdb, 0xb1, 0xd6, 0x63, 0x3d, 0xfc, 0x6e, 0x0c, 0x4e],
    [0xe5, 0x3f, 0x92, 0x85, 0x9e, 0x48, 0x19, 0xa8, 0xdc, 0x06, 0x95, 0x73, 0x9f, 0xea, 0x8c, 0x65],
    [0xb2, 0xf8, 0x58, 0xc7, 0xc9, 0xea, 0x80, 0x1d, 0x53, 0xd6, 0x03, 0x59, 0x6d, 0x65, 0x78, 0x44],
    [0x87, 0xe7, 0x62, 0x68, 0xdb, 0xc9, 0x22, 0x72, 0x26, 0xb0, 0xca, 0x66, 0x5f, 0x64, 0xe3, 0x78],
    [0xc1, 0x7e, 0x55, 0x05, 0xb2, 0xbd, 0x52, 0x6c, 0x29, 0x21, 0xcd, 0xec, 0x1e, 0x7e, 0x01, 0x09],
    [0xd0, 0xa8, 0xd9, 0x57, 0x15, 0x51, 0x8e, 0xeb, 0xb5, 0x13, 0xb0, 0xf8, 0x3d, 0x9e, 0x17, 0x93],
    [0x23, 0x41, 0x26, 0xf9, 0x3f, 0xbb, 0x66, 0x8d, 0x97, 0x51, 0x12, 0xe8, 0xfe, 0xbd, 0xf7, 0xec],
    [0xef, 0x42, 0xf0, 0x3d, 0xb7, 0x8f, 0x70, 0x4d, 0x02, 0x3c, 0x44, 0x9f, 0x16, 0xb7, 0x09, 0x2b],
    [0xab, 0xf7, 0x62, 0x38, 0xc2, 0x0a, 0xf1, 0x61, 0xb2, 0x31, 0x4b, 0x4d, 0x55, 0x26, 0xbc, 0xe9],
    [0x3c, 0x2c, 0x2f, 0x11, 0xbb, 0x90, 0xcf, 0x0b, 0xe3, 0x35, 0xca, 0x9b, 0x2e, 0x91, 0xe9, 0xb7],
    [0x2a, 0x7a, 0x68, 0x0f, 0x22, 0xa0, 0x2a, 0x92, 0xf4, 0x51, 0x49, 0xd2, 0x0f, 0xec, 0xe0, 0xef],
    [0xc9, 0xa8, 0xd1, 0x30, 0x23, 0x1d, 0xd4, 0x3e, 0x42, 0xe6, 0x45, 0x69, 0x57, 0xf8, 0x37, 0x79],
    [0x1d, 0x12, 0x7b, 0x84, 0x40, 0x5c, 0xea, 0xb9, 0x9f, 0xd8, 0x77, 0x5a, 0x9b, 0xe6, 0xc5, 0x59],
    [0x9e, 0x4b, 0xf8, 0x37, 0xbc, 0xfd, 0x92, 0xca, 0xce, 0x09, 0xd2, 0x06, 0x1a, 0x84, 0xd0, 0x4a],
    [0x39, 0x03, 0x1a, 0x96, 0x5d, 0x73, 0xb4, 0xaf, 0x5a, 0x27, 0x4d, 0x18, 0xf9, 0x73, 0xb1, 0xd2],
    [0x7f, 0x4d, 0x0a, 0x12, 0x09, 0xd6, 0x7e, 0x4e, 0xd0, 0x6f, 0x75, 0x38, 0xe1, 0xcf, 0xad, 0x64],
    [0xe6, 0x1e, 0xe2, 0x40, 0xfb, 0xdc, 0xce, 0x38, 0x96, 0x9f, 0x4c, 0xd2, 0x49, 0x27, 0xdd, 0x93],
    [0x4c, 0x3b, 0xa2, 0xb3, 0x7b, 0x0f, 0xdd, 0x8c, 0xfa, 0x5e, 0x95, 0xc1, 0x89, 0xb2, 0x94, 0x14],
    [0xe0, 0x6f, 0xd4, 0xca, 0x06, 0x6f, 0xec, 0xdd, 0x54, 0x06, 0x8a, 0x5a, 0xd8, 0x89, 0x6f, 0x86],
    [0x5c, 0xa8, 0x4c, 0x34, 0x13, 0x9c, 0x65, 0x80, 0xa8, 0x8a, 0xf2, 0x49, 0x90, 0x72, 0x07, 0x06],
    [0x42, 0xea, 0x96, 0x1c, 0x5b, 0x3c, 0x85, 0x8b, 0x17, 0xc3, 0xe5, 0x50, 0xdf, 0xa7, 0x90, 0x10],
    [0x40, 0x6c, 0x44, 0xde, 0xe6, 0x78, 0x57, 0xb2, 0x94, 0x31, 0x60, 0xf3, 0x0c, 0x74, 0x17, 0xd3],
    [0xc5, 0xf5, 0x7b, 0xae, 0x13, 0x20, 0xfc, 0xf4, 0xb4, 0xe8, 0x68, 0xe7, 0x1d, 0x56, 0xc6, 0x6b],
    [0x04, 0xbf, 0x73, 0x7a, 0x5b, 0x67, 0x6b, 0xe7, 0xc3, 0xde, 0x05, 0x01, 0x7d, 0xf4, 0xbf, 0xf9],
    [0x51, 0x63, 0xc9, 0xc0, 0x3f, 0x19, 0x07, 0xea, 0x10, 0x44, 0xed, 0x5c, 0x30, 0x72, 0x7b, 0x4f],
    [0x37, 0xa1, 0x10, 0xf0, 0x02, 0x71, 0x8e, 0xda, 0xd2, 0x4b, 0x3f, 0x9e, 0xe4, 0x53, 0xf1, 0x40],
    [0xb9, 0x87, 0x7e, 0x38, 0x1a, 0xed, 0xd3, 0xda, 0x08, 0xc3, 0x3e, 0x75, 0xff, 0x23, 0xac, 0x10],
    [0x7c, 0x50, 0x04, 0x00, 0x5e, 0xc5, 0xda, 0x4c, 0x5a, 0xc9, 0x44, 0x0e, 0x5c, 0x72, 0x31, 0x93],
    [0x81, 0xb8, 0x24, 0x37, 0x83, 0xdb, 0xc6, 0x46, 0xca, 0x9d, 0x0c, 0xd8, 0x2a, 0xbd, 0xb4, 0x6c],
    [0x50, 0x57, 0x20, 0x54, 0x3e, 0xb9, 0xb4, 0x13, 0xd5, 0x0b, 0x3c, 0xfa, 0xd9, 0xee, 0xf9, 0x38],
    [0x94, 0x5f, 0x59, 0x4d, 0xe7, 0x24, 0x11, 0xe4, 0xd3, 0x35, 0xbe, 0x87, 0x44, 0x56, 0xd8, 0xf3],
    [0x37, 0x92, 0x3b, 0x3e, 0x37, 0x17, 0x77, 0xb2, 0x11, 0x70, 0xbf, 0x9d, 0x7e, 0x62, 0xf6, 0x02],
    [0x3a, 0xd4, 0xe7, 0xc8, 0x57, 0x64, 0x96, 0x46, 0x11, 0xeb, 0x0a, 0x6c, 0x4d, 0x62, 0xde, 0x56],
    [0xcd, 0x91, 0x39, 0x6c, 0x44, 0xaf, 0x4f, 0x51, 0x85, 0x57, 0x8d, 0x9d, 0xd9, 0x80, 0x3f, 0x0a],
    [0xfe, 0x28, 0x15, 0x8e, 0x72, 0x7b, 0x86, 0x8f, 0x39, 0x03, 0xc9, 0xac, 0xda, 0x64, 0xa2, 0x58],
    [0x40, 0xcc, 0x10, 0xb8, 0x28, 0x8c, 0xe5, 0xf0, 0xbc, 0x3a, 0xc0, 0xb6, 0x8a, 0x0e, 0xeb, 0xc8],
    [0x6f, 0x14, 0x90, 0xf5, 0x40, 0x69, 0x9a, 0x3c, 0xd4, 0x97, 0x44, 0x20, 0xec, 0xc9, 0x27, 0x37],
    [0xd5, 0x05, 0xf1, 0xb7, 0x5e, 0x1a, 0x84, 0xa6, 0x03, 0xc4, 0x35, 0x83, 0xb2, 0xed, 0x03, 0x08],
    [0x49, 0x15, 0x73, 0xcf, 0xd7, 0x2b, 0xb4, 0x68, 0x2b, 0x7c, 0xa5, 0x88, 0x0e, 0x1c, 0x8d, 0x6f],
    [0x3e, 0xd6, 0x9c, 0xfe, 0x45, 0xab, 0x40, 0x3f, 0x2f, 0xd2, 0xad, 0x95, 0x9b, 0xa2, 0x76, 0x66],
    [0x8b, 0xe8, 0x39, 0xef, 0x1b, 0x20, 0xb5, 0x7c, 0x83, 0xba, 0x7e, 0xb6, 0xa8, 0xc2, 0x2b, 0x6a],
    [0x14, 0x09, 0x18, 0x6a, 0xb4, 0x22, 0x31, 0xfe, 0xde, 0xe1, 0x81, 0x62, 0xcf, 0x1c, 0xb4, 0xca],
    [0x2b, 0xf3, 0xcc, 0xc2, 0x4a, 0xb6, 0x72, 0xcf, 0x15, 0x1f, 0xb8, 0xd2, 0xf3, 0xf3, 0x06, 0x9b],
    [0xb9, 0xb9, 0x3a, 0x28, 0x82, 0xd6, 0x02, 0x5c, 0xdb, 0x8c, 0x56, 0xfa, 0x13, 0xf7, 0x53, 0x7b],
    [0xd9, 0x7c, 0xca, 0x36, 0x94, 0xfb, 0x20, 0x6d, 0xb8, 0xbd, 0x1f, 0x36, 0x50, 0xc3, 0x33, 0x22],
    [0x94, 0xec, 0x2e, 0x19, 0xa4, 0x0b, 0xe4, 0x1a, 0xf3, 0x94, 0x0d, 0x6b, 0x30, 0xc4, 0x93, 0x84],
    [0x4b, 0x41, 0x60, 0x3f, 0x20, 0x9a, 0x04, 0x5b, 0xe1, 0x40, 0xa3, 0x41, 0xa3, 0xdf, 0xfe, 0x10],
    [0x23, 0xfb, 0xcb, 0x30, 0x9f, 0x1c, 0xf0, 0x94, 0x89, 0x07, 0x55, 0xab, 0x1b, 0x42, 0x65, 0x69],
    [0xe7, 0xd9, 0xb6, 0x56, 0x90, 0x91, 0x8a, 0x2b, 0x23, 0x2f, 0x2f, 0x5c, 0x12, 0xc8, 0x30, 0x0e],
    [0xad, 0xe8, 0x3c, 0xf7, 0xe7, 0xf3, 0x84, 0x7b, 0x36, 0xfa, 0x4b, 0x54, 0xb0, 0x0d, 0xce, 0x61],
    [0x06, 0x10, 0xc5, 0xf2, 0xee, 0x57, 0x1c, 0x8a, 0xc8, 0x0c, 0xbf, 0xe5, 0x38, 0xbd, 0xf1, 0xc7],
    [0x27, 0x1d, 0x5d, 0x00, 0xfb, 0xdb, 0x5d, 0x15, 0x5d, 0x9d, 0xce, 0xa9, 0x7c, 0xb4, 0x02, 0x18],
    [0x4c, 0x58, 0x00, 0xe3, 0x4e, 0xfe, 0x42, 0x6f, 0x07, 0x9f, 0x6b, 0x0a, 0xa7, 0x52, 0x60, 0xad],
];

#[test]
fn test_siphash_1_3_test_vector() {
    let k0 = 0x_07_06_05_04_03_02_01_00;
    let k1 = 0x_0f_0e_0d_0c_0b_0a_09_08;

    let mut input: Vec<u8> = Vec::new();

    for i in 0..64 {
        let out = hash_with(SipHasher128::new_with_keys(k0, k1), &Bytes(&input[..]));
        let expected = (
            ((TEST_VECTOR[i][0] as u64) << 0)
                | ((TEST_VECTOR[i][1] as u64) << 8)
                | ((TEST_VECTOR[i][2] as u64) << 16)
                | ((TEST_VECTOR[i][3] as u64) << 24)
                | ((TEST_VECTOR[i][4] as u64) << 32)
                | ((TEST_VECTOR[i][5] as u64) << 40)
                | ((TEST_VECTOR[i][6] as u64) << 48)
                | ((TEST_VECTOR[i][7] as u64) << 56),
            ((TEST_VECTOR[i][8] as u64) << 0)
                | ((TEST_VECTOR[i][9] as u64) << 8)
                | ((TEST_VECTOR[i][10] as u64) << 16)
                | ((TEST_VECTOR[i][11] as u64) << 24)
                | ((TEST_VECTOR[i][12] as u64) << 32)
                | ((TEST_VECTOR[i][13] as u64) << 40)
                | ((TEST_VECTOR[i][14] as u64) << 48)
                | ((TEST_VECTOR[i][15] as u64) << 56),
        );

        assert_eq!(out, expected);
        input.push(i as u8);
    }
}

#[test]
#[cfg(target_arch = "arm")]
fn test_hash_usize() {
    let val = 0xdeadbeef_deadbeef_u64;
    assert!(hash(&(val as u64)) != hash(&(val as usize)));
    assert_eq!(hash(&(val as u32)), hash(&(val as usize)));
}
#[test]
#[cfg(target_arch = "x86_64")]
fn test_hash_usize() {
    let val = 0xdeadbeef_deadbeef_u64;
    assert_eq!(hash(&(val as u64)), hash(&(val as usize)));
    assert!(hash(&(val as u32)) != hash(&(val as usize)));
}
#[test]
#[cfg(target_arch = "x86")]
fn test_hash_usize() {
    let val = 0xdeadbeef_deadbeef_u64;
    assert!(hash(&(val as u64)) != hash(&(val as usize)));
    assert_eq!(hash(&(val as u32)), hash(&(val as usize)));
}

#[test]
fn test_hash_idempotent() {
    let val64 = 0xdeadbeef_deadbeef_u64;
    assert_eq!(hash(&val64), hash(&val64));
    let val32 = 0xdeadbeef_u32;
    assert_eq!(hash(&val32), hash(&val32));
}

#[test]
fn test_hash_no_bytes_dropped_64() {
    let val = 0xdeadbeef_deadbeef_u64;

    assert!(hash(&val) != hash(&zero_byte(val, 0)));
    assert!(hash(&val) != hash(&zero_byte(val, 1)));
    assert!(hash(&val) != hash(&zero_byte(val, 2)));
    assert!(hash(&val) != hash(&zero_byte(val, 3)));
    assert!(hash(&val) != hash(&zero_byte(val, 4)));
    assert!(hash(&val) != hash(&zero_byte(val, 5)));
    assert!(hash(&val) != hash(&zero_byte(val, 6)));
    assert!(hash(&val) != hash(&zero_byte(val, 7)));

    fn zero_byte(val: u64, byte: usize) -> u64 {
        assert!(byte < 8);
        val & !(0xff << (byte * 8))
    }
}

#[test]
fn test_hash_no_bytes_dropped_32() {
    let val = 0xdeadbeef_u32;

    assert!(hash(&val) != hash(&zero_byte(val, 0)));
    assert!(hash(&val) != hash(&zero_byte(val, 1)));
    assert!(hash(&val) != hash(&zero_byte(val, 2)));
    assert!(hash(&val) != hash(&zero_byte(val, 3)));

    fn zero_byte(val: u32, byte: usize) -> u32 {
        assert!(byte < 4);
        val & !(0xff << (byte * 8))
    }
}

#[test]
fn test_hash_no_concat_alias() {
    let s = ("aa", "bb");
    let t = ("aabb", "");
    let u = ("a", "abb");

    assert!(s != t && t != u);
    assert!(hash(&s) != hash(&t) && hash(&s) != hash(&u));

    let u = [1, 0, 0, 0];
    let v = (&u[..1], &u[1..3], &u[3..]);
    let w = (&u[..], &u[4..4], &u[4..4]);

    assert!(v != w);
    assert!(hash(&v) != hash(&w));
}

#[test]
fn test_short_write_works() {
    let test_u8 = 0xFF_u8;
    let test_u16 = 0x1122_u16;
    let test_u32 = 0x22334455_u32;
    let test_u64 = 0x33445566_778899AA_u64;
    let test_u128 = 0x11223344_55667788_99AABBCC_DDEEFF77_u128;
    let test_usize = 0xD0C0B0A0_usize;

    let test_i8 = -1_i8;
    let test_i16 = -2_i16;
    let test_i32 = -3_i32;
    let test_i64 = -4_i64;
    let test_i128 = -5_i128;
    let test_isize = -6_isize;

    let mut h1 = SipHasher128::new_with_keys(0, 0);
    h1.write(b"bytes");
    h1.write(b"string");
    h1.write_u8(test_u8);
    h1.write_u16(test_u16);
    h1.write_u32(test_u32);
    h1.write_u64(test_u64);
    h1.write_u128(test_u128);
    h1.write_usize(test_usize);
    h1.write_i8(test_i8);
    h1.write_i16(test_i16);
    h1.write_i32(test_i32);
    h1.write_i64(test_i64);
    h1.write_i128(test_i128);
    h1.write_isize(test_isize);

    let mut h2 = SipHasher128::new_with_keys(0, 0);
    h2.write(b"bytes");
    h2.write(b"string");
    h2.write(&test_u8.to_ne_bytes());
    h2.write(&test_u16.to_ne_bytes());
    h2.write(&test_u32.to_ne_bytes());
    h2.write(&test_u64.to_ne_bytes());
    h2.write(&test_u128.to_ne_bytes());
    h2.write(&test_usize.to_ne_bytes());
    h2.write(&test_i8.to_ne_bytes());
    h2.write(&test_i16.to_ne_bytes());
    h2.write(&test_i32.to_ne_bytes());
    h2.write(&test_i64.to_ne_bytes());
    h2.write(&test_i128.to_ne_bytes());
    h2.write(&test_isize.to_ne_bytes());

    let h1_hash = h1.finish128();
    let h2_hash = h2.finish128();

    assert_eq!(h1_hash, h2_hash);
}

macro_rules! test_fill_buffer {
    ($type:ty, $write_method:ident) => {{
        // Test filling and overfilling the buffer from all possible offsets
        // for a given integer type and its corresponding write method.
        const SIZE: usize = std::mem::size_of::<$type>();
        let input = [42; BUFFER_SIZE];
        let x = 0x01234567_89ABCDEF_76543210_FEDCBA98_u128 as $type;
        let x_bytes = &x.to_ne_bytes();

        for i in 1..=SIZE {
            let s = &input[..BUFFER_SIZE - i];

            let mut h1 = SipHasher128::new_with_keys(7, 13);
            h1.write(s);
            h1.$write_method(x);

            let mut h2 = SipHasher128::new_with_keys(7, 13);
            h2.write(s);
            h2.write(x_bytes);

            let h1_hash = h1.finish128();
            let h2_hash = h2.finish128();

            assert_eq!(h1_hash, h2_hash);
        }
    }};
}

#[test]
fn test_fill_buffer() {
    test_fill_buffer!(u8, write_u8);
    test_fill_buffer!(u16, write_u16);
    test_fill_buffer!(u32, write_u32);
    test_fill_buffer!(u64, write_u64);
    test_fill_buffer!(u128, write_u128);
    test_fill_buffer!(usize, write_usize);

    test_fill_buffer!(i8, write_i8);
    test_fill_buffer!(i16, write_i16);
    test_fill_buffer!(i32, write_i32);
    test_fill_buffer!(i64, write_i64);
    test_fill_buffer!(i128, write_i128);
    test_fill_buffer!(isize, write_isize);
}
