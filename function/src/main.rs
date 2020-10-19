fn main() {
    another_function(5);

    let six = {
        let five = 5;
        five + 1
    };


    let result: i32 = 7;
    if result > 6 {
        println!("{}", result);
    } else {
        println!("less ... {}", result);
    }

    println!("the sum is {}", sum(4, 6));

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    let mut count: i32 = 0;

    while count < 10 {
        println!("{}", count);
        count += 1;
    }

    println!("The result is {}", result);

    let numbers = [1,2,3,4,5,6];

    for num in numbers.iter() {
        println!("{}", num * num);
    }

    for num in 1..6 {
        println!("{}", num * num * num);
    }

    // {
    //     let s1: String = String::from("hello");
    //     let s2 = s1;
    //     println!("{}, world!", s1);
    // }

    let str = String::from("hola");

    borrow_me(str);

   // println!("{}",str);

}

fn borrow_me(str : String) {

    println!("{}",str);

}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
