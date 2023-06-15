struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main1() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

// 可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 方法

impl Rectangle {
    // impl 块中的所有内容都将与 Rectangle 类型相关联
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
