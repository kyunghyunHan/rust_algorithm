/// Educational MD5 implementation in Rust (RFC 1321)
/// NOT constant-time; DO NOT use in production.

#[inline]
fn rotl(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

// MD5 basic functions
#[inline]
fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}
#[inline]
fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}
#[inline]
fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}
#[inline]
fn i_func(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
} 

/// S: per-round left-rotation amounts
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

/// K: floor(2^32 * abs(sin(i+1))) for i=0..63 (RFC 1321)
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

#[derive(Clone)]
pub struct Md5 {
    // state (A, B, C, D)
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    // number of bytes processed (for bit length)
    len_bytes: u64,
    // pending unprocessed bytes (<64)
    buffer: Vec<u8>,
}

impl Md5 {
    pub fn new() -> Self {
        Self {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
            len_bytes: 0,
            buffer: Vec::with_capacity(64),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.len_bytes = self.len_bytes.wrapping_add(data.len() as u64);
        let mut input = data;

        // 버퍼에 남은 데이터 먼저 채우기
        if !self.buffer.is_empty() {
            let needed = 64 - self.buffer.len();
            let take = needed.min(input.len());
            self.buffer.extend_from_slice(&input[..take]);
            input = &input[take..];

            if self.buffer.len() == 64 {
                let mut block = [0u8; 64];
                block.copy_from_slice(&self.buffer);
                self.process_block(&block); 
                self.buffer.clear();
            }
        }

        while input.len() >= 64 {
            let mut block = [0u8; 64];
            block.copy_from_slice(&input[..64]);
            self.process_block(&block); 
            input = &input[64..];
        }

        if !input.is_empty() {
            self.buffer.extend_from_slice(input);
        }
    }

    pub fn finalize(mut self) -> [u8; 16] {
        let bit_len = self.len_bytes.wrapping_mul(8);

        // append 0x80
        self.buffer.push(0x80);
        // pad with zeros until len % 64 == 56
        while (self.buffer.len() % 64) != 56 {
            self.buffer.push(0x00);
        }

        // append original length in bits (little-endian)
        self.buffer.extend_from_slice(&bit_len.to_le_bytes());

        // process final blocks by copying each 64-byte chunk into a local array
        let mut i = 0;
        while i < self.buffer.len() {
            let mut block = [0u8; 64];
            block.copy_from_slice(&self.buffer[i..i + 64]);
            self.process_block(&block); // 이제 self.buffer를 불변으로 빌리지 않음
            i += 64;
        }

        // output A||B||C||D (little-endian each)
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&self.a.to_le_bytes());
        out[4..8].copy_from_slice(&self.b.to_le_bytes());
        out[8..12].copy_from_slice(&self.c.to_le_bytes());
        out[12..16].copy_from_slice(&self.d.to_le_bytes());
        out
    }

    fn process_block(&mut self, block64: &[u8; 64]) {
        let mut m = [0u32; 16];
        for (i, chunk) in block64.chunks(4).enumerate() {
            m[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        let (mut a, mut b, mut c, mut d) = (self.a, self.b, self.c, self.d);

        for round in 0..64 {
            let (mut fval, gidx) = if round < 16 {
                (f(b, c, d), round as usize)
            } else if round < 32 {
                (g(b, c, d), ((5 * round + 1) % 16) as usize)
            } else if round < 48 {
                (h(b, c, d), ((3 * round + 5) % 16) as usize)
            } else {
                (i_func(b, c, d), ((7 * round) % 16) as usize)
            };

            fval = fval
                .wrapping_add(a)
                .wrapping_add(K[round])
                .wrapping_add(m[gidx]);

            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(rotl(fval, S[round]));
        }

        self.a = self.a.wrapping_add(a);
        self.b = self.b.wrapping_add(b);
        self.c = self.c.wrapping_add(c);
        self.d = self.d.wrapping_add(d);
    }
}

/// One-shot convenience API
pub fn md5(input: &[u8]) -> [u8; 16] {
    let mut h = Md5::new();
    h.update(input);
    h.finalize()
}

pub fn to_hex(digest: &[u8]) -> String {
    let mut s = String::with_capacity(digest.len() * 2);
    for b in digest {
        use core::fmt::Write;
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
/// One-shot convenience API

pub fn example() {
    let samples = vec![
        ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ("a", "0cc175b9c0f1b6a831c399e269772661"),
        ("abc", "900150983cd24fb0d6963f7d28e17f72"),
        ("message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
        (
            "abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        ),
        (
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        ),
    ];

    println!("🔑 MD5 Example Runs");
    println!("{}", "=".repeat(60));

    for (input, expected) in samples {
        let digest = md5(input.as_bytes());
        let hex_out = to_hex(&digest);
        println!("Input: {:<64} | MD5: {}", format!("\"{}\"", input), hex_out);
        println!("  Expected: {}", expected);
        println!(
            "  Match?   {}\n",
            if hex_out == expected { "✅" } else { "❌" }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, expected_hex: &str) {
        let got = to_hex(&md5(input.as_bytes()));
        assert_eq!(got, expected_hex, "input: {:?}", input);
    }

    #[test]
    fn rfc1321_vectors() {
        check("", "d41d8cd98f00b204e9800998ecf8427e");
        check("a", "0cc175b9c0f1b6a831c399e269772661");
        check("abc", "900150983cd24fb0d6963f7d28e17f72");
        check("message digest", "f96b697d7cb7938d525a2f31aaf161d0");
        check(
            "abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b",
        );
        check(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        );
        check(
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        );
    }
}
