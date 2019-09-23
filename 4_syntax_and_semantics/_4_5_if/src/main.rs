fn main() {
    let x = 6;
    if x == 5 {
        println!("x is 5!");
    } else if x == 6 {
        println!("x is 6!");
    } else {
        println!("x is either 5 or 6!");
    }

    let x = 5;
    let y = if x == 5 { 30 } else { 100 };
    println!("x: {} and y: {}", x, y);
}
