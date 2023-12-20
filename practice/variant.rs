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