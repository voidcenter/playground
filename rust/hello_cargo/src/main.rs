use std::collections::VecDeque;
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Progress {
    None,
    Some,
    Complete,
}


fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.values().filter(|&x| x == &value).count()
}

fn get_map() -> HashMap<String, Progress> {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("variables1"), Complete);
    map.insert(String::from("functions1"), Complete);
    map.insert(String::from("hashmap1"), Complete);
    map.insert(String::from("arc1"), Some);
    map.insert(String::from("as_ref_mut"), None);
    map.insert(String::from("from_str"), None);

    map
}


fn get_map_i() -> HashMap<String, i32> {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("variables1"), 1);
    map.insert(String::from("functions1"), 1);
    map.insert(String::from("hashmap1"), 1);
    map.insert(String::from("arc1"), 2);
    map.insert(String::from("as_ref_mut"), 3);
    map.insert(String::from("from_str"), 3);

    map
}



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn count_iterator_i(map: &HashMap<String, i32>, value: i32) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.values().filter(|&x| x == &value).count()
}


fn misc() {
    println!("Hello, world!");

    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter()
                             .map(|&x| x * 2)
                             .collect();
    
    let range = (0..10).collect::<Vec<i32>>();

    println!("{:?}", range);

    println!("{:?}", vec![5; 6].iter().product::<i32>());

    {   
        let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();
        println!("{:?}", doubled);
    }

    let map = get_map();
    assert_eq!(3, count_iterator(&map, Progress::Complete));

    println!("{:?}", map.values().collect::<Vec<_>>());

    print_type_of(&&map);
    print_type_of(&map.values());


    let map_i = get_map_i();
    assert_eq!(3, count_iterator_i(&map_i, 1));
    

}


fn tchannel() {

    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        tx.send("Hello, thread".to_owned())
            .expect("Unable to send on channel");
    });

    let receiver = thread::spawn(move || {
        let value = rx.recv().expect("Unable to receive from channel");
        println!("{value}");
    });

    sender.join().expect("The sender thread has panicked");
    receiver.join().expect("The receiver thread has panicked");

}


fn tmutex() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}


fn tcow() {
    // let slice = [-1, 0, 1];
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    let a = abs_all(&mut input);
    match abs_all(&mut input) {
        Cow::Owned(_) => { println!("owned!"); },
        _ => { println!("not owned"); },
    }
    // println!("{}", a.is_borrowed());
    // print_type_of(a);
}



fn clippy() {
    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(7, 5);
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, & mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {}; value b: {}", value_a, value_b);
    
}




#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}


use std::num::ParseIntError;

#[derive(Debug)]
enum ParseError {
    MissingField,
    ExtraFields,
    ParseIntError(ParseIntError)
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::ParseIntError(err)
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {

        let parsePerson = |s: &str| -> Result<Person, ParseError> {
            if s.len() == 0 {
                return Err::<Person, ParseError>(ParseError::MissingField);
            }
            let mut parts = s.split(',');
            let name = parts.next().ok_or(ParseError::MissingField)?.trim();
            if name.len() == 0 {
                return Err::<Person, ParseError>(ParseError::MissingField);
            }
            let ageStr = parts.next().ok_or(ParseError::MissingField)?.trim();
            let age = ageStr.parse::<usize>()?;
            if parts.next() != None {
                return Err::<Person, ParseError>(ParseError::ExtraFields);
            }
            Ok(Person { name: name.to_string(), age })
        };

        parsePerson(s).unwrap_or_else(|_| Default::default())
    }
}

fn parse() {
    // Use the `from` function
    let p1 = Person::from("j,1,j");
    // let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("test test");
}



// fn is_hello<T: AsRef<str>>(s: T) {
//     assert_eq!("hello", s.as_ref());
// }

fn is_hello<T: AsRef<str>>(s: T) {
assert_eq!("hello", s.as_ref());
}
 

fn as_ref_() {
    let s = "hello";
    is_hello(s);
    
    let s = "hello".to_string();
    is_hello(s);
}



fn main() {
    // tchannel();
    // tmutex();
    // tcow();
    // clippy();

    // parse();

    as_ref_();

}


