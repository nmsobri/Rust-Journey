pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(_x: T, _y: T) -> Self {
        Self { x: _x, y: _y }
    }
}

impl<T: Summary> Pair<T> {
    fn info(&self) {
        println!("X Summary is {}", self.x.summary());
        println!("Y Summary is {}", self.y.summary());
    }
}

pub trait Summary {
    fn summary(&self) -> String {
        return format!("Read more from {}", self.summarize_author());
    }

    fn summarize_author(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
    retweet: bool,
}

impl Tweet {
    fn new(u: String, c: String) -> Self {
        return Tweet {
            username: u,
            content: c,
            retweet: false,
        };
    }

    fn get_username(&self) -> String {
        return self.username.clone();
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
}

fn foo<T>(bar: T) where T: Summary {
    println!("Breaking news: {}", bar.summary());
}

fn main() {
    let p: Pair<u32> = Pair::new(3, 5);
//    p.info();

    let q: Pair<Tweet> = Pair::new(
        Tweet::new("Sobri".to_string(), "This is content".to_string()),
        Tweet::new("Sobri".to_string(), "This is content".to_string()),
    );

    q.info();


//    let tweet = Tweet::new(String::from("Sobri"), String::from("Hello From Tweet"));
//    foo(tweet);
//    println!("Tweet summary is: {}", tweet.summary());
//    println!("Tweet username is: {}", tweet.get_username());
}
