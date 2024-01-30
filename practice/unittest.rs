fn print_message(msg: String) -> String {
    println!("{}", msg);
    msg
}

// 以下のように[test]をつけるとテストを行える
#[test]
fn test_message() {
    assert_eq!("Hello", print_message("Hello".to_string()));
}

//特定のものだけ実行したい場合は関数名を cargo test --- と入力