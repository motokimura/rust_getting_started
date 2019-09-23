fn main() {
    // ブーリアン型
    let x = true;
    println!("{}", x);

    let y: bool = false;
    println!("{}", y);

    // char
    let x = 'x';
    let two_hearts = '💕';

    println!("{}", x);
    println!("{}", two_hearts);

    // 数値型
    let x = 42;
    let y = 1.0;

    println!("{}", x);
    println!("{}", y);

    // 配列
    let a = [1, 2, 3];
    println!("{:?}", a);

    let a = [0; 20];
    println!("{:?}", a);
    println!("{}", a.len());

    let names = ["Graydon", "Brain", "Niko"];
    println!("The second name is: {}", names[1]);

    // スライス
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..];
    let middle = &a[1..4];

    println!("{:?}", complete);
    println!("{:?}", middle);

    // タプル
    let x = (1, "hello");
    println!("{:?}", x);

    let x: (i32, &str) = (1, "hello");
    println!("{:?}", x);

    let mut x = (1, 2);
    println!("{:?}", x);
    let y = (2, 3);
    x = y;
    println!("{:?}", x);

    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);

    // タプルのインデックス
    let tuple = (1, 2, 3);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("tuple = ({}, {}, {})", x, y, z);

    // 関数
    let fx: fn(i32) -> i32 = foo;
    println!("{}", fx(100));
}

fn foo(x: i32) -> i32 {
    x
}