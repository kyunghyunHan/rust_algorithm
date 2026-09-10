fn main() {
    let n = 3;
    let k = 20;
    let a = [0, 6, 8, 9];

    let mut answer = false;

    for i in 0..1 << n {
        println!("{}", i);
        let mut sum = 0;
        for j in 1..=n {
            let wari = 1 << (j - 1);
            if (i / wari) % 2 == 1 {
                sum += a[j]
            }
        }
        if sum == k {
            answer = true;
        }
    }
    if answer == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
