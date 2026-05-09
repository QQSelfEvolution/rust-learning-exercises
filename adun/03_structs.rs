// 阿盾 Day1练习 #3: 结构体
// Rust语言基础 - 结构体定义与方法
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

impl Person {
    // 关联函数 - 构造函数
    fn new(name: &str, age: u8, email: &str) -> Person {
        Person {
            name: String::from(name),
            age,
            email: String::from(email),
        }
    }
    
    // 方法
    fn greet(&self) -> String {
        format!("你好，我是{}，今年{}岁", self.name, self.age)
    }
    
    fn set_age(&mut self, new_age: u8) {
        self.age = new_age;
    }
    
    // 静态方法
    fn create_student(name: &str, age: u8) -> Person {
        Person {
            name: String::from(name),
            age,
            email: format!("{}@school.edu", name),
        }
    }
}

// 带生命周期的结构体
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("公告: {}", announcement);
        self.part
    }
}

fn main() {
    println!("=== 阿盾 Rust学习 Day1 - 结构体 ===");
    
    // 创建Person实例
    let person1 = Person::new("张三", 25, "zhangsan@example.com");
    println!("{:?}", person1);
    println!("{}", person1.greet());
    
    // 创建学生
    let student = Person::create_student("李四", 18);
    println!("{:?}", student);
    println!("{}", student.greet());
    
    // 修改可变实例
    let mut person2 = Person::new("王五", 30, "wangwu@example.com");
    println!("修改前: {}", person2.greet());
    person2.set_age(31);
    println!("修改后: {}", person2.greet());
    
    // 生命周期结构体
    let novel = String::from("白日依山尽，黄河入海流。欲穷千里目，更上一层楼。");
    let first_sentence = novel.split('。').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("{:?}", excerpt);
    println!("level: {}", excerpt.level());
    
    let returned = excerpt.announce_and_return("学习Rust!");
    println!("返回: {}", returned);
    
    println!("=== Day1练习3完成 ===");
}
