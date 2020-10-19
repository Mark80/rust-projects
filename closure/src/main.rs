use std::thread;
use std::time::Duration;

struct Cache<T>
    where
        T: Fn(u32) -> u32
{
    calculation: T,
    result: Option<u32>,
}

impl<T> Cache<T>
    where
        T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            result: None,
        }
    }

    fn value(&mut self, i: u32) -> u32 {
        if self.result.is_none() {
            let result = (self.calculation)(i);
            self.result = Some(i);
            i
        } else {
            self.result.unwrap()
        }
    }
}


fn main() {
    let simulated_user_specified_value = 50;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];
    let equal_to_x =  |z| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let variable = method(7);

    println!("{}",variable);

    let x = &Box::new(5);

    println!("{}", x)


}

fn stuff_box(b : Box<i32>) {
    println!("BOX")
}

fn method(a : u32) -> u32 {

    if a > 5 {
        a
    }
    else {
        8
    }

}

fn generate_workout(intensity: u32, random_number: u32) {

    let cache: Cache<fn(u32) -> u32> = Cache::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", (cache.calculation)(intensity)
        );
        println!(
            "Next, do {} situps!",
            (cache.calculation)(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",  (cache.calculation)(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_iterator() {
        let vector = vec![1,2,3,4,5,6];
        let iterator = vector.iter();

        let total : i32 = iterator.sum();

        assert_eq!(total,21)
    }

    #[test]
    fn test_iterator_map() {
        let vector = vec![1,2,3,4,5,6];
        let iterator = vector.iter();

        let total : i32 = iterator.map(|x| x+1 ).sum();

        assert_eq!(total,27)
    }

    #[test]
    fn test_iterator_map() {
        let vector = vec![1,2,3,4,5,6];
        let iterator = vector.iter();

        let total : i32 = iterator.filter(|x| x % 2 == 0 ).sum();

        assert_eq!(total,9)
    }


}
