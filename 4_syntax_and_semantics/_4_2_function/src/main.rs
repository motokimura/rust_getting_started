fn main() {
    foo1();

    print_number(5);

    print_sum(12, 9);

    let x = add_one(10);
    print_number(x);

    // 式対文
    //let mut y = 5;
    //let x = (y = 6);
    //println!("x = {}", x);

    // 早期リターン
    let x = foo2(1);
    print_number(x);

    // 発散する関数
    //let x = diverges();

    // 関数ポインタ
    //let mut f: fn(i32) -> i32;
    // w/o tyoe inference
    let f: fn(i32) -> i32 = add_one;
    print_number(f(32));
    // w/ type inference
    let f = add_one;
    print_number(f(42));

    let six = f(5);
    print_number(six);
}

fn foo1() {
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn foo2(x: i32) -> i32 {
    return x + 1;

    // このコードは走らない
    //x + 1
}

//fn diverges() -> ! {
//    panic!("This function never returns!");
//}
