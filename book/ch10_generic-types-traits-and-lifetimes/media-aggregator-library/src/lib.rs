#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        assert_eq!(2 + 2, 4);
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        // String::from("(Read more...)") // default
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /*
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    */
    fn summarize_author(&self) -> String {
        format!("{} ({})", self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /*
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    */

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from(""),
        content: String::from(""),
        reply: false,
        retweet: false,
    }
}

#[allow(dead_code)]
fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(""),
            location: String::from(""),
            author: String::from(""),
            content: String::from(""),
        }
    } else {
        Tweet {
            username: String::from(""),
            content: String::from(""),
            reply: false,
            retweet: false,
        }
    }
}
