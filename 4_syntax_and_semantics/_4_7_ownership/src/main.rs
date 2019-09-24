fn main() {
    // ムーブセマンティクス
    let v = vec![1, 2, 3];
    let v2 = v;
    //println!("v is: {:?}", v);
    println!("v2 is: {:?}", v2);

    let v = vec![1, 2, 3];
    take(v);
    //println!("v is: {:?}", v);

    // Copy型
    let a = 5;
    let _y = double(a);
    println!("{}", a);

    let a = true;
    let _y = change_truth(a);
    println!("{}", a);

    // 所有権を超えて
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let (v1, v2, ret) = bar(v1, v2);
    println!("{:?}, {:?}, {}", v1, v2, ret);
}

fn take(v: Vec<i32>) {
    // ここで何が起きるかは重要ではない
    println!("v is: {:?}", v);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn change_truth(x: bool) -> bool {
    !x
}

fn bar(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // 所有権と関数の結果を返す
    (v1, v2, 42)
}