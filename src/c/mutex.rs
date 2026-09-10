use std::sync::Mutex;

pub fn example() {
    let x: Mutex<i32> = Mutex::new(10);
    {
        let mut guard = x.lock().unwrap(); // 🔒

        *guard += 1;
    }
    {
        *x.lock().unwrap() += 1;
        println!("{:?}", x);
    }
}
