mod cli;
mod config;

fn main() {
    // 加载配置文件
    let conf = config::Config::new().unwrap_or_else(|e| {
        eprintln!("加载失败: {}", e);
        std::process::exit(1);
    });
    println!("加载配置成功: {:#?}", conf);
}
