fn main() {
    let out = 42;
    // fn add(i: i32, j: i32) -> i32 { i + j + out }
    fn add(i: i32, j: i32) -> i32 { i + j }
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out }; // 帶有型別附註
    let closure_inferred = |i, j| i + j + out; // 簡化
    let i = 1;
    let j = 2;
    println!("{:?}", add(i, j));
    println!("{:?}", closure_annotated(i, j));
    println!("{:?}", closure_inferred(i ,j));


    let product = |i,j| i * j * out;
    println!("{:?}", product(i, j));
}
