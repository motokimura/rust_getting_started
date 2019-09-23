fn main() {
    // loop
    //loop {
    //    println!("Loop forever!");
    //}

    // while
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    // for
    for x in 0..10 {
        println!("{}", x);
    }

    // 列挙
    for (i, x) in (5..10).enumerate() {
        println!("{}, {}", i, x);
    }

    // 反復の早期終了
    let mut x = 5;
    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 { break; }
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }

    // ループラベル
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }
}
