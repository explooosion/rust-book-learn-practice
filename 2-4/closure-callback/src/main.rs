fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 32;
    move |j| j * i
}
fn main() {
    let result = two_times_impl();
    println!("{:?}", result(2));
}
