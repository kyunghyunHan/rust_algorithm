use std::ffi::NulError;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::ptr::null;
use std::ptr::null_mut;

const MAX_ITMEMS: usize = 10;
#[derive(Default)]
struct Item {
    name: [u8; 20],
    attack: i32,
    price: i32,
}
#[derive(Default)]
struct Player {
    name: [u8; 20],
    hp: i32,
    attack: i32,
    inventory: [Item; MAX_ITMEMS],
    item_count: i32,
    weapon: *mut Item,
}

struct Monster {
    name: [u8; 20],
    hp: i32,
    attack: i32,
}

fn init_player(player: *mut Player) {
    unsafe {
        let name = "바보".as_bytes();

        let name_ptr: *mut u8 = (*player).name.as_mut_ptr();
        for i in 0..name.len() {
            *name_ptr.add(i) = name[i];
        }
        (*player).hp = 200;
        (*player).attack = 20;
        (*player).inventory[0].attack = 10;
        (*player).inventory[0].price = 100;

        (*player).item_count = 1;
        (*player).weapon = null_mut();
    }
}

fn print_player(player: *const Player) {
    unsafe {
        let len = (*player).item_count;
        let index = len - 1;
        print!("이름 : {:?}\n", (*player).name);
        print!("hp : {:?}\n", (*player).hp);
        print!("attack : {:?}\n", (*player).attack);
        print!("가지고 있는 아이템 수 : {:?}\n", len);
        for i in 0..len {
            print!("무기 이름 : {:?}\n", (*player).inventory[i as usize].name);
            print!(
                "무기 공격력 : {:?}\n",
                (*player).inventory[i as usize].attack
            );
            print!("무기 가격  : {:?}\n", (*player).inventory[i as usize].price);
        }
        if (*player).weapon.is_null() {
            println!("장착 아이템 : 없음");
        } else {
            println!("장착 아이템 : {:?}", (*(*player).weapon).name);
        }    }
}
pub fn example() {
    let mut player: *mut Player = &mut Player::default();
    let mut data = Player::default();

    let player: *mut Player = &mut data;

    init_player(player);
    let mut menu;
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());

    init_player(player);

    loop {
        print!("\n");
        print!("===== RPG GAME =====\n");

        print!("0. 종료\n");
        print!("1. 플레이어 정보\n");
        print!("2. 아이템 추가\n");
        print!("3. 인벤토리 출력\n");
        print!("4. 아이템 검색\n");
        print!("5. 아이템 삭제\n");
        print!("6. 무기 장착\n");
        print!("7. 인벤토리 정렬\n");
        print!("8. 몬스터와 전투\n");

        print!("선택: ");
        input.clear();
        reader.read_line(&mut input).unwrap();
        menu = input.trim().parse::<i32>().unwrap();
        // let input = input.as_bytes();
        // name[..name_bytes.len()].copy_from_slice(name_bytes);
        init_player(player);

        match menu {
            0 => {
                println!("게임종료");
                return;
            }
            1=> {
                print_player(player);
      
            }
            _=>{

            }
        }
    }
}
