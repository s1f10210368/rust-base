struct Rectangle {
    hight: u32,
    widthe: u32,
}

// implを使用すると構造体やトレイトに関する機能をまとめて管理できる

impl Rectangle {
    // 以下のnewのように&selfを引数としないようなものを関連関数という
    fn new(h: u32, w: u32) -> Rectangle {
        Rectangle { hight: h, widthe: w }
    }

    // 以下のareaのように&selfを引数とするものをメソッドという
    fn area(&self) ->u32 {
        self.hight * self.widthe
    }
}

pub fn rectangle_main() {
    let rec = Rectangle::new(3, 5);

    // 関連関数とメソッドで呼び出し方が変化することに注意
    print!("Height={}, Width={}, Area={}", rec.hight, rec.widthe, rec.area());
}