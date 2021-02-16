use std::ops::Add;

/*
  l' ownership si applica agli oggetti immagazzinati nella heap non quelle
  nello stack (ad esempio gli interi) in quel caso viene copiatob il valore dell'
  oggetto
  l' ownership è legato allo scope della variabile, questo significa che
  all'interno dello scope dove la variabile è definita posso usarla,
  nel caso la ceda ad una altro scope se proverò a richiamarla dopo nello scope
  originale mi verrà ritornato un errore

  esempio:

  let a = String::from("ciao");
  calling_funtion(a);
  print(a) //ERROR

  mentre

  let five = 5;
  calling_funtion(five);
  print(a) //OK

  Per riprendere l'ownership di un oggetto bisogna solo prestarla
  in questo caso il metodo calling_funtion prenderà un riferimento all'oggetto
  e non l'oggetto come parmatro

  let a = String::from("ciao");
  calling_funtion(&a);
  print(a) //OK

  questo si chiama borrowing

  posso anche passare un riferimento mutabile

  let mut a = String::from("ciao");
  calling_funtion(&a);

  ma non posso passare un riferimento mutabile due volte


*/

fn main() {

    let mut s = String::from("hello");
    let r1 =  s;
    let r2 = r1;

    println!("{}",r2);

    let mut a = String::from("ciao");

    let  c = &mut a;
    let  d = &mut a;

    let mut b = takes_give_ownership(&mut a);


    println!("{}", a);


    let s = String::from("eccomi");

    takes_ownership(s);
    /*    questo non funziona perchè quando passo
          l'oggetto cedo anche la ownership su
          quell'oggetto.
          nel caso volessi solo darla in prestito (borrowing)
          devo modificare il parametro della funzione
          in modo che accetti il rifermento (& s)
    */
    //println!("{}",s);
    let number = 5;
    make_copy(number);

    println!("{}", number);

    let s2 = String::from("eccomi2");

    reference(&s2);

    let mut s_mut = String::from("eccomi3");

    println!("{}", s_mut);

    change(&mut s_mut);

    println!("{}", s_mut);


    let s_tmp = s2;

    let s3 = return_string(s_tmp);

    println!("{}", s3);

    let text = String::from("Hello world");

    let f = &text[0..5];

    let first = first_word(&text);

    first_word("ciao mondo");

    println!("{}", first);

    let array = vec![1, 2, 3, 4, 5, 8, 9, 1, 11, 12];

    let array2 = Vec::from([1, 2, 3, 4, 6, 7]);
    let value = longest_sequence(array);

    println!("{}", value);

    let v = "ciccio";

    first_(v);
}

fn first_(s : &str) {
    println!("{}",s);
    second(s);
}

fn second(s : &str) {
   println!("{}",s);
}

fn longest_sequence(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut max = 1;
    let mut current_max = 1;

    for i in 1..(arr.len()) {
        if arr[i] == arr[i - 1] + 1 {
            current_max += 1;
            if current_max > max {
                max = current_max;
            }
        } else {
            current_max = 1;
        }
    }
    max
}

fn takes_give_ownership(str: & String) -> &String {
    println!("{}", str);
    str
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn reference(str: &String) {
    println!(" 1 {}", str);
    println!(" 2 {}", &str);
}

fn return_string(str: String) -> String {
    str.add("_eccomi")
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn make_copy(i: i32) {
    println!("{}", i);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[0..bytes.len()]
}


