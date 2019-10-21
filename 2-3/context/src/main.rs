pub fn temp() -> i32 {
    return 1;
}

fn main() {
    let _x = &temp();
    // temp() = *x; // error[E0070]: invalid left-hand side expression
    print!("{}",_x);
}
