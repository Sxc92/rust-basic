fn main() {
    // another_function()
    function_demo()
}

fn another_function() {
    println!("another function case!");
}

fn another_function_parameter(x: i32) {
    println!("The value is :{}", x)
}

fn function_demo() {
    // 定义属性 🟰 右边需要是一个表达式，不能是一个语句例如一下两个案例
    let x = 5;
    // {} 里面就是表达式
    let y = {
        // 第一步
        let x = 4;
        // 第二步操作，不能加; 不然会变成语句会报错
        x + 3
    };
    println!("The value is :{}", y)
}

fn function_return(x: i32) -> i32 {
    x + 5
}
