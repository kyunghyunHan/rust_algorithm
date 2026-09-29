use std::{
    collections::{HashMap, HashSet},
    env::join_paths,
};

pub fn example() {
    split_test();
}

fn split_test() {
    let a = "www.jaen.comx";
    let b = a.split(" ").collect::<Vec<&str>>();
    let b = a.split_whitespace().collect::<Vec<&str>>();
    let c = " ".to_string();
    let d = a.strip_suffix("w");

    println!("{:?}", d);
}

fn test() {
    let mut a: HashSet<i32> = HashSet::new();

    a.insert(1);
    a.get(&1);

    let mut b: HashMap<i32, i32> = HashMap::new();
    b.insert(1, 2);
    //키가 없으며 넣고 있으면 기존값 유지
    b.entry(1).or_insert(1);

    let mut a = [0, 1, 2, 34];
    let mut d = a.iter();
    let b = d.next().unwrap();
    let c = d.next().unwrap();

    let mut c = [0, 1, 2, 34];
    //enumerate index까지줌
    let e = c.iter().enumerate();

    for x in e {
        println!("{:?}", x);
    }
}
