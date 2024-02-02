let optional = Some(5);
match optional {
    Some(i) => {
        println!("Matched {:?}!", i)
    },
    _ => {},        
}

// 上記にif-letを適用すると以下のようになる

if let Some(i) = optional {
    println!("Matched {:?}!", i);
}

// 以下のようにelseも使用可能

if let Some(i) = optional {
    println!("マッチしました {:?}!", i);
} else {
    println!{"マッチしません {:?}!", i};
}

// 以下はlet-elseを用いる方法

let パターン = 値 else { never型を返す処理 };