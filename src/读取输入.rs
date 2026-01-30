// 导入标准库的输入输出模块
use std::io;

fn main() {
    // 定义一个包含5个整数元素的数组
    let a = [1, 2, 3, 4, 5];

    // 提示用户输入数组索引
    println!("Please enter an array index.");

    // 创建一个可变字符串变量来存储用户输入
    let mut index = String::new();

    // 从标准输入读取用户输入的一行
    // read_line()方法将输入内容追加到index变量中
    // expect()用于处理可能的错误，如果读取失败则会崩溃并显示指定错误信息
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // 将字符串类型的索引转换为usize类型（Rust中数组索引的类型）
    // trim()方法去除字符串两端的空白字符（包括换行符）
    // parse()尝试将字符串解析为指定类型
    // expect()用于处理解析失败的情况
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // 根据索引访问数组元素
    let element = a[index];

    // 打印指定索引处的元素值
    println!("The value of the element at index {index} is: {element}");
}
