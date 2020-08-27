// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// Trait bounds:

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// ...is syntax sugar for:

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 'where' keyword:

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
}

// can be changed to:
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
}

fn main() {

}
