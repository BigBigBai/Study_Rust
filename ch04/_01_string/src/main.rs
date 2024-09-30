fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s);

    // 能过
    // let x = 5;
    // let y = x;

    // 不过, s1已经被废弃
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // 能过, 深拷贝
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 = {}, s2 = {}", s1, s2);

    // Ex4.3
    // let s = String::from("hello");
    // takes_ownership(s);
    // let x = 5;
    // makes_copy(x);
    // println!("{}", s);
    // println!("{}", x);

    // Ex4.4
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("{}", s1);
    // // println!("{}", s2);
    // println!("{}", s3);

    // Ex4.5
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // let s1 = "hello"; // &str类型, 放在栈中
    // calculate_length1(s1);
    // println!("{}", s1);

    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length2(&s1);
    // let s3 = s2;
    // println!("{}", s1);
    // println!("The lenth of '{}' is {}.", s2, len);
    // println!("{}", s3);

    // Ex4.6
    // let s = String::from("hello");
    // change(&s);

    // Ex4.7
    // let mut s = String::from("hello");
    // change1(&mut s);

    // 悬垂引用
    // let reference_to_nothing = no_dangle();
}

// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn change1(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn calculate_length2(s: &String) -> (&String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn calculate_length1(s: &str) -> (&str, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
