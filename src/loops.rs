pub fn loops() {
    loop {
        println!("Will repeat forever until break");
        break;
    }

    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count;
        }
    };
    println!("Result: {}", result);
    // Result: 10

    let mut count = 0;
    'count_loop: loop {
        'inner_loop: loop {
            if count == 5 {
                break 'inner_loop;
            }

            if count == 10 {
                break 'count_loop;
            }

            count += 1;
        }

        println!("Outer loop");
        count += 1;
    }

    let mut count = 0;
    loop {
        if count == 10 {
            break;
        }

        if count == 5 {
            continue;
        }

        println!("Count: {count}");

        count += 1;
    }
    // will count from 0 to 4, then skip 5, then count from 6 to 9
}

pub fn whiles() {
    let mut num = 0;
    while num < 100 {
        println!("Num: {num}");
        num += 1;
    }
}

pub fn for_loop() {
    let a = [1, 2, 3];

    for el in a {
        println!("el: {}", el);
    }
    // el: 1
    // el: 2
    // el: 3
}
