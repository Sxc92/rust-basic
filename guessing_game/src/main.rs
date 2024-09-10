use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜数游戏！!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    // println!("神秘数字是:{}", secret_number);
    loop {
        println!("猜测一个数字");
        // 声明一个可变变量
        let mut guess = String::new();
        // 引用io 包下stdin 函数
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // 数据类型转换 shadow 隐藏 机制  允许相同变量名
        // 以下是正常写法 但是不优雅
        // let guess: u32 = guess.trim().parse().expect("请输入一个数字");
        // 使用match 关键字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            // 下面的是Rust的枚举
            Ordering::Less => {
                println!("小了!");
            }
            Ordering::Equal => {
                println!("猜对了!");
                // 跳出循环
                break
            }
            Ordering::Greater => {
                println!("大了!");
            }
        }
    }
}
