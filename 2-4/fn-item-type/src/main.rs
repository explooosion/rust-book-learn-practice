fn is_true() -> bool { true }
fn true_maker() -> fn() -> bool { is_true }
// fn() -> bool 是函數宣告, true_maker 是傳函數指標
fn main() {
    println!("{:?}", true_maker()());
}
