pub trait Summary<T> {
    fn summarize(&self, other : T) -> T;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl<T> Summary<T> for NewsArticle {
    fn summarize(&self, other : T) -> T {
        other
    }
    
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl<T> Summary<T> for Tweet {
    fn summarize(&self, other : T) -> T {
        other
    }
}
