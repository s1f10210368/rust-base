// 列挙型はenumを用いて定義する
enum ANIMAL {
    DOG,
    CAT,
    RABBIT,
    MOUSE,
}

// 上記を参照したい場合は以下のように
let your_pet = ANIMAL::CAT;
let my_pet = ANIMAL::DOG;

// enumで定義する際にデータを持たせて列挙型を構成することが可能
// 下記ではShape列挙型に対してLine構造体とCircle構造体を列挙しとして並べている
enum Shape {
    Line {x1: i32, y1: i32, x2: i32, y2: i32},
    Circle {x: i32, y: i32, r: i32},
}

// 乱数生成

let mut rng = rand::thread_rng();

let x: i32 = rng.gen();

/*
モジュール
rand には次のようなモジュール

distributions: 確率分布に基づいて乱数を生成する昨日を提供
rngs: 乱数生成器を提供
seq: シーケンスに関連した乱数生成機能を提供
distributions
Standard
Alphanumeric: ASCII文字と数字（a-z, A-Z, 0-9）に一様に分布するu8をサンプリング
Uniform: 2つの境界の間で一様に値をサンプリング
Alphanumeric を用いたランダムな文字列生成
*/

let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
println!("ランダムキャラクター{}: {}", n, chars);

// Uniform を用いた数値間からのランダムな数値選択

let between = Uniform::from(10..10000);
println!("{}", between.sample(&mut rng));