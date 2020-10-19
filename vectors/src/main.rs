use std::collections::HashMap;
use std::collections::hash_map::RandomState;

fn main() {
    /*    let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        println!("The first element is: {}", first);*/

    let mut vector = vec![1, 2, 3, 4, 5, 6, 6];
    let third = vector[4];

    vector.push(10);
    println!("{}", third);

    for i in &vector {
        print!("{}", i);
    };

    let mut s = String::from("foo");
    let mut s2 = String::from("bar");

    s.push_str(&s2[..]);

    println!("{}", s);


    let string = s + "eccocmi";
    println!("{}", string);
    //println!("{}", s); this no compile the method + (add) take the ownership


    let hello = String::from("Здравствуйте");

    println!("{}", hello.len());

    let mut string = String::from("Hello world");

    let slice: &str = &string[0..5];

    println!("{}", slice);

    //string.clear();

    println!("{}", slice);

    let hello = "Здравствуйте";
    let s = &hello[0..2];

    println!("{}", s);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    let mut  score2 = HashMap::new();

    score2.insert(String::from("Blue"), String::from("Blue"));
    score2.insert(String::from("Yellow"), String::from("Blue"));
    let team_name = String::from("Blue");
    let value = score2.get(&team_name);
    let value2 = score2.get(&team_name);


    score2.entry(String::from("Blue")).or_insert(String::from("Blue2"));

    let i : &mut i32 = &mut 5;
    *i = *i + 1;

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        let pp = *count;
        *count += 1;
    }
    println!("{:?}", map);

}

fn first_word(string: &str) -> &str {
    let arr: &[u8] = string.as_bytes();

    for (i, &item) in arr.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }
    &string[..]
}
