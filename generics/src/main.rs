use generics::{Tweet, Summary};
use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl <T : Display> ToString for T {
    fn to_string(&self) -> String {
        String::from("eccomi")
    }
}


fn main() {
    let numbers = vec![1, 5, 3, 7, 5, 3, 6];
    println!("{:?}", largest(&numbers));

    let numbers = [1, 5, 3, 7, 5, 3, 6];
    println!("{:?}", largest(&numbers));

    let tweet = Tweet {
        author: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    println!("{}", notify(&tweet));
    println!("{}", notify2(&tweet));
    println!("{}", notify3(&tweet));
}

//queste tre forme sono equivalenti
fn notify(item: & impl Summary) -> String {
    item.summarize()
}

fn notify2<T : Summary>(item : &T)-> String {
    item.summarize()
}

fn notify3<T>(item : &T)-> String
  where T : Summary {
    item.summarize()
}


// rappresenta un slice di interi
fn largest<T : PartialOrd>(numbers: &[T]) -> &T {
    let mut max = &numbers[0];

    for number in numbers {
        if max < number {
            max = number;
        }
    }

    &max
}