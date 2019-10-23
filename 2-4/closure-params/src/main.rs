fn math<F: Fn() -> i32>(op: fn) -> i32 {
    op()
}
fn main() {
    let a = 2;
    let b = 3;
    println!("{:?}", math(|| a + b));
    println!("{:?}", math(|| a * b));
}
