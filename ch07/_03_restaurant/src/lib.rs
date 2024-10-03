mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;   // 绝对路径
// use self::front_of_house::hosting; // 相对路径

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
