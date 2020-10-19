
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(side : u32) -> Rectangle{
        Rectangle{width :side,height:side}
    }
}

fn main() {

    let rectangle: Rectangle = Rectangle{width :7,height:2};
    let square: Rectangle = Rectangle::square(5);

    println!("The area of {:?} is {}",rectangle,rectangle.area());
    println!("The area of {:?} is {}",square,square.area());


}
