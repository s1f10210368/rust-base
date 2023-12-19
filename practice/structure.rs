// 構造体

struct Employee {
    username: String,  // <-- 個々の要素をフィールドという
    email: String,
    employee_no: u32,
    role: String,
}

// キー:値という形式で束縛して生成
let emp = Employee {
    username: String::from("yanashin18618"),
    email: String::from("yanashin@email.com"),
    employee_no = 1,
    role: String::from("Engineer"),
};

// 構造体自体を返す関数を定義することも可能
fn create_employee(username: &String, email: &String, employee_no: u32, role: &String) -> Employee {
    Employee {
        username = username.to_string(),
        email = email.to_string(),
        employee_no = employee_no, // <-- 引数でemployee_noを指定しているため省略が可能
        role = role.to_string(),
    }
}