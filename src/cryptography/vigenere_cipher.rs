fn vigenere_cipher_encrypt(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let key_bytes: Vec<u8> = key.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c as u8 - b'A')
        .collect();
    
    if key_bytes.is_empty() {
        return text.to_string();
    }
    
    let mut key_index = 0;
    
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'A' + shift) % 26) + b'A') as char
            } else if c.is_ascii_lowercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'a' + shift) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

fn vigenere_cipher_decrypt(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let key_bytes: Vec<u8> = key.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c as u8 - b'A')
        .collect();
    
    if key_bytes.is_empty() {
        return text.to_string();
    }
    
    let mut key_index = 0;
    
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'A' + 26 - shift) % 26) + b'A') as char
            } else if c.is_ascii_lowercase() {
                let shift = key_bytes[key_index % key_bytes.len()];
                key_index += 1;
                (((c as u8 - b'a' + 26 - shift) % 26) + b'a') as char
            } else {
                c
            }
        })
        .collect()
}

fn generate_key_sequence(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let key_chars: Vec<char> = key.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    
    if key_chars.is_empty() {
        return String::new();
    }
    
    let mut key_index = 0;
    let mut result = String::new();
    
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            result.push(key_chars[key_index % key_chars.len()]);
            key_index += 1;
        } else {
            result.push(' ');
        }
    }
    
    result
}

pub fn example() {
    let plaintext = "Hello, World!";
    let key = "KEY";
    
    let encrypted = vigenere_cipher_encrypt(plaintext, key);
    let decrypted = vigenere_cipher_decrypt(&encrypted, key);
    let key_sequence = generate_key_sequence(plaintext, key);
    
    println!("=== Vigenère Cipher Example ===");
    println!("Plaintext:    {}", plaintext);
    println!("Key:          {}", key);
    println!("Key sequence: {}", key_sequence);
    println!("Encrypted:    {}", encrypted);
    println!("Decrypted:    {}", decrypted);
    println!();
    
    // 더 긴 예시
    let long_text = "ATTACKATDAWN";
    let long_key = "LEMON";
    
    let long_encrypted = vigenere_cipher_encrypt(long_text, long_key);
    let long_decrypted = vigenere_cipher_decrypt(&long_encrypted, long_key);
    let long_key_sequence = generate_key_sequence(long_text, long_key);
    
    println!("=== Longer Example ===");
    println!("Plaintext:    {}", long_text);
    println!("Key:          {}", long_key);
    println!("Key sequence: {}", long_key_sequence);
    println!("Encrypted:    {}", long_encrypted);
    println!("Decrypted:    {}", long_decrypted);
    println!();
    
    // 단계별 분석 예시
    step_by_step_analysis("HELLO", "KEY");
}

fn step_by_step_analysis(plaintext: &str, key: &str) {
    println!("=== Step by Step Analysis ===");
    println!("Plaintext: {} | Key: {}", plaintext, key);
    println!("Position | Plain | Key | Shift | Encrypted");
    println!("---------|-------|-----|-------|----------");
    
    let key_chars: Vec<char> = key.chars().collect();
    let mut encrypted = String::new();
    
    for (i, c) in plaintext.chars().enumerate() {
        if c.is_ascii_alphabetic() {
            let key_char = key_chars[i % key_chars.len()];
            let plain_val = (c.to_ascii_uppercase() as u8 - b'A') as u32;
            let key_val = (key_char as u8 - b'A') as u32;
            let encrypted_val = (plain_val + key_val) % 26;
            let encrypted_char = (encrypted_val as u8 + b'A') as char;
            
            println!("    {}    |   {}   |  {}  |   {}   |    {}", 
                     i + 1, 
                     c.to_ascii_uppercase(), 
                     key_char, 
                     key_val, 
                     encrypted_char);
            
            encrypted.push(encrypted_char);
        }
    }
    
    println!("Final encrypted text: {}", encrypted);
}