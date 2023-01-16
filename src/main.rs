use std::io;

mod guess_number;

fn main() {
    println!("Rust demo 正在运行...");

    loop {
        println!();
        println!("请选择要执行的程序:");
        println!("1. 猜数字");
        println!();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        println!();

        match number.trim() {
            "1" => guess_number::guess_number(),
            _ => println!("没有该程序"),
        }
    }
}
