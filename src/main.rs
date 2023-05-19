use std::io;

mod data_type;
mod guess_number;
mod slice;
mod struct_test;

fn main() {
    println!("Rust demo 正在运行...");

    loop {
        println!();
        println!("请选择要执行的程序:");
        println!("1. 猜数字");
        println!("2. 数据类型");
        println!("3. Slice");
        println!();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        println!();

        match number.trim() {
            "1" => guess_number::guess_number(),
            "2" => data_type::main(),
            "3" => slice::main(),
            _ => println!("没有该程序"),
        }
    }
}
