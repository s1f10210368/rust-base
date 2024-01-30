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

#[cfg(test)]
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