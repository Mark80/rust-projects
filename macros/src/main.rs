use std::collections::HashMap;
use std::collections::hash_map::RandomState;

/*
   expr rappresenta una espressione rust, di fatto qualsiasi porzione di codice valido
*/
macro_rules! x_and_y {
     (x => $e:expr) => (println!("X :{}",$e));
     (y => $e:expr) => (println!("Y :{}",$e));
}


/*
   ident sta di solito per un nome di una variabile o di una funzione
*/
macro_rules! make_fn {
  ($func_name:ident ) => (
     fn $func_name() {
        println!("Ecco la funzione {:?}", stringify!($func_name));
     }

  )
}

macro_rules! print_ex {
   ($e:expr) => (
      println!("{:?} = {:?}", stringify!($e), $e);

   )
}

macro_rules! new_map {
   ($($key : expr => $val:expr)*) => {
       {
           let mut map = HashMap::new();

           $(
           map.insert($key,$val);
           )*

       map
       }
   };

}

fn main() {
    x_and_y!(x => 4  + 5);

    make_fn!(ciccio);
    ciccio();
    print_ex!(5+7);

    print_ex!({
       let x = 5;
       let y = 7;
       let z= x + y;
       z
    });

    let m: HashMap<&str, i32, RandomState> = new_map! {
    "ciao" => 2
    "eccomi" => 3
    };

    println!("{:?}", m.get("ciao"));
}
