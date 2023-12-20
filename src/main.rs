// 型について
std::convert; // <-- 型を変換するために使用するトレイト

// FromとIntoなどトレイト毎に異なった変換を実施

std::convert::Into; // <-- 現在のクレートの外側の型への値から値への変換
std::convert::From; // <-- 値から値への変換

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