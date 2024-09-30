fn main() {
    println!("{}", fibo(0));
}

fn fibo(x: i32) -> i32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    }

    fibo(x - 1) + fibo(x - 2)
}
