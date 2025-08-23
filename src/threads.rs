use std::thread;

pub fn threads() {
    thread::spawn(|| {
        print_to_five();
    });

    let handle = thread::spawn(|| {
        print_to_five();
    });

    print_to_five();

    let vec = vec![1, 2, 3];

    thread::spawn(move || {
        println!("Vector: {:?}", vec);
    });

    // vec not valid here

    handle.join().unwrap();
}

fn print_to_five() {
    for i in 1..=5 {
        println!("{}", i);
    }
}
