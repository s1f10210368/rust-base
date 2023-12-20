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