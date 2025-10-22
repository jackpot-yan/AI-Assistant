use std::io;

pub fn get_command() {
    println!("请输入您需要的帮助");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("输入错误");
    println!("您输入的是: {}", input_string.trim());
}
