/*

Requirements
Deposit and Withdrawal
Transfer
Account statement (date, amount, balance)
Statement printing
Statement filters (just deposits, withdrawal, date)

The Rules
One level of indentation per method
Don’t use the ELSE keyword
Wrap all primitives and Strings
First class collections
One dot per line
Don’t abbreviate
Keep all entities small (50 lines)
No classes with more than two instance variables
No getters/setters/properties

*/

struct A {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
}

fn main() {
    println!("Hello, world!");

    println!("{}", std::mem::size_of::<A>());
    println!("{}", std::mem::align_of::<A>());

    let size = 240usize;
    let align = 22isize;

    let n = (size as f64 / align as f64).round() as isize;
    println!("{}", (n * align) as isize <= isize::MAX );
    println!("{}", n * align);
    println!("{:?}", size.checked_add(size % align as usize));
    println!("{:?}", size % align as usize);





}

enum Movement {
    Deposit(String, i32),
    Withdrawal(String, i32),
}

struct Balance {
    movements: Vec<Movement>
}

impl Balance {
    fn do_movement(&mut self, movement: Movement) {
        self.movements.push(movement);
    }

    fn get_deposit(&self) -> Vec<Movement> {
        todo!()
    }

    fn amount(&self) -> i32 {
        let mut balance = 0;
        for mov in &self.movements {
            match mov {
                Movement::Deposit(date, amount) =>
                    balance += amount
                ,
                Movement::Withdrawal(date, amount) =>
                    balance -= amount
            };
        }
        balance
    }

    fn new() -> Self {
        Balance {
            movements: vec![]
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_balance() {
        let balance = Balance::new();
        let amount = balance.amount();
        assert_eq!(amount, 0)
    }

    #[test]
    fn balance_with_a_deposits() {
        let mut balance = Balance::new();

        let deposit_1 = Movement::Deposit(
            "24/10/1980".to_string(),
            100,
        );

        let deposit_2 = Movement::Deposit(
            "24/10/1980".to_string(),
            200,
        );

        balance.do_movement(deposit_1);
        balance.do_movement(deposit_2);

        assert_eq!(balance.amount(), 300)
    }

    #[test]
    fn balance_with_a_deposits_and_a_withdrawal() {
        let mut balance = Balance::new();

        let deposit_1 = Movement::Deposit(
            "24/10/1980".to_string(),
            100,
        );

        let withdrawal_2 = Movement::Withdrawal(
            "24/10/1980".to_string(),
            70,
        );

        balance.do_movement(deposit_1);
        balance.do_movement(withdrawal_2);

        assert_eq!(balance.amount(), 30)
    }

    #[test]
    fn filter_deposit() {
        let mut balance = Balance::new();

        let deposit_1 = Movement::Deposit(
            "24/10/1980".to_string(),
            100,
        );

        let withdrawal_2 = Movement::Withdrawal(
            "24/10/1980".to_string(),
            70,
        );

        balance.do_movement(deposit_1);
        balance.do_movement(withdrawal_2);

        assert_eq!(balance.get_deposit().len(), 1)
    }
}
