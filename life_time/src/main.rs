use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ImportantExcerpt<'a> {
    part: &'a str,
}

struct RpcServer {}

struct TpcStream {}

enum RHState {
    Start,
    Complete,
    Running,
}

struct RequestHandler<'a> {
    server: &'a RpcServer,
    stream: TpcStream,
    id: i32,
    State: RHState,
}

// impl Future for RequestHandler {
//     type Output = ();
//
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         match self.State {
//             RHState::Start => self.server
//             RHState::Complete => {}
//             RHState::Running => {}
//         }
//     }
// }

fn main() {
    let a = String::from("aaaa");
    let b = "bb";

    let c = 6;
    let d = scope(&c, &3);

    // println!("{}", longest(a.as_str(),b))

    let i1: i32;
    let rrr: &i32;
    {
        i1 = 4;
        let i2 = 7;
        rrr = test2(&i1, &i2);
        println!("{}", rrr)
    }


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: "first_sentence",
    };
}

fn longest2<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result
}

// fn longest2(str1 : &str,str2 : &str) -> &str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn scope<'a>(i: &'a i32, j: &i32) -> &'a i32 {
    println!("{}", j);
    i
}


fn test2<'a, 'b : 'a>(l: &'a i32, r: &'b i32) -> &'a i32 {
    return if l > r
    {
        l
    } else {
        r
    };
}


