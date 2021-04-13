use std::fmt::Display;

pub fn run() {
    run_summary();
}

fn run_summary() {
    let x = Tweet {
        text: String::from("text"),
        author: String::from("John Doe"),
    };

    // println!("{} {}", x.print_headline(), x.summarize());

    let y = NewsArticle {
        author: String::from("Jonny Dee"),
        content: String::from("This is a rust code"),
        headline: String::from("RUST!"),
        page: 10,
    };
    //  print!("{} {}", y.print_headline(), y.summarize());

    notify(&x);
    notify(&y);
}

pub trait Summary {
    fn summarize(&self) -> String {
        self.summarize_author()
    }

    fn print_headline(&self) -> String {
        String::from("Big news!!")
    }

    fn summarize_author(&self) -> String;
}

struct Tweet {
    author: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.text)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

struct NewsArticle {
    headline: String,
    content: String,
    author: String,
    page: u8,
}

impl Summary for NewsArticle {
    fn print_headline(&self) -> String {
        format!("Big news! {}", self.headline)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

pub fn notify3<T: Display + Summary>(item: &T) {}

pub fn notify4<T>(item: &T)
where
    T: Display + Summary,
{
}

fn return_summarizable() -> impl Summary {
    Tweet {
        author: String::from("fdsdf"),
        text: String::from("fdf"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn run_pair() {
    let pair = Pair::new(4, 8);
}

trait TraitNew {
    fn new(text: String) -> Self;

    fn get_text(&self) -> &str;
}

fn print_text(item: &impl TraitNew) {
    println!("{}", item.get_text());
}

struct Text {
    text: String,
}

impl TraitNew for Text {
    fn new(text: String) -> Self {
        Self { text }
    }
    fn get_text(&self) -> &str {
        &self.text
    }
}
