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
