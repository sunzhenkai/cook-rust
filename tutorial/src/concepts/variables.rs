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

/*
 * i8~128, u8~128
 * isize
 * */
pub fn number_integer_types() {
    let a: i8 = 1;
    let b: isize = 2;

    println!("a={}, b={}", a, b)
}

// f32, f64
pub fn number_float_types() {
    let a = 2.0; // default f64
    let b: f32 = 3.0;
    let c: f64 = 4.0;

    println!("a={}, b={}, c={}", a, b, c)
}

pub fn play() {
    def_vars();
    number_integer_types();
    number_float_types();
}
