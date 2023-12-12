// クロージャとは関数とその関数が作成された環境をキャプチャして、後でその環境を使用できるもの

// 関数
fn do_something(param: u32) -> u32 { param + 1 }

// クロージャ
let do_something = |param: u32| -> u32 { param + 1 };

// クロージャ (型アノテーションを省略)
let do_something = |param| { param + 1 };

// クロージャ (ブロックを省略, ブロック内の処理が一個だけの場合だけ可能)
let do_something = |param| param + 1 ;


// クロージャを定義すると同時にコンパイラは新しく構造体を作成している
// 主に三種類あり、Fn, FnMut, FnOnceがある。

// Fn トレイト
// &self を受け取る
// FnMut を継承する
// FnOnce を継承する
pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

// FnMutトレイト
// &mut self を受け取る
// FnOnce を継承する
pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

// FnOnce トレイト
// self を受け取る
pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}