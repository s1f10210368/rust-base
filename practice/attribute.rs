#[derive(Debug,PartialEq)]

// ↑幽霊型を用いていた構造体の宣言時に使っていた。

#[derive(Debug,PartialEq)]
struct PhantomStruct<X, A> {
    value: A,
    phantom: PhantomData<X>
}

// これは derive という属性を構造体に付与して、derive が提供している性質によって 
// Debug と PartialEq という振る舞いを追加していたのです。


/*
実はこの derive は Rust の手続きマクロの一種になります。Rust にはマクロの定義の仕方が複数あります:

宣言型マクロ
手続き型マクロ
カスタム Derive マクロ
属性型マクロ
関数型マクロ
*/


/*
属性 (アトリビュート) とは、以下のような Rust の構文に対して メタデータ (追加機能) を追加できるような書式です:

クレート
モジュール
関数
構造体
など
*/

#[item_attribute]

// クレート全体に適用する場合の属性は、! を追加する必要がある

#![crate_attribute]

// 属性に対するパラメータの設定はいくつかの書式:

#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
// 複数設定も可能

#[attribute(value1, value2, value3)]
/* 
属性の用途は主に以下の通り

コンパイル時の条件分岐
クレート名、バージョン、種類（バイナリか、ライブラリか）の設定
リントの無効化 (警告の抑止)
コンパイラ付属の機能（マクロ、glob、インポートなど）の有効化
外部ライブラリへのリンク
ユニットテスト用の関数として明示
ベンチマーク用の関数として明示
*/

/*
頻出の属性
使用頻度が比較的高いと思われる属性:

test 属性
cfg 属性
derive 属性
allow 属性
deny 属性
test 属性
単体テストを作成するときに使用する属性
#[test] と属性がついている関数のみが単体テストとして実施

cargo test で実行。
*/

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

/*
cfg 属性
条件を宣言し、コンパイル時のその条件に応じたコンパイルを実施するための属性
*/

// #[cfg] に条件をパラメータ設定して使用する。

#[cfg(windows)]
fn something_for_windows(){
   // Windows 環境に依存した処理を記述
}

// derive 属性
// derive 属性に対応したトレイトの実装を自動的に構造体や列挙型に実装することのできる属性

// Debug を実装する場合:

// Debugトレイトの fmt 関数が自動的に実装されているので、:?フォーマット文字列をつかうことができる

#[derive(Debug)]
struct Point{ x: i32, y: i32, z: i32 }

fn main(){
    let some_point = Point {x: 10, y: 20, z: 0};
    println!("Debug: {:?}", some_point);
}

[Running] 
Debug: Point { x: 10, y: 20, z: 0 }

[Done] exited with code=0 in _.___ seconds

// allow 属性
// Rust には Lint チェックというソースコードの静的解析をしてくれるしくみがあります。

// リント一覧
// そのチェック対象とされているリント項目を無視するようにするための属性です。

fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn main() {
    used_function();
}

// unused_function 関数は使用されていないので警告が通常なら出力されますが、#[allow(dead_code)] によりリントチェック dead_code ルールを無視するようにしています。そのため警告が出力されません。

// deny 属性
// allow 属性とは逆に、リントチェックの内容を全てエラーとする属性です。

// 次の例はアンチパターンとして有名なもの
// 設定しないで欲しいのが、問題があればビルドを停止させるために 
//deny 属性でリントチェックの warning ルールを設定している

#![deny(warnings)]