fn main() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }
    // for i in &v {
    //     println!("{}", i);
    // }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String).
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
