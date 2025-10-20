fn flow_if() {
    let number = 6;
    if number > 2 {
        println!("number > 2")
    }
    if number != 0 {
        println!("number != 0")
    }
    if number % 5 == 0 {
        println!("number %5 == 0")
    }

    let number2 = if number > 2 { 1 } else { 0 };
    println!("number2={}", number2);
}

fn flow_loop() {
    // infinity loop
    // loop {
    //     println!("again!");
    // }

    // loop 返回值
    let mut count = 0;
    let result = loop {
        if count > 3 {
            break count * 2;
        }
        count += 1;
    };
    println!("count={}", result);

    // while
    while count != 0 {
        count -= 1;
    }

    // for
    let a = [1, 2, 3];
    for e in a {
        println!("{}", e);
    }

    for number in (1..4).rev() {
        println!("{}", number)
    }
}

pub fn play() {
    flow_if();
    flow_loop();
}
