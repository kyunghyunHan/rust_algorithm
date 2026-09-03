use super::super::utils;
use std::io::{stdin, BufRead, BufReader};
use std::ptr::null;
use std::{cmp, ptr};
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
    println!("연락처 추가 완료");
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

fn find_contract(
    contacts: *const Contact,
    count: i32,
    name: *const u8,
) -> *const Contact {
    unsafe {
        for i in 0..count {
            let contact = contacts.add(i as usize);

            for j in 0..20 {
                let contact_name = (*contact).name[j];
                let search_name = *name.add(j);

                // 둘 다 문자열 끝까지 같으면 찾은 것
                if contact_name == 0 && search_name == 0 {
                    return contact;
                }

                // 중간에 하나라도 다르면 다음 연락처
                if contact_name != search_name {
                    break;
                }
            }
        }

        std::ptr::null()
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
    let mut name = [0u8; 20];
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
        println!("{}", menu);
        println!("선택: ");

        drop(reader); // ← 여기서 lock 해제

        match menu {
            "0\n" => {
                println!("종료합니다.");
                return;
            }
            "1\n" => {
                add_contract(contacts.as_mut_ptr(), &mut count);
            }
            "2\n" => {
                printf(contacts.as_mut_ptr(), count);
            }
            "3\n" => {
                println!("검색할 이름 : ");
                input.clear();
                let mut reader = BufReader::new(stdin().lock());
                reader.read_line(&mut input).unwrap();
                let name_bytes = input.trim().as_bytes();
                name[..name_bytes.len()].copy_from_slice(name_bytes);
                let result: *const Contact = find_contract(contacts.as_ptr(), count, name.as_ptr());
                println!("this");

                unsafe {
                    if result != null() {
                        println!("이름 : {:?}", *result)
                    }else{
                        println!("검색대지 않았스빈다.");

                    }
                }
            }
            _ => {}
        }
    }
}
