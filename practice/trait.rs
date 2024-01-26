// 32 bit の整数型の要素を２つもつ構造体
struct Point(i32, i32);

// Point の座標があっているかどうかを確認するトレイト
trait Position<X, Y> {
    fn exist(&self, _: &X, _: &Y) -> bool;
    fn h_axis(&self) -> i32;
    fn v_axis(&self) -> i32;
}

impl Position<i32, i32> for Point {
    // ２つの要素が正しいことを確認
    fn exist(&self, x: &i32, y: &i32) -> bool {
        (&self.0 == x) && (&self.1 == y)
    }

    // x座標を取得
    fn h_axis(&self) -> i32 {
        self.0
    }

    // y座標を取得
    fn v_axis(&self) -> i32 {
        self.1
    }
}

fn main() {
    let x = 5;
    let y = 10;

    let point = Point(x, y);

    println!("Point X:{}, Y:{}", &x, &y);
    println!("Exist?:{}", point.exist(&x, &y));

    println!("Point-X:{}", point.v_axis());
    println!("Point-X:{}", point.h_axis());
}


// -------- 上記を関連型に変更すると以下のようになる ------

trait Position {
    type X;
    type Y;

    fn exist(&self, _: &Self::X, _: &Self::Y) -> bool;
    fn h_axis(&self) -> i32;
    fn v_axis(&self) -> i32;
}

fn new_point<Z: Position>(point: &Z) {
    println!("POINT:({},{})", point.v_axis(), point.h_axis())
}

/*
関連型
「トレイト」 : 「実装対象の型 (Self)」 = 1 : 1
ジェネリクス
「トレイト」 : 「実装対象の型 (Self)」 = N : 1
*/

// ------------------------------------------------------------------

// 型について
std::convert; // <-- 型を変換するために使用するトレイト

// FromとIntoなどトレイト毎に異なった変換を実施

std::convert::Into; // <-- 現在のクレートの外側の型への値から値への変換
std::convert::From; // <-- 値から値への変換

// ------------------------------------------------------------------

// Fromトレイト
// ある型に対して別の型からその型を作る方法を定義するようにするものです。
pub trait From<T> {
    fn from(T) -> Self;
}

// 例としてstrに対してStringを作る場合は次のように
let str = "example";
let String = String::from(str);

// i32から自作のNumber型を作る場合は次のように
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

let num = Number::from(30);

/*
impl From<変換元> for 変換先 {
    fn from(from: 変換元) -> 変換先 {
    }
}
*/

// Intoトレイト
// intoトレイトはFromトレイトとは逆の関係になっている
pub trait Into<T> {
    fn into(self) -> T;
}

// 自作の型に From トレイトが実装されている場合、Into は必要に応じてそれを呼び出す

struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}