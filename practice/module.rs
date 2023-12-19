// クレートとは
// ソースコードを格納するところ、実行バイナリクレートとライブラリクレートの二種類がある
// 実行バイナリクレートはmain関数を持つところ、ライブラリクレートはそれ以外のクレート

// モジュールとは
// クレートの中の要素に対して空間を付与して階層構造を持たせる仕組み
// modを使用してモージュールを定義

mod Module1 {
    mod Module2 {
        fn doSomething()
    }

    mod Module3 {
        fn doSomethingElse()
        fn doNothing()
    }
}
// mod の前にpubをつけると外部にも公開されるように
// 外部に公開しているモジュールを使用するには「パス」を考慮しなければいけない(javaでいうパッケージ)

pub mod Module1 {
    pub mod Module2 {
        pub fn doSomething()
    }

    pub mod Module3 {
        pub fn doSomethingElse()
        pub fn doNothing()
    }
}

// パスの指定方法について
// 絶対パスと相対パスの二種類が存在

// 上記のdoSomethingを指定する場合には以下のように
crate::Module1::Module2::doSomething

// 以下のようにまとめることも可能
// 冗長な書き方
use crate::Module1::Module2::doSomething
use crate::Module1::Module3::doSomethingElse

// まとめると
use crate::Module1::{Module2::doSomething, Module3::doSomethingElse}