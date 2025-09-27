// src/lib.rs
// SHA-256 from-scratch (educational)
// -----------------------------------------------------------------------------
// ⚠️ WARNING
// This code is for learning. It is NOT constant-time and NOT hardened against
// side-channel attacks. Do not use it in production. Prefer crates:
//   sha2 = "*"   (RustCrypto Digest)
// -----------------------------------------------------------------------------

// ========== Bitwise helpers ==========
#[inline] fn rotr(x: u32, n: u32) -> u32 { (x >> n) | (x << (32 - n)) }
#[inline] fn ch(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (!x & z) }
#[inline] fn maj(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (x & z) ^ (y & z) }
#[inline] fn bsig0(x: u32) -> u32 { rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22) }
#[inline] fn bsig1(x: u32) -> u32 { rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25) }
#[inline] fn ssig0(x: u32) -> u32 { rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3) }
#[inline] fn ssig1(x: u32) -> u32 { rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10) }

// ========== IV and K constants (FIPS 180-4) ==========
const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
];

// ========== Padding ==========
fn pad_message(msg: &[u8]) -> Vec<u8> {
    // Append '1' bit (0x80), then k zero bytes so that len ≡ 56 (mod 64),
    // then 64-bit big-endian original length in bits.
    let bit_len = (msg.len() as u64) * 8;
    let mut v = Vec::with_capacity(((msg.len() + 9 + 63) / 64) * 64);
    v.extend_from_slice(msg);
    v.push(0x80);
    while (v.len() % 64) != 56 { v.push(0x00); }
    v.extend_from_slice(&bit_len.to_be_bytes());
    v
}

// ========== Compression on one 512-bit block ==========
fn compress_block(state: &mut [u32; 8], block: &[u8; 64]) {
    // 1) Message schedule
    let mut w = [0u32; 64];
    for t in 0..16 {
        let i = 4 * t;
        w[t] = u32::from_be_bytes([block[i], block[i+1], block[i+2], block[i+3]]);
    }
    for t in 16..64 {
        w[t] = ssig1(w[t-2])
            .wrapping_add(w[t-7])
            .wrapping_add(ssig0(w[t-15]))
            .wrapping_add(w[t-16]);
    }

    // 2) Initialize working vars
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    // 3) 64 rounds
    for t in 0..64 {
        let t1 = h
            .wrapping_add(bsig1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[t])
            .wrapping_add(w[t]);
        let t2 = bsig0(a).wrapping_add(maj(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    // 4) Add back to state
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

// ========== One-shot API ==========
pub fn sha256(msg: &[u8]) -> [u8; 32] {
    let mut state = H0;
    let padded = pad_message(msg);
    for chunk in padded.chunks_exact(64) {
        let mut block = [0u8; 64];
        block.copy_from_slice(chunk);
        compress_block(&mut state, &block);
    }
    let mut out = [0u8; 32];
    for (i, v) in state.iter().enumerate() {
        out[4*i..4*i+4].copy_from_slice(&v.to_be_bytes());
    }
    out
}

// ========== Streaming API ==========
pub struct Sha256 {
    state: [u32; 8],
    buf: [u8; 64],
    buf_len: usize,
    len_bytes: u64,
}

impl Sha256 {
    pub fn new() -> Self {
        Self { state: H0, buf: [0;64], buf_len: 0, len_bytes: 0 }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        self.len_bytes = self.len_bytes.wrapping_add(data.len() as u64);

        // If buffer has leftover, fill first
        if self.buf_len > 0 {
            let to_take = (64 - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len+to_take].copy_from_slice(&data[..to_take]);
            self.buf_len += to_take;
            data = &data[to_take..];
            if self.buf_len == 64 {
                compress_block(&mut self.state, &self.buf);
                self.buf_len = 0;
                self.buf_len = 0;
            }
        }

        // Full blocks
        for chunk in data.chunks_exact(64) {
            compress_block(&mut self.state, &self.buf);
            self.buf_len = 0;
        }

        // Remainder
        let rem = data.len() % 64;
        if rem > 0 {
            self.buf[..rem].copy_from_slice(&data[data.len()-rem..]);
            self.buf_len = rem;
        }
    }

    pub fn finalize(mut self) -> [u8; 32] {
        // Manual padding using internal buffer
        let bit_len = self.len_bytes.wrapping_mul(8);

        // Append 0x80
        self.buf[self.buf_len] = 0x80;
        self.buf_len += 1;

        if self.buf_len > 56 {
            // zero until 64, compress, then clear to 56
            for b in &mut self.buf[self.buf_len..64] { *b = 0; }
            compress_block(&mut self.state, &self.buf);
            self.buf_len = 0;
            self.buf = [0u8; 64];
            self.buf_len = 0;
        }

        // zero-pad until 56, then length
        for b in &mut self.buf[self.buf_len..56] { *b = 0; }
        self.buf[56..64].copy_from_slice(&bit_len.to_be_bytes());
        compress_block(&mut self.state, &self.buf);
        self.buf_len = 0;

        let mut out = [0u8; 32];
        for (i, v) in self.state.iter().enumerate() {
            out[4*i..4*i+4].copy_from_slice(&v.to_be_bytes());
        }
        out
    }
}

// ========== Utilities ==========
pub fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

// ========== Demo / Analysis / Validation ==========
pub fn example() {
    let msg = b"hello world";
    let digest = sha256(msg);

    println!("🔑 SHA-256 Example");
    println!("{}", "=".repeat(60));
    println!("Input      : \"{}\"", String::from_utf8_lossy(msg));
    println!("SHA-256(hex): {}", to_hex(&digest));

    analyze_hash_output(msg);
    println!();
    comprehensive_validation();

    // (Optional) RustCrypto cross-check (enable feature "rustcrypto")
    #[cfg(feature = "rustcrypto")]
    {
        cross_validation_test();
    }
}

pub fn analyze_hash_output(msg: &[u8]) {
    println!("🔬 Hash Output Analysis");
    println!("{}", "=".repeat(60));
    println!("Input bytes: {} ({} bytes)", to_hex(msg), msg.len());

    let padded = pad_message(msg);
    println!("Padded len : {} bytes ({} blocks)", padded.len(), padded.len()/64);
    if !padded.is_empty() {
        let first = &padded[..64.min(padded.len())];
        println!("First block: {}", to_hex(first));
        if padded.len() > 64 {
            let last = &padded[padded.len()-64..];
            println!("Last  block: {}", to_hex(last));
        }
    }
}

pub fn comprehensive_validation() {
    println!("🧪 Comprehensive Validation Suite");
    println!("{}", "=".repeat(60));

    let vectors = [
        // FIPS 180-4 test vectors
        ("", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        ("abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        ("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
         "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"),
        ("The quick brown fox jumps over the lazy dog",
         "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"),
        ("The quick brown fox jumps over the lazy dog.",
         "ef537f25c895bfa782526529a9b63d97aa631564d5d789c2b765448c8635fb6c"),
    ];

    let mut all_ok = true;
    for (i, (input, expect_hex)) in vectors.iter().enumerate() {
        let out = sha256(input.as_bytes());
        let out_hex = to_hex(&out);
        let ok = out_hex == *expect_hex;
        println!("Test {}: {} => {}", i+1, if ok {"✅ PASS"} else {"❌ FAIL"}, input);
        if !ok {
            println!("  expected: {}", expect_hex);
            println!("  got     : {}", out_hex);
        }
        all_ok &= ok;
    }

    // Streaming API cross-check on a multi-chunk update
    let chunks = ["The quick ", "brown fox jumps ", "over the lazy dog"];
    let mut st = Sha256::new();
    for c in &chunks { st.update(c.as_bytes()); }
    let stream_hex = to_hex(&st.finalize());
    let one_shot_hex = to_hex(&sha256(b"The quick brown fox jumps over the lazy dog"));
    let stream_ok = stream_hex == one_shot_hex;
    println!("Streaming vs One-shot: {}", if stream_ok { "✅ MATCH" } else { "❌ MISMATCH" });
    if !stream_ok {
        println!("  streaming: {}", stream_hex);
        println!("  one-shot : {}", one_shot_hex);
    }

    if all_ok && stream_ok {
        println!("🎉 All vectors passed!");
    }
}

pub fn cross_validation_test() {
    use sha2::{Digest, Sha256 as RC_Sha256};

    println!("🔍 Cross Validation: Our Implementation vs RustCrypto");
    println!("{}", "=".repeat(60));

    let cases = [
        b"hello world".as_slice(),
        b"",
        b"a",
        b"exactly 16 bytes!".as_slice(), // 17 bytes just to differ from block size here
        b"This is a long message spanning multiple blocks".as_slice(),
        "🦀 Rust SHA-256! 🔐".as_bytes(),
    ];

    let mut all_ok = true;

    for (i, msg) in cases.iter().enumerate() {
        let ours = sha256(msg);
        let mut hasher = RC_Sha256::new();
        hasher.update(msg);
        let rc = hasher.finalize();

        let eq = ours.as_slice() == rc.as_slice();
        println!(
            "Case {:02}: {} (len {})",
            i + 1,
            if eq { "✅ MATCH" } else { "❌ MISMATCH" },
            msg.len()
        );
        if !eq {
            println!("  ours: {}", to_hex(&ours));
            println!("  rc  : {}", to_hex(&rc));
            all_ok = false;
        }
    }

    if all_ok {
        println!("🎉 Success! Our SHA-256 matches RustCrypto for all cases.");
    } else {
        println!("❌ Some cases differ. Debugging required.");
    }
}
