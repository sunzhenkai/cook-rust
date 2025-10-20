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

pub fn other_types() {
    // boolean
    let b = true;
    // char
    let c = 'a';
    // tuple
    let t: (bool, char, i32) = (b, c, 100);
    let (d, e, f) = t;
    // array
    let aa = [1, 2, 3];
    let ab: [i32; 3] = [1, 2, 3];

    println!(
        "b={}, c={}, tuple=({}, {}, {}) tuple=({}, {}, {}), arr[0]={}, arr[0]={}",
        b, c, d, e, f, t.0, t.1, t.2, aa[0], ab[0]
    );
}

pub fn play() {
    def_vars();
    number_integer_types();
    number_float_types();
    other_types();
}
