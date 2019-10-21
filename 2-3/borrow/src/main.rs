fn main() {
    let a = [1,2,3];
    let b = &a;
    println!("{:p}", b); // :p 印出指標位置, 即記憶體位置
    let mut c = vec![1,2,3];
    let d = &mut c;
    d.push(4);
    print!("{:?}", d);
    let e = &42;
    assert_eq!(42, *e);
}