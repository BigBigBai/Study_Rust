fn main() {
    // let x = 2;
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);

    // let x = five();
    // println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
