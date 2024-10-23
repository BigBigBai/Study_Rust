//! # _01_doc
//! 
//! doc是一系列工具的集合
//! 这些工具被用来简化特定的计算操作
//! 

/// 将传入的数字加 1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = _01_doc::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
