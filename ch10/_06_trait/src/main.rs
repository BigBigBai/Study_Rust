use _06_trait::Summary;
use _06_trait::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(tweet);
}

// fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
