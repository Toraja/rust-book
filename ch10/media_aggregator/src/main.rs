mod aggregator;
use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    notifiy(&tweet);

    let article = NewsArticle {
        headline: "Wealthiest People".to_string(),
        location: "https://news.com/articles/123".to_string(),
        author: "Bob Jones".to_string(),
        content: "1st: Peter, 2nd: Jacob, 3rd: Susan...".to_string(),
    };
    notifiy(&article);

    notifiy2(&tweet, &tweet);
    // this does not compile as the type of both of the arguments must be the same type.
    // notifiy2(&tweet, &article);
}

fn notifiy(item: &impl Summary) {
    println!("Braking news! {}", item.summarize());
}

fn notifiy2<T: Summary>(item1: &T, item2: &T) {
    println!("First news! {}", item1.summarize());
    println!("Second news! {}", item2.summarize());
}
