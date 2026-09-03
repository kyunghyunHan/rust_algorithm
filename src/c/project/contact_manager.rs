use super::super::utils;
use std::io::{stdin, BufRead, BufReader};
use std::ptr;
#[derive(Debug, Clone, Copy)]
struct Contact {
    name: [u8; 20],
    phone: [u8; 20],
}
const MAX_CONTACTS: usize = 100;

fn add_contract(contacts: *mut Contact, count: *mut i32) {
    let mut name = [0u8; 20];
    let mut phone = [0u8; 20];

    let mut name_input = String::new();
    let mut phone_input = String::new();

    let mut reader = BufReader::new(stdin().lock());

    reader.read_line(&mut name_input).unwrap();

    reader.read_line(&mut phone_input).unwrap();

    let name_bytes = name_input.trim().as_bytes();
    let phone_bytes = phone_input.trim().as_bytes();

    name[..name_bytes.len()].copy_from_slice(name_bytes);
    phone[..phone_bytes.len()].copy_from_slice(phone_bytes);
    let len = name.iter().position(|&x| x == 0).unwrap_or(name.len());
    let len2 = phone.iter().position(|&x| x == 0).unwrap_or(phone.len());

    let name_str = std::str::from_utf8(&name[..len]).unwrap();
    let phone_str = std::str::from_utf8(&phone[..len2]).unwrap();
    let test = Contact { name, phone };
    unsafe {
        *contacts.add(*count as usize) = test;
        *count += 1;
    }

    println!("{:?}", name_str);
    println!("{:?}", phone_str);
}

fn printf(contract: *mut Contact, count: i32) {
    for i in 0..count {
        unsafe {
            let contact = &*contract.add(i as usize);
            let name_len = contact
                .name
                .iter()
                .position(|&x| x == 0)
                .unwrap_or(contact.name.len());
            let phone_len = contact
                .phone
                .iter()
                .position(|&x| x == 0)
                .unwrap_or(contact.phone.len());
            let name_str = std::str::from_utf8(&contact.name[..name_len]).unwrap();
            let phone_str = std::str::from_utf8(&contact.phone[..phone_len]).unwrap();

            println!("name : {:?}, phone : {:?}", name_str, phone_str);
        }
    }
}
pub fn example() {
    let mut count: i32 = 0;
    let mut menu;
    let menu_text = Vec::from([
        "종료",
        "연락처 추가",
        "전체 출력",
        "연락처 검색",
        "연락처 삭제",
        "이름순 정렬",
    ]);
    let mut contacts: [Contact; MAX_CONTACTS] = [Contact {
        name: [0; 20],
        phone: [0; 20],
    }; MAX_CONTACTS];

    loop {
        println!("===== Contact Manager =====");

        for i in 0..6 {
            println!("{} {}", i, menu_text[i])
        }
        let mut input = String::new();
        let mut reader = BufReader::new(stdin().lock());

        reader.read_line(&mut input).unwrap();
        menu = input.as_str();
        println!("선택: ");

        match menu {
            "0" => {
                println!("종료합니다.");
            }
            "1" => {
                add_contract(contacts.as_mut_ptr(), count as *mut i32);
                break;
            }
            "2" => {
                printf(contacts.as_mut_ptr(), count);
                break;
            }
            _ => {}
        }
    }
}
