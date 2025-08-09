fn add(x: i32, y: i32) -> i32 {
    x + y
}

// use this attribute to stop rust from adding this code the compiled output
// (cargo build) to help save space
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let res = add(1, 2);
        assert_eq!(res, 3);

        let res = add(3, 4);
        assert_ne!(res, 4);
    }

    #[test]
    fn should_fail() {
        let res = add(1, 1);
        assert_eq!(res, 3, "x and y do not eq 3"); // use 3rd param as a err msg
    }

    #[test]
    #[should_panic(expected = "Just panic here")]
    // #[should_panic] // You do not have to have an expected message
    fn should_throw() {
        panic!("Nope");
    }

    #[test]
    // #[ignore] // use this to ignore the test (e.g. if it takes a long time to run)
    fn using_result() -> Result<(), String> {
        let res = add(1, 2);

        if res == 3 {
            Ok(())
        } else {
            Err("It does not eq 3".to_string())
        }
    }
}

// cargo run // to run all tests
// cargo run add // to run all tests with 'add' in the name
// cargo run -- --show-output // to output println! statements
// cargo run -- --test-threads=1 // to control the threads while running
// cargo run -- --ignore // only run the ignored tests
