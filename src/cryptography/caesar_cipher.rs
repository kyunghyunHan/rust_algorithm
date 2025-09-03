use crate::algorithms::backtracking::knights_tour::example1;

fn caesar_cipher_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + shift) % 26) + b'A') as char
            } else if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + shift) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

fn caesar_cipher_decrypt(text: &str, shift: u8) -> String {
    // 복호화는 암호화의 역순
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + 26 - (shift % 26)) % 26) + b'A') as char
            } else if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + 26 - (shift % 26)) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

pub fn example() {
    let plaintext = "Hello, World!";
    let shift = 3;

    let encrypted = caesar_cipher_encrypt(plaintext, shift);
    let decrypted = caesar_cipher_decrypt(&encrypted, shift);

    println!("Plaintext: {}", plaintext);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
