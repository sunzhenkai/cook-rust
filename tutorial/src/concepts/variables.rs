// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
pub fn def_vars() {
    // let
    let x = 5;
    // let mut
    let mut y = 10;
    println!("y={}", y);
    y = x;
    // const
    const ZA: i32 = 15;
    // const ZE: i32 = x; // ERROR: attempt to use a non-constant value in a constant

    println!("y={}, z={}", y, ZA);

    // let 定义不可变变量，可在运行初始化
    // let mut 定义可变变量
    // const 定义编译器初始化变量，变量名大写
}
