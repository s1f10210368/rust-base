// ジェネリクスとは汎用的に肩や関数を使えるようにするための機能

// <>(ダイアモンド演算子)を使用してジェネリック型の名前を指定する

// 関数
fn foo<T>(a: T, b:T) -> T {
    a+b
}

// 列挙型
enum Result<T,E> {
    Ok(T),
    Err(E),
}

// 構造体
struct Point<T> {
    x: T,
    y: T
}

// メソッド
struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn do_something(self) -> (T, T) {
        (self.x, self.y)
    }
}

// トレイト境界という「このトレイトがないと動かない」という制約

// 型引数宣言部に記述
// 以下はGeometryというトレイトがないと動かない
fn draw<T: Geometry>(geometry: &T) {
    ...
}


// where節に記述
// 以下はDisplayというトレイトがないと動かない
fn draw<T>(geometry: &T)
    where T: Display {
    ...
}