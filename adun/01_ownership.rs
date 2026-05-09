// 阿盾 Day1练习 #1: 所有权基础
// Rust语言基础 - 所有权系统
fn main() {
    println!("=== 阿盾 Rust学习 Day1 - 所有权 ===");
    
    // 所有权规则1: 每个值有唯一所有者
    let s1 = String::from("hello");
    let s2 = s1; // 所有权转移到s2
    // println!("{}", s1); // 编译错误！s1已无效
    println!("s2 = {}", s2);
    
    // 克隆（深拷贝）
    let s3 = String::from("world");
    let s4 = s3.clone(); // 克隆一份
    println!("s3 = {}, s4 = {}", s3, s4);
    
    // 整数是Copy类型
    let x = 5;
    let y = x; // 复制，不是移动
    println!("x = {}, y = {}", x, y);
    
    // 所有权规则2: 值离开作用域时被释放
    {
        let inner_var = String::from("inner");
        println!("在作用域内: {}", inner_var);
    } // inner_var在这里被释放
    // println!("{}", inner_var); // 编译错误！
    
    // 所有权规则3: 引用不获得所有权
    let s5 = String::from("reference");
    let len = calculate_length(&s5); // 借用s5
    println!("'{}'的长度是 {}", s5, len); // s5仍然有效
    
    println!("=== Day1练习1完成 ===");
}

// 使用引用作为参数
fn calculate_length(s: &String) -> usize {
    s.len()
}
