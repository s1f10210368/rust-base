// 乱数生成

use rand::Rng; // Rngトレイトを導入

fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen(); // 乱数を生成
    println!("{}", x);
}

