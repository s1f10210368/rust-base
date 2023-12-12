// Fizzbuzzとは1から指定された上限までの数を数え上げて
// 各数に対して特定の条件を満たす場合に特定のメッセージを返すもの

pub mod rectangle;

fn main() {
    fizzbyzz_to(30);
    rectangle::rectangle_main();
}

// 引数の後には必ず型指定
// 戻り値は -> の後ろに型を指定

fn is_divisible_by(x: u32, y: u32) -> bool {
    if y == 0 {
        return false;
    }
    x % y == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("{}: fizzbuzz", n);
    } else if is_divisible_by(n, 3) {
        println!("{}: fizz", n)
    } else if is_divisible_by(n, 5) {
        println!("{}: buzz", n)
    } else {
        println!("{}: ()", n)
    }
}

fn fizzbyzz_to(upper_limit: u32) {
    for e in 1..=upper_limit {
        fizzbuzz(e)
    }
}