use main;
#[test]
fn integration_test() {
    assert_eq!(3,day_55_test::sub(day_55_test::add(3, 6), day_55_test::add(2, 4)));
}

#[test]
fn test_add() {
    assert!(add(1, 2) ==  3);       // 正常（ 1 + 2 == 3 -> true ）
    assert_eq!(add(1, 2), 3);       // 正常（ 1 + 2 == 3 ）
    assert_ne!(add(1, 2), 4);      
    assert_eq!(add(1, 2), 1);       
}