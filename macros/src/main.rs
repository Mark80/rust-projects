use std::collections::hash_map::RandomState;
use std::collections::HashMap;

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

macro_rules! java {

   (static void $name:ident($($_:tt)+) {$($body:tt)+} ) => {
       fn $name() {
          //java!($($body:tt)+);
         println($body);
       }
   };

   (System.$__:ident.$fn_name:ident($args:tt);) => {
      println!($args);
   };

}

macro_rules! items {


   ($item:item) => {
        println!("{}",stringify!($item));
    };


    ($item:item $($item2:item)*) => {
        println!("{}",stringify!($item));
        println!(">>>>>>>>>>>>");
        items!($($item2)+)
    };
}

use std::ops::Index;

macro_rules! recurrence {
    ( a[n]: $sty:ty = $($inits:expr),+ , ... , $recur:expr ) => {
        {
            /*
                What follows here is *literally* the code from before,
                cut and pasted into a new position. No other changes
                have been made.
            */

            use std::ops::Index;

            struct Recurrence {
                mem: [u64; 2],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [u64; 2],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = u64;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b u64 {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(2);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = u64;

                #[inline]
                fn next(&mut self) -> Option<u64> {
                    if self.pos < 2 {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let n = self.pos;
                            let a = IndexOffset { slice: &self.mem, offset: n };
                            (a[n-1] + a[n-2])
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..2).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [0, 1], pos: 0 }
        }
    };
}

fn main() {
    items! {
    struct Foo;
    enum Bar {
        Baz
    }
    impl Foo {}
}


    // x_and_y!(x => 4  + 5);
    //
    // make_fn!(ciccio);
    // ciccio();
    // print_ex!(5+7);
    //
    // print_ex!({
    //    let x = 5;
    //    let y = 7;
    //    let z= x + y;
    //    z
    // });
    //
    // let m: HashMap<&str, i32, RandomState> = new_map! {
    // "ciao" => 2
    // "eccomi" => 3
    // };
    //
    // println!("{:?}", m.get("ciao"));
}
