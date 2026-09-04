use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::str::FromStr;

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

fn divi(a: i32, b: i32) -> i32 {
    return a / b;
}

fn mods(a: i32, b: i32) -> i32 {
    return a % b;
}
struct Op {
    name: *const [u8; 20],
    func: fn(i32, i32) -> i32,
}
const OP_COUNT: usize = 5;

fn init(ops: &mut [Op; OP_COUNT]) {
    const fn name(value: &str) -> [u8; 20] {
        let bytes = value.as_bytes();
        let mut result = [0; 20];
        let mut i = 0;

        while i < bytes.len() && i < result.len() {
            result[i] = bytes[i];
            i += 1;
        }

        result
    }

    ops[0] = Op {
        name: &name("더하기"),
        func: add,
    };
    ops[1] = Op {
        name: &name("빼기"),
        func: sub,
    };
    ops[2] = Op {
        name: &name("곱하기"),
        func: mul,
    };
    ops[3] = Op {
        name: &name("나누기"),
        func: divi,
    };
    ops[4] = Op {
        name: &name("나눈값"),
        func: mods,
    };
}

fn read_number<T: FromStr>(reader: &mut impl BufRead, prompt: &str) -> Option<T> {
    loop {
        print!("{prompt}");
        stdout().flush().ok()?;

        let mut input = String::new();
        match reader.read_line(&mut input) {
            Ok(0) => return None,
            Ok(_) => match input.trim().parse::<T>() {
                Ok(number) => return Some(number),
                Err(_) => println!("숫자를 입력해주세요."),
            },
            Err(error) => {
                eprintln!("입력 오류: {error}");
                return None;
            }
        }
    }
}

pub fn example() {
    let mut ops = [
        Op {
            name: std::ptr::null(),
            func: add,
        },
        Op {
            name: std::ptr::null(),
            func: add,
        },
        Op {
            name: std::ptr::null(),
            func: add,
        },
        Op {
            name: std::ptr::null(),
            func: add,
        },
        Op {
            name: std::ptr::null(),
            func: add,
        },
    ];

    let mut total = 0;
    let mut reader = BufReader::new(stdin().lock());
    init(&mut ops);
    loop {
        print!("======================\n");
        print!("0. 종료\n");
        print!("1. 더하기\n");
        print!("2. 빼기\n");
        print!("3. 곱하기\n");
        print!("4. 나누기\n");
        let Some(menu): Option<usize> = read_number(&mut reader, "연산 번호를 입력하세요: ")
        else {
            println!("\n입력이 종료되었습니다.");
            return;
        };

        match menu {
            0 => {
                print!("종료\n");
                return;
            }
            1 => {
                print!("====={:?}======\n", ops[menu - 1].name);

                let Some(n) = read_number(&mut reader, "입력 : ") else {
                    println!("\n입력이 종료되었습니다.");
                    return;
                };
                total = (ops[menu - 1].func)(total, n);
                println!("종합 : {}", total);
            }
            2 => {
                print!("====={:?}======\n", ops[menu - 1].name);

                let Some(n) = read_number(&mut reader, "입력 : ") else {
                    println!("\n입력이 종료되었습니다.");
                    return;
                };
                total = (ops[menu - 1].func)(total, n);
                println!("종합 : {}", total);
            }
            3 => {
                print!("====={:?}======\n", ops[menu - 1].name);

                let Some(n) = read_number(&mut reader, "입력 : ") else {
                    println!("\n입력이 종료되었습니다.");
                    return;
                };
                total = (ops[menu - 1].func)(total, n);
                println!("종합 : {}", total);
            }
            4 => {
                print!("====={:?}======\n", ops[menu - 1].name);
                let Some(n) = read_number(&mut reader, "입력 : ") else {
                    println!("\n입력이 종료되었습니다.");
                    return;
                };
                if n == 0 {
                    println!("0으로 나눌 수 없습니다.");
                    continue;
                }
                total = (ops[menu - 1].func)(total, n);
                println!("종합 : {}", total);
            }
            _ => {}
        }
    }
}
