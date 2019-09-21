fn main() {
    // 変数束縛
    let x = 5;
    println!("x = {}", x);

    // パターン
    let (x, y) = (1, 2);
    println!("(x, y) = ({} {})", x, y);

    // 型アノテーション
    let x: i32 = 5;
    println!("x = {}", x);

    // 可変性
    //let x = 5;
    let mut x = 5;
    println!("x = {}", x);

    x = 10;
    println!("x = {}", x);

    // 束縛を初期化する
    let x: i32;
    x = 100;

    println!("Hello, world!");
    println!("x = {}", x);

    // スコープとシャドーイング
    let a: i32 = 17;
    {
        let b: i32 = 3;
        println!("The value of a is {} and value of b is {}", a, b);
    }
    //println!("The value of a is {} and value of b is {}", a, b);

    let x: i32 = 8;
    {
        println!("{}", x); // 8
        let x = 12;
        println!("{}", x) // 12
    }
    println!("{}", x); // 8
    let x = 42;
    println!("{}", x); // 42

    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    let x = x;
    println!("{}", x);

    let y = 4;
    println!("{}", y);
    let y = "I can also be bound to text!";
    println!("{}", y);
}
