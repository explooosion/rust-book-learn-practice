fn main() {
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!"); // variable shadow
    {
        let v = "hello World!";
        assert_eq!(v, "hello World!");
        println!("inside: {:?}", v);
    }
    assert_eq!(v, "hello Rust!");
    println!("outside: {:?}", v);
}
