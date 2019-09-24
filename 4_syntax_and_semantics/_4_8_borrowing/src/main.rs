fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1 ,2, 3];
    let ret = foo(&v1, &v2);
    println!("{:?}, {:?}, {}", v1, v2, ret);

    //let v = vec![];
    //bar(&v);

    // &mut参照
    let mut x = 5;
    {
        let y = &mut x; // -+ &mut借用がここから始まる
        *y += 1;        //  |
    }                   // -+ ... そしてここで終わる
    println!("{}", x);  // <- ここでxを借用しようとする

    // 借用が回避する問題
    // イテレータの無効
    let mut v = vec![1, 2, 3];
    v[0] += 100;

    for i in &v {
        println!{"{}", i};
        //v.push(34);
    }

    for i in &mut v {
        *i += 1;
        println!{"{}", i};
        //v.push(34);
    }

    // 解放後の使用
    //let y: &i32;
    //{
    //    let x = 5;
    //    y = &x;
    //}
    //println!("{}", y);

    let y: &i32;
    let x = 5;
    y = &x;
    println!("{}", y);
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    println!("{:?}, {:?}", v1, v2);
    // 結果を返す
    42
}

//fn bar(v: &Vec<i32>) {
//    v.push(5);
//}