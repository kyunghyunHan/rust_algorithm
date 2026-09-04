use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

fn example() {
    let output = Command::new("echo")
        .arg("Hello Process")
        .output()
        .expect("failed");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    let handle = thread::spawn(|| {
        println!("Thread Running");
    });

    handle.join().unwrap();

    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let d = Arc::clone(&data);

        let h = thread::spawn(move || {
            let mut value = d.lock().unwrap();
            *value += 1;
        });

        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Result = {}", *data.lock().unwrap());
}
