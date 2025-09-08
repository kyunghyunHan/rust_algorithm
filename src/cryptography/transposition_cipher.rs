// 열 전치 암호 (Columnar Transposition)
fn columnar_transposition_encrypt(text: &str, key: &str) -> String {
    let text = text.replace(" ", "").to_uppercase();
    let key_order = get_key_order(key);
    let cols = key.len();
    let rows = (text.len() + cols - 1) / cols; // 올림 계산
    
    // 그리드 생성 (패딩 포함)
    let mut grid = vec![vec!['X'; cols]; rows];
    
    // 텍스트를 그리드에 채우기
    for (i, c) in text.chars().enumerate() {
        let row = i / cols;
        let col = i % cols;
        if row < rows && col < cols {
            grid[row][col] = c;
        }
    }
    
    // 키 순서대로 열을 읽어서 암호화
    let mut encrypted = String::new();
    for &col_index in &key_order {
        for row in 0..rows {
            encrypted.push(grid[row][col_index]);
        }
    }
    
    encrypted
}

fn columnar_transposition_decrypt(text: &str, key: &str) -> String {
    let cols = key.len();
    let rows = text.len() / cols;
    let key_order = get_key_order(key);
    
    // 그리드 생성
    let mut grid = vec![vec!['X'; cols]; rows];
    
    // 암호문을 키 순서대로 그리드에 채우기
    let mut char_iter = text.chars();
    for &col_index in &key_order {
        for row in 0..rows {
            if let Some(c) = char_iter.next() {
                grid[row][col_index] = c;
            }
        }
    }
    
    // 행별로 읽어서 복호화
    let mut decrypted = String::new();
    for row in 0..rows {
        for col in 0..cols {
            let c = grid[row][col];
            if c != 'X' {
                decrypted.push(c);
            }
        }
    }
    
    decrypted
}

// 키 순서 계산 (알파벳 순서로 정렬)
fn get_key_order(key: &str) -> Vec<usize> {
    let mut indexed_chars: Vec<(char, usize)> = key.to_uppercase().chars().enumerate()
        .map(|(i, c)| (c, i))
        .collect();
    
    indexed_chars.sort_by_key(|&(c, _)| c);
    
    indexed_chars.into_iter().map(|(_, i)| i).collect()
}

// 철로울타리 암호 (Rail Fence Cipher)
fn rail_fence_encrypt(text: &str, rails: usize) -> String {
    if rails <= 1 {
        return text.to_string();
    }
    
    let text = text.replace(" ", "").to_uppercase();
    let mut fence = vec![Vec::new(); rails];
    let mut rail = 0;
    let mut direction = 1; // 1: 아래로, -1: 위로
    
    for c in text.chars() {
        fence[rail].push(c);
        
        if rail == 0 {
            direction = 1;
        } else if rail == rails - 1 {
            direction = -1;
        }
        
        rail = (rail as isize + direction) as usize;
    }
    
    // 각 레일에서 문자들을 순서대로 읽기
    let mut encrypted = String::new();
    for rail in fence {
        for c in rail {
            encrypted.push(c);
        }
    }
    
    encrypted
}

fn rail_fence_decrypt(text: &str, rails: usize) -> String {
    if rails <= 1 {
        return text.to_string();
    }
    
    let len = text.len();
    let mut fence = vec![vec!['\0'; len]; rails];
    let mut rail = 0;
    let mut direction = 1;
    
    // 패턴 마킹
    for col in 0..len {
        fence[rail][col] = '*';
        
        if rail == 0 {
            direction = 1;
        } else if rail == rails - 1 {
            direction = -1;
        }
        
        rail = (rail as isize + direction) as usize;
    }
    
    // 암호문을 레일별로 채우기
    let mut char_iter = text.chars();
    for row in 0..rails {
        for col in 0..len {
            if fence[row][col] == '*' {
                if let Some(c) = char_iter.next() {
                    fence[row][col] = c;
                }
            }
        }
    }
    
    // 지그재그 패턴으로 읽기
    let mut decrypted = String::new();
    rail = 0;
    direction = 1;
    
    for col in 0..len {
        decrypted.push(fence[rail][col]);
        
        if rail == 0 {
            direction = 1;
        } else if rail == rails - 1 {
            direction = -1;
        }
        
        rail = (rail as isize + direction) as usize;
    }
    
    decrypted
}

// 블록 전치 암호 (Block Transposition)
fn block_transposition_encrypt(text: &str, block_size: usize) -> String {
    let text = text.replace(" ", "").to_uppercase();
    let mut encrypted = String::new();
    
    for chunk in text.chars().collect::<Vec<char>>().chunks(block_size) {
        let mut block: Vec<char> = chunk.to_vec();
        // 블록을 뒤집기
        block.reverse();
        encrypted.extend(block);
    }
    
    encrypted
}

fn block_transposition_decrypt(text: &str, block_size: usize) -> String {
    let mut decrypted = String::new();
    
    for chunk in text.chars().collect::<Vec<char>>().chunks(block_size) {
        let mut block: Vec<char> = chunk.to_vec();
        // 블록을 다시 뒤집기
        block.reverse();
        decrypted.extend(block);
    }
    
    decrypted
}

// 그리드 시각화 함수
fn print_grid(text: &str, cols: usize, title: &str) {
    println!("=== {} ===", title);
    
    for (i, c) in text.chars().enumerate() {
        if i % cols == 0 && i > 0 {
            println!();
        }
        print!("{} ", c);
    }
    println!();
    println!();
}

// 철로울타리 시각화
fn visualize_rail_fence(text: &str, rails: usize) {
    println!("=== Rail Fence Visualization (Rails: {}) ===", rails);
    let text = text.replace(" ", "").to_uppercase();
    
    let mut fence = vec![vec![' '; text.len()]; rails];
    let mut rail = 0;
    let mut direction = 1;
    
    for (col, c) in text.chars().enumerate() {
        fence[rail][col] = c;
        
        if rail == 0 {
            direction = 1;
        } else if rail == rails - 1 {
            direction = -1;
        }
        
        rail = (rail as isize + direction) as usize;
    }
    
    for row in fence {
        for c in row {
            print!("{} ", c);
        }
        println!();
    }
    println!();
}

// 단계별 분석
fn step_by_step_columnar_analysis(text: &str, key: &str) {
    println!("=== Step by Step Columnar Transposition Analysis ===");
    println!("Plaintext: {} | Key: {}", text, key);
    
    let text = text.replace(" ", "").to_uppercase();
    let key_order = get_key_order(key);
    let cols = key.len();
    let rows = (text.len() + cols - 1) / cols;
    
    println!("Key order: {:?}", key_order);
    println!("Grid size: {} rows × {} columns", rows, cols);
    println!();
    
    // 그리드 생성 및 출력
    let mut grid = vec![vec!['X'; cols]; rows];
    
    println!("1. Fill grid row by row:");
    println!("   {}", key);
    println!("  {}", "-".repeat(key.len() * 2 - 1));
    
    for (i, c) in text.chars().enumerate() {
        let row = i / cols;
        let col = i % cols;
        if row < rows && col < cols {
            grid[row][col] = c;
        }
    }
    
    for row in 0..rows {
        print!("  ");
        for col in 0..cols {
            print!("{} ", grid[row][col]);
        }
        println!();
    }
    println!();
    
    println!("2. Read columns in key order:");
    let mut encrypted = String::new();
    for &col_index in &key_order {
        let key_char = key.chars().nth(col_index).unwrap();
        print!("   Column '{}' (position {}): ", key_char, col_index + 1);
        for row in 0..rows {
            let c = grid[row][col_index];
            print!("{}", c);
            encrypted.push(c);
        }
        println!();
    }
    
    println!();
    println!("Final encrypted text: {}", encrypted);
    println!();
}

pub fn example() {
    // 열 전치 암호 예제
    let plaintext = "MEET ME AT MIDNIGHT";
    let key = "SECRET";
    
    let encrypted = columnar_transposition_encrypt(plaintext, key);
    let decrypted = columnar_transposition_decrypt(&encrypted, key);
    
    println!("=== Columnar Transposition Example ===");
    println!("Plaintext:  {}", plaintext);
    println!("Key:        {}", key);
    println!("Encrypted:  {}", encrypted);
    println!("Decrypted:  {}", decrypted);
    println!();
    
    // 단계별 분석
    step_by_step_columnar_analysis(plaintext, key);
    
    // 철로울타리 암호 예제
    let rail_text = "HELLO WORLD";
    let rails = 3;
    
    let rail_encrypted = rail_fence_encrypt(rail_text, rails);
    let rail_decrypted = rail_fence_decrypt(&rail_encrypted, rails);
    
    println!("=== Rail Fence Cipher Example ===");
    println!("Plaintext:  {}", rail_text);
    println!("Rails:      {}", rails);
    println!("Encrypted:  {}", rail_encrypted);
    println!("Decrypted:  {}", rail_decrypted);
    println!();
    
    // 철로울타리 시각화
    visualize_rail_fence(rail_text, rails);
    
    // 블록 전치 암호 예제
    let block_text = "ATTACKATDAWN";
    let block_size = 4;
    
    let block_encrypted = block_transposition_encrypt(block_text, block_size);
    let block_decrypted = block_transposition_decrypt(&block_encrypted, block_size);
    
    println!("=== Block Transposition Example ===");
    println!("Plaintext:   {}", block_text);
    println!("Block size:  {}", block_size);
    println!("Encrypted:   {}", block_encrypted);
    println!("Decrypted:   {}", block_decrypted);
    println!();
    
    // 블록별 시각화
    print_grid(block_text, block_size, "Original blocks");
    print_grid(&block_encrypted, block_size, "Encrypted blocks (reversed)");
    
    // 추가 예제들
    println!("=== Additional Examples ===");
    
    // 다른 키로 열 전치
    let plaintext2 = "THE QUICK BROWN FOX";
    let key2 = "ZEBRA";
    let encrypted2 = columnar_transposition_encrypt(plaintext2, key2);
    println!("Text: {} | Key: {} | Encrypted: {}", plaintext2, key2, encrypted2);
    
    // 다른 레일 수로 철로울타리
    let rail_text2 = "CRYPTOGRAPHY";
    let rails2 = 4;
    let rail_encrypted2 = rail_fence_encrypt(rail_text2, rails2);
    println!("Text: {} | Rails: {} | Encrypted: {}", rail_text2, rails2, rail_encrypted2);
    
    visualize_rail_fence(rail_text2, rails2);
}