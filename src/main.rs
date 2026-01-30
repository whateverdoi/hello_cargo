fn main() {
    println!("Hello, world!");
    let c =2+3;

    another_function();
    let sum= jisuansum(5, 6);
    println!("The sum of 5 and 6 is: {sum}");
}

fn another_function() {
    println!("Another function.");
}
fn jisuansum(a: i32, b: i32) -> i32 {
    a + b
}