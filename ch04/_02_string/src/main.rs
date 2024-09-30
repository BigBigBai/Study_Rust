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
    let my_string = String::from("hello world");
    let word = first_word2(&my_string[..]);
    let my_string_literal = "hello world";
    let word = first_word2(my_string_literal);
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

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
