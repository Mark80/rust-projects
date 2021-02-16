use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
pub struct Player {
    score : i32
}

impl Player{

    pub fn set_score(& mut self, score:i32) {
        self.score = score;
    }

}

fn main() {

    let mut vec = vec![1,2,3];

    let first = vec.first().unwrap();
    let last  = vec.last().unwrap();


    println!("{:?} {:?}", first,last);

    *vec.first_mut().unwrap() += 1;

    println!("{:?}", vec.first());


    let mut player = Player{score: 2};

    player.set_score(
        player.score +1
    );

    println!("{:?}", player);

    let text = "ciao ciao a tutti";

    let mut freq = HashMap::new();

    for word in text.split_whitespace() {
        match freq.get_mut(word) {
            Some(value) => *value += 1,
            None => {
                freq.insert(word,1);
            },
        }
    };

    println!("{:?}",freq);

    let  f = File::create("cicc").unwrap();

    f.drop();


}