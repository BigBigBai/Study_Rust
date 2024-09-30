use std::io;
use std::io::Write;

fn main() {
    // Ex4.7
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // // s.clear();
    // println!("{}", word);

    // let s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("{}", hello);
    // println!("{}", world);

    // let mut s = String::from("hello world");
    // let word = first_word1(&s);
    // s.clear();
    // println!("{}", word);

    // Ex4.9
    // let my_string = String::from("hello world");
    // let word = first_word2(&my_string[..]);
    // let my_string_literal = "hello world";
    // let word = first_word2(my_string_literal);

    // let x: usize = 0;
    // let x = x - 1;
    // println!("{}", x);

    // let dir = [-1, 0];
    // check(&dir);

    // println!("Hello");
    // io::stdout().flush().expect("Failed to flush stdout.");
    // println!("World");

    let arr = "1234";
    let arr: Vec<_> = arr.chars().collect();
    println!("{}", arr.len());
}

// fn check(dir: &[i32]) {
// }

// fn first_word2(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn first_word1(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
