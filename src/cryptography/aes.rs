// AES-128 CBC from-scratch (educational)
// -----------------------------------------------------------------------------
// ⚠️ WARNING
// This code is for learning. It is NOT constant‑time and is NOT hardened
// against side‑channel attacks. Do not use it in production. Prefer crates:
//   aes = "*", cbc = "*", pkcs7 = "*" (or RustCrypto block-modes).
// -----------------------------------------------------------------------------

use cbc::Encryptor;
use std::fmt::Write as _;
const NB: usize = 4; // columns in state (4 words = 16 bytes)
const NK: usize = 4; // key length in words (AES-128)
const NR: usize = 10; // rounds (AES-128)
use cbc::cipher::BlockDecryptMut;
use cbc::cipher::BlockEncryptMut;
use cbc::cipher::KeyIvInit;
use cbc::cipher::block_padding::Pkcs7;
use aes::Aes128;
use cbc::Decryptor;
// S-Box and inverse S-Box
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const RCON: [u8; 11] = [
    0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36,
];

// GF(2^8) helpers
#[inline]
fn xtime(x: u8) -> u8 {
    ((x << 1) ^ (((x >> 7) & 1) * 0x1b))
}
#[inline]
fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    p
}

// State is 4x4 bytes (column-major)
#[derive(Clone, Copy)]
struct State {
    s: [[u8; 4]; 4],
}

impl State {
    fn from_block(block: [u8; 16]) -> Self {
        let mut s = [[0u8; 4]; 4];
        for c in 0..4 {
            for r in 0..4 {
                s[c][r] = block[c * 4 + r];
            }
        }
        Self { s }
    }
    fn to_block(&self) -> [u8; 16] {
        let mut b = [0u8; 16];
        for c in 0..4 {
            for r in 0..4 {
                b[c * 4 + r] = self.s[c][r];
            }
        }
        b
    }
}

fn sub_bytes(st: &mut State) {
    for c in 0..4 {
        for r in 0..4 {
            st.s[c][r] = SBOX[st.s[c][r] as usize];
        }
    }
}
fn inv_sub_bytes(st: &mut State) {
    for c in 0..4 {
        for r in 0..4 {
            st.s[c][r] = INV_SBOX[st.s[c][r] as usize];
        }
    }
}

fn shift_rows(st: &mut State) {
    // row 0 unchanged
    // row 1 left shift 1
    st.s[0][1] = st.s[0][1];
    let t1 = [st.s[0][1], st.s[1][1], st.s[2][1], st.s[3][1]];
    let t1s = [t1[1], t1[2], t1[3], t1[0]];
    for c in 0..4 {
        st.s[c][1] = t1s[c];
    }

    // row 2 left shift 2
    let t2 = [st.s[0][2], st.s[1][2], st.s[2][2], st.s[3][2]];
    let t2s = [t2[2], t2[3], t2[0], t2[1]];
    for c in 0..4 {
        st.s[c][2] = t2s[c];
    }

    // row 3 left shift 3
    let t3 = [st.s[0][3], st.s[1][3], st.s[2][3], st.s[3][3]];
    let t3s = [t3[3], t3[0], t3[1], t3[2]];
    for c in 0..4 {
        st.s[c][3] = t3s[c];
    }
}

fn inv_shift_rows(st: &mut State) {
    // inverse: right shifts
    let t1 = [st.s[0][1], st.s[1][1], st.s[2][1], st.s[3][1]];
    let t1s = [t1[3], t1[0], t1[1], t1[2]];
    for c in 0..4 {
        st.s[c][1] = t1s[c];
    }

    let t2 = [st.s[0][2], st.s[1][2], st.s[2][2], st.s[3][2]];
    let t2s = [t2[2], t2[3], t2[0], t2[1]]; // same as left2
    for c in 0..4 {
        st.s[c][2] = t2s[c];
    }

    let t3 = [st.s[0][3], st.s[1][3], st.s[2][3], st.s[3][3]];
    let t3s = [t3[1], t3[2], t3[3], t3[0]];
    for c in 0..4 {
        st.s[c][3] = t3s[c];
    }
}

fn mix_columns(st: &mut State) {
    for c in 0..4 {
        let a0 = st.s[c][0];
        let a1 = st.s[c][1];
        let a2 = st.s[c][2];
        let a3 = st.s[c][3];
        st.s[c][0] = gmul(0x02, a0) ^ gmul(0x03, a1) ^ a2 ^ a3;
        st.s[c][1] = a0 ^ gmul(0x02, a1) ^ gmul(0x03, a2) ^ a3;
        st.s[c][2] = a0 ^ a1 ^ gmul(0x02, a2) ^ gmul(0x03, a3);
        st.s[c][3] = gmul(0x03, a0) ^ a1 ^ a2 ^ gmul(0x02, a3);
    }
}

fn inv_mix_columns(st: &mut State) {
    for c in 0..4 {
        let a0 = st.s[c][0];
        let a1 = st.s[c][1];
        let a2 = st.s[c][2];
        let a3 = st.s[c][3];
        st.s[c][0] = gmul(0x0e, a0) ^ gmul(0x0b, a1) ^ gmul(0x0d, a2) ^ gmul(0x09, a3);
        st.s[c][1] = gmul(0x09, a0) ^ gmul(0x0e, a1) ^ gmul(0x0b, a2) ^ gmul(0x0d, a3);
        st.s[c][2] = gmul(0x0d, a0) ^ gmul(0x09, a1) ^ gmul(0x0e, a2) ^ gmul(0x0b, a3);
        st.s[c][3] = gmul(0x0b, a0) ^ gmul(0x0d, a1) ^ gmul(0x09, a2) ^ gmul(0x0e, a3);
    }
}

fn add_round_key(st: &mut State, round_key: &[u8; 16]) {
    for c in 0..4 {
        for r in 0..4 {
            st.s[c][r] ^= round_key[c * 4 + r];
        }
    }
}

fn rot_word(w: [u8; 4]) -> [u8; 4] {
    [w[1], w[2], w[3], w[0]]
}
fn sub_word(mut w: [u8; 4]) -> [u8; 4] {
    for i in 0..4 {
        w[i] = SBOX[w[i] as usize];
    }
    w
}

fn key_expansion(key: [u8; 16]) -> [[u8; 16]; NR + 1] {
    // w holds 44 32-bit words => 176 bytes => 11 round keys
    let mut w = [[0u8; 4]; NB * (NR + 1)];
    for i in 0..NK {
        w[i][0] = key[4 * i];
        w[i][1] = key[4 * i + 1];
        w[i][2] = key[4 * i + 2];
        w[i][3] = key[4 * i + 3];
    }
    for i in NK..NB * (NR + 1) {
        let mut temp = w[i - 1];
        if i % NK == 0 {
            temp = sub_word(rot_word(temp));
            temp[0] ^= RCON[i / NK];
        }
        for j in 0..4 {
            w[i][j] = w[i - NK][j] ^ temp[j];
        }
    }
    let mut rks = [[0u8; 16]; NR + 1];
    for r in 0..(NR + 1) {
        for c in 0..4 {
            for j in 0..4 {
                rks[r][4 * c + j] = w[4 * r + c][j];
            }
        }
    }
    rks
}

fn aes_encrypt_block(block: [u8; 16], round_keys: &[[u8; 16]; NR + 1]) -> [u8; 16] {
    let mut st = State::from_block(block);
    add_round_key(&mut st, &round_keys[0]);
    for round in 1..NR {
        sub_bytes(&mut st);
        shift_rows(&mut st);
        mix_columns(&mut st);
        add_round_key(&mut st, &round_keys[round]);
    }
    sub_bytes(&mut st);
    shift_rows(&mut st);
    add_round_key(&mut st, &round_keys[NR]);
    st.to_block()
}

fn aes_decrypt_block(block: [u8; 16], round_keys: &[[u8; 16]; NR + 1]) -> [u8; 16] {
    let mut st = State::from_block(block);
    add_round_key(&mut st, &round_keys[NR]);
    for round in (1..NR).rev() {
        inv_shift_rows(&mut st);
        inv_sub_bytes(&mut st);
        add_round_key(&mut st, &round_keys[round]);
        inv_mix_columns(&mut st);
    }
    inv_shift_rows(&mut st);
    inv_sub_bytes(&mut st);
    add_round_key(&mut st, &round_keys[0]);
    st.to_block()
}

// PKCS#7 padding
fn pkcs7_pad(mut data: Vec<u8>) -> Vec<u8> {
    let bsize = 16usize;
    let padlen = bsize - (data.len() % bsize);
    data.extend(std::iter::repeat(padlen as u8).take(padlen));
    data
}
fn pkcs7_unpad(mut data: Vec<u8>) -> Option<Vec<u8>> {
    if data.is_empty() {
        return None;
    }
    let pad = *data.last().unwrap() as usize;
    if pad == 0 || pad > 16 || pad > data.len() {
        return None;
    }
    if !data[data.len() - pad..].iter().all(|&b| b as usize == pad) {
        return None;
    }
    data.truncate(data.len() - pad);
    Some(data)
}

// CBC mode
fn xor16(a: &mut [u8; 16], b: &[u8; 16]) {
    for i in 0..16 {
        a[i] ^= b[i];
    }
}

fn cbc_encrypt(key: [u8; 16], iv: [u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let rks = key_expansion(key);
    let mut ct = Vec::with_capacity(((plaintext.len() / 16) + 2) * 16);
    let mut prev = iv;
    let mut data = pkcs7_pad(plaintext.to_vec());
    for chunk in data.chunks_exact_mut(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        xor16(&mut block, &prev);
        let enc = aes_encrypt_block(block, &rks);
        ct.extend_from_slice(&enc);
        prev = enc;
    }
    ct
}

fn cbc_decrypt(key: [u8; 16], iv: [u8; 16], ciphertext: &[u8]) -> Option<Vec<u8>> {
    if ciphertext.len() % 16 != 0 {
        return None;
    }
    let rks = key_expansion(key);
    let mut pt = Vec::with_capacity(ciphertext.len());
    let mut prev = iv;
    for chunk in ciphertext.chunks_exact(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        let dec = aes_decrypt_block(block, &rks);
        let mut x = dec;
        xor16(&mut x, &prev);
        pt.extend_from_slice(&x);
        prev = block;
    }
    pkcs7_unpad(pt)
}

// Utilities
fn to_hex(data: &[u8]) -> String {
    let mut s = String::with_capacity(data.len() * 2);
    for b in data {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}



fn rustcrypto_encrypt(key: [u8; 16], iv: [u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let mut buffer = plaintext.to_vec();
    let pos = buffer.len();
    buffer.resize(pos + 16, 0u8);

    let cipher = Encryptor::<Aes128>::new(&key.into(), &iv.into());
    let ct_len = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, pos)
        .unwrap()
        .len();
    buffer.truncate(ct_len);
    buffer
}

fn rustcrypto_decrypt(key: [u8; 16], iv: [u8; 16], ciphertext: &[u8]) -> Option<Vec<u8>> {
    let mut buffer = ciphertext.to_vec();
    let cipher = Decryptor::<Aes128>::new(&key.into(), &iv.into());
    match cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer) {
        Ok(pt) => Some(pt.to_vec()),
        Err(_) => None,
    }
}

fn analyze_encryption_output() {
    let key: [u8; 16] = *b"0123456789abcdef";
    let iv: [u8; 16] = *b"abcdef0123456789";
    let plaintext = b"hello world"; // 11 bytes

    println!("🔬 Encryption Output Analysis");
    println!("{}", "=".repeat(60));

    println!("Input Analysis:");
    println!(
        "  Plaintext: \"{}\" ({} bytes)",
        String::from_utf8_lossy(plaintext),
        plaintext.len()
    );
    println!("  Key: {} (16 bytes)", hex::encode(&key));
    println!("  IV: {} (16 bytes)", hex::encode(&iv));
    println!();

    // Padding info
    let padded = pkcs7_pad(plaintext.to_vec());
    println!("PKCS#7 Padding:");
    println!("  Original:  {} bytes", plaintext.len());
    println!("  After pad: {} bytes", padded.len());
    println!(
        "  Padding:   {} bytes, value 0x{:02x}",
        16 - plaintext.len(),
        16 - plaintext.len()
    );
    println!("  Padded hex: {}", hex::encode(&padded));
    println!();

    // Ciphertext info
    let ciphertext = cbc_encrypt(key, iv, plaintext);
    println!("Ciphertext Result:");
    println!(
        "  Ciphertext: {} ({} bytes)",
        hex::encode(&ciphertext),
        ciphertext.len()
    );
    println!("  Blocks:    {} (16 bytes each)", ciphertext.len() / 16);
    println!();

    for (i, block) in ciphertext.chunks(16).enumerate() {
        println!("  Block {}: {}", i, hex::encode(block));
    }
}

fn comprehensive_validation() {
    println!("🧪 Comprehensive Validation Suite");
    println!("{}", "=".repeat(60));

    let test_cases = vec![
        ("hello world", "basic message"),
        ("", "empty message"),
        ("a", "single character"),
        ("exactly 16 byte", "exactly one block"),
        (
            "This is a long message spanning multiple blocks",
            "multi-block message",
        ),
        ("🦀 Rust AES! 🔐", "unicode characters"),
    ];

    let key: [u8; 16] = *b"0123456789abcdef";
    let iv: [u8; 16] = *b"abcdef0123456789";

    let mut all_passed = true;

    for (i, (plaintext, description)) in test_cases.iter().enumerate() {
        println!("Test {}: {}", i + 1, description);
        println!("  Input: \"{}\"", plaintext);

        let plaintext_bytes = plaintext.as_bytes();
        let our_ct = cbc_encrypt(key, iv, plaintext_bytes);
        let rustcrypto_ct = rustcrypto_encrypt(key, iv, plaintext_bytes);

        let matches = our_ct == rustcrypto_ct;
        println!("  Result: {}", if matches { "✅ PASS" } else { "❌ FAIL" });

        if matches {
            // Round-trip decryption test
            let our_pt = cbc_decrypt(key, iv, &our_ct).unwrap();
            let round_trip_ok = our_pt == plaintext_bytes;
            println!(
                "  Round-trip: {}",
                if round_trip_ok {
                    "✅ PASS"
                } else {
                    "❌ FAIL"
                }
            );
            all_passed &= round_trip_ok;
        } else {
            println!("    Our hex:       {}", hex::encode(&our_ct));
            println!("    RustCrypto hex: {}", hex::encode(&rustcrypto_ct));
            all_passed = false;
        }
        println!();
    }

    if all_passed {
        println!(
            "🎉 All tests passed! Our implementation is mathematically equivalent to RustCrypto."
        );
    } else {
        println!("❌ Some tests failed. Debugging is required.");
    }
}

fn cross_validation_test() {
    println!("🔍 Cross Validation: Our Implementation vs RustCrypto");
    println!("{}", "=".repeat(60));

    let key: [u8; 16] = *b"0123456789abcdef";
    let iv: [u8; 16] = *b"abcdef0123456789";
    let plaintext = b"hello world";

    println!("Test Parameters:");
    println!("  Key (hex): {}", hex::encode(&key));
    println!("  IV (hex): {}", hex::encode(&iv));
    println!("  Plaintext: \"{}\"", String::from_utf8_lossy(plaintext));
    println!();

    // Our implementation
    println!("=== Our Implementation ===");
    let our_ciphertext = cbc_encrypt(key, iv, plaintext);
    println!("Our ciphertext (hex): {}", hex::encode(&our_ciphertext));

    let our_decrypted = cbc_decrypt(key, iv, &our_ciphertext).expect("Decryption failed");
    println!(
        "Our decrypted: \"{}\"",
        String::from_utf8_lossy(&our_decrypted)
    );
    println!();

    // RustCrypto implementation
    println!("=== RustCrypto Implementation ===");
    let rustcrypto_ciphertext = rustcrypto_encrypt(key, iv, plaintext);
    println!(
        "RustCrypto ciphertext (hex): {}",
        hex::encode(&rustcrypto_ciphertext)
    );

    let rustcrypto_decrypted =
        rustcrypto_decrypt(key, iv, &rustcrypto_ciphertext).expect("Decryption failed");
    println!(
        "RustCrypto decrypted: \"{}\"",
        String::from_utf8_lossy(&rustcrypto_decrypted)
    );
    println!();

    // Verification
    println!("=== Verification Results ===");
    let ciphertext_match = our_ciphertext == rustcrypto_ciphertext;
    let plaintext_match = our_decrypted == rustcrypto_decrypted;
    let original_match = plaintext == our_decrypted.as_slice();

    println!(
        "✅ Ciphertext match: {}",
        if ciphertext_match { "PASS" } else { "FAIL" }
    );
    println!(
        "✅ Plaintext match:  {}",
        if plaintext_match { "PASS" } else { "FAIL" }
    );
    println!(
        "✅ Round-trip:       {}",
        if original_match { "PASS" } else { "FAIL" }
    );

    if ciphertext_match && plaintext_match && original_match {
        println!();
        println!("🎉 Success! Our implementation produces the same result as RustCrypto.");
        println!("This proves the mathematical correctness of our AES-128-CBC implementation.");
    } else {
        println!();
        println!("❌ Mismatch detected! Our implementation has an issue.");
        println!("Let's debug the differences...");
    }
}

pub fn example() {
    // Fixed key/IV (for learning/demo)
    let key: [u8; 16] = *b"0123456789abcdef"; // 128-bit key
    let iv: [u8; 16] = *b"abcdef0123456789"; // 128-bit IV
    let msg = b"hello world";

    println!("🔑 AES-128 CBC Example");
    println!("{}", "=".repeat(60));

    // 1️⃣ Our implementation test
    let ct = cbc_encrypt(key, iv, msg);
    let pt = cbc_decrypt(key, iv, &ct).expect("valid padding");

    println!("[Our Implementation]");
    println!("  Plaintext : {}", String::from_utf8_lossy(msg));
    println!("  Ciphertext (hex): {}", to_hex(&ct));
    println!("  Decrypted : {}", String::from_utf8_lossy(&pt));
    println!();

    // 2️⃣ RustCrypto cross validation
    println!("[RustCrypto Cross Validation]");
    let rc_ct = rustcrypto_encrypt(key, iv, msg);
    let rc_pt = rustcrypto_decrypt(key, iv, &rc_ct).expect("decrypt failed");
    println!("  RustCrypto ciphertext (hex): {}", hex::encode(&rc_ct));
    println!(
        "  RustCrypto decrypted : {}",
        String::from_utf8_lossy(&rc_pt)
    );
    println!(
        "  Ciphertext match? {}",
        if ct == rc_ct { "✅" } else { "❌" }
    );
    println!(
        "  Plaintext match?  {}",
        if pt == rc_pt { "✅" } else { "❌" }
    );
    println!();

    // 3️⃣ Detailed analysis
    println!("[Output Analysis]");
    analyze_encryption_output();

    // 4️⃣ Run comprehensive validation suite
    println!();
    comprehensive_validation();
}
