// 阿盾 Day1练习 #2: 借用(Borrowing)
// Rust语言基础 - 借用检查器
fn main() {
    println!("=== 阿盾 Rust学习 Day1 - 借用 ===");
    
    // 不可变借用
    let s1 = String::from("hello");
    let r1 = &s1; // 不可变引用
    let r2 = &s1; // 可以有多个不可变引用
    println!("s1='{}', r1='{}', r2='{}'", s1, r1, r2);
    
    // 可变借用
    let mut s2 = String::from("可变");
    let r3 = &mut s2; // 可变引用
    r3.push_str("字符串");
    println!("r3 = {}", r3);
    
    // 可变借用的限制：同一时间只能有一个可变引用
    let mut s3 = String::from("限制");
    let r4 = &mut s3;
    r4.push_str("测试");
    // let r5 = &mut s3; // 编译错误！不能同时有两个可变引用
    println!("r4 = {}", r4);
    
    // 不可变和可变不能共存
    let mut s4 = String::from("混合");
    let r6 = &s4; // 不可变引用
    // r6.push_str("测试"); // 编译错误！r6是不可变的
    println!("r6 = {}", r6);
    
    // r6使用完毕后，才能创建可变引用
    let r7 = &mut s4;
    r7.push_str("完成");
    println!("r7 = {}", r7);
    
    // 悬垂引用示例
    let reference = dangle();
    println!("返回的字符串: {}", reference);
    
    println!("=== Day1练习2完成 ===");
}

// 尝试返回悬垂引用 - 编译失败
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回s的引用，但s会被销毁
// }

fn correct_dangle() -> String {
    let s = String::from("hello");
    s // 返回所有权的String
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
