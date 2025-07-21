fn variables() {
    // variables (let) are constant by default
    let x = 50;

    // Will not work
    // x = 25;

    // you can redeclare variables my using the `let` keyword
    let x = 100;

    // this is mutable (can change)
    let mut x = 50;
    x = 25;

    // - mutable variables cannot change data type
    // unless they are redeclared with `let`
}

fn consts() {
    // there are `consts` but there are differences
    // - consts must be uppercase and camel case
    // - consts need to be explicitly typed
    // - consts cannot be mutable
    const MY_NUM: u16 = 2212;
}

fn scopes() {
    let mut x = 10;
    {
        x += 5;
        println!("x: {x}");
    }
    println!("x: {x}");
    // both times x will be 15
    // this is because we are using the same variable for it

    let mut x = 10;
    {
        let x = x + 5;
        println!("x: {x}"); // 15
    }
    println!("x: {x}") // 10
    // as we are redeclaring x it will stay as 10 outside of the scope
}

fn data_types() {
    // ints and unsigned ints
    let int: i32 = 0;
    let int: u32 = 20;

    // size of system (32bit/64bit)
    let int: usize = 10;
    let int: isize = 10;

    // Doubles (floats)
    let float = 2.0
    let float :f32 = 2.0;

    // Characters
    let char = 'z';
    let char: char = 'b';

    // tuples
    // - elements can be of different type
    let tup = (14, 18, 21);
    let tup: (isize, i32, &str) = (10, 10, "bob");
    let (_, _, name) = tup;
    let name = tup.2;

    // arrays
    // elements must be of same type
    let a = [1,2,3];
    let a: [i64; 1] = [1];
    // create an array with 5 elements set to 3
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first_el = a.first().unwrap();
    let first_el = a[0];

    let s = "10";
    let as_num: usize = s.parse().expect("Enter a number");
}
