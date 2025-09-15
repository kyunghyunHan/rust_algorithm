use algorithm::{
    algorithms::{graph, math, np_complete, sort},
    conding_test,
    cryptography::caesar_cipher,
    cryptography::modular,
    cryptography::roter,
    cryptography::transposition_cipher,
    cryptography::vigenere_cipher,
    cryptography::aes as Aes,
    data_structure,
};

fn main() {
    /* Math */

    // math::prime_factors::example();
    // math::square_root::example();

    /*Sort */
    // algorithms::sort::bubble::example();
    // algorithms::sort::insertion::example();
    // algorithms::sort::shell::example();
    // sort::bubble::example();

    /* Search */

    // algorithms::search::linear_search::example();

    /*data structure */
    // data_structure::array::example();

    /* */
    // caesar_cipher::example();
    // vigenere_cipher::example();
    // transposition_cipher::example()
    // roter::example();
    // Aes::example();
    use aes::Aes128;
    use aes::cipher::{KeyIvInit, BlockEncryptMut, BlockDecryptMut}; // ★ KeyIvInit 가져오기
    use cbc::{Encryptor, Decryptor};
    use block_padding::Pkcs7;
    let key = *b"0123456789abcdef";
    let iv  = *b"abcdef0123456789";
    let plaintext = b"hello world";

    // === 암호화 ===
    // 1) 패딩 포함한 버퍼 준비 (여유 공간 포함)
    let mut buffer = plaintext.to_vec();
    let pos = buffer.len();
    // 최대 16바이트까지 패딩을 넣기 위해 16 추가
    buffer.resize(pos + 16, 0u8);

    // 2) CBC Encryptor 생성
    let cipher = Encryptor::<Aes128>::new(&key.into(), &iv.into());

    // 3) 버퍼를 제자리에서 암호화
    let ct_len = cipher.encrypt_padded_mut::<Pkcs7>(&mut buffer, pos).unwrap().len();
    buffer.truncate(ct_len);
    println!("Cipher (hex): {}", hex::encode(&buffer));

    // === 복호화 ===
    let cipher = Decryptor::<Aes128>::new(&key.into(), &iv.into());
    let pt = cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer).unwrap();
    println!("Plain: {}", String::from_utf8_lossy(pt));
    

}
