fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[test]
fn should_added_number_when_two_numbers() {
    assert_eq!(9, add(3, 6))
}

#[test]
fn should_subtracted_number_when_two_numbers() {
    assert_eq!(3, sub(6, 3))
}

// rust ではテストコードをひとまとめにすることができる

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]  // cargo test でだけ動くようになっている
mod tests {
    use super::*;
    
    #[test]
    fn should_added_number_when_two_numbers() {
        assert_eq!(9, add(3, 6))
    }

    #[test]
    fn should_subtracted_number_when_two_numbers() {
        assert_eq!(3, sub(6, 3))
    }
}

/*
assert!(x);	x が ture ならば正常、異なればパニック
assert_eq!(x, y);	x == y ならば正常、異なればパニック
assert_ne!(x, y);	x != y ならば正常、異なればパニック
*/

use day_55_test;
#[test]
fn integration_test() {
    assert_eq!(3,day_55_test::sub(day_55_test::add(3, 6), day_55_test::add(2, 4)));
}
