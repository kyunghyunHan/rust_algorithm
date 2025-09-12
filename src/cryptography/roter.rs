/// 아주 단순화한 Enigma (플러그보드 + 로터3개 + 리플렉터)
fn plugboard(c: char) -> char {
    match c {
        'A' => 'B',
        'B' => 'A',
        _ => c,
    }
}

fn rotor_forward(c: char, rotor: &[u8; 26], offset: u8) -> char {
    let idx = (c as u8 - b'A' + offset) % 26;
    let mapped = rotor[idx as usize];
    ((mapped + 26 - offset) % 26 + b'A') as char
}

fn rotor_backward(c: char, rotor: &[u8; 26], offset: u8) -> char {
    let idx = (c as u8 - b'A' + offset) % 26;
    let pos = rotor.iter().position(|&x| x == idx).unwrap() as u8;
    ((pos + 26 - offset) % 26 + b'A') as char
}

fn reflector(c: char) -> char {
    (b'Z' - (c as u8 - b'A')) as char
}

fn enigma(text: &str, r1: &[u8;26], r2: &[u8;26], r3: &[u8;26], pos: (u8,u8,u8)) -> String {
    let mut result = String::new();
    let mut counter = 0;

    for c in text.chars() {
        if !c.is_ascii_uppercase() {
            result.push(c);
            continue;
        }
        let mut x = plugboard(c);

        let o1 = (pos.0 as usize + counter % 26) as u8;
        let o2 = (pos.1 as usize + (counter / 26) % 26) as u8;
        let o3 = (pos.2 as usize + (counter / (26 * 26)) % 26) as u8;
        

        x = rotor_forward(x, r1, o1);
        x = rotor_forward(x, r2, o2);
        x = rotor_forward(x, r3, o3);

        x = reflector(x);

        x = rotor_backward(x, r3, o3);
        x = rotor_backward(x, r2, o2);
        x = rotor_backward(x, r1, o1);

        x = plugboard(x);
        result.push(x);
        counter += 1;
    }
    result
}

pub fn example() {
    // 간단한 로터들 (실제 역사랑 다름)
    let rotor1: [u8; 26] = [4,10,12,5,11,6,3,16,21,25,13,19,14,22,24,7,23,20,18,15,0,8,1,17,2,9];
    let rotor2: [u8; 26] = [0,9,3,10,18,8,17,20,23,1,11,7,22,19,12,2,16,6,25,13,15,24,5,21,14,4];
    let rotor3: [u8; 26] = [1,3,5,7,9,11,2,15,17,19,23,21,25,13,24,4,8,22,6,0,10,12,20,18,16,14];

    let plaintext = "WEATTACKATDAWNHEILHITLER";
    let pos = (0,0,0);

    // 암호화
    let ciphertext = enigma(plaintext, &rotor1, &rotor2, &rotor3, pos);
    println!("Ciphertext: {}", ciphertext);

    // 영화식 해독: 가능한 모든 시작 위치를 시도하며 "HEILHITLER"가 있는지 찾음
    for a in 0..26 {
        for b in 0..26 {
            for c in 0..26 {
                let guess = enigma(&ciphertext, &rotor1, &rotor2, &rotor3, (a,b,c));
                if guess.contains("HEILHITLER") {
                    println!("Found key ({},{},{}) -> {}", a,b,c, guess);
                    return;
                }
            }
        }
    }
}
