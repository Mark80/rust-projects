// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
