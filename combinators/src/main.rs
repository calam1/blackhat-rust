use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    vector();
    hashmap();
    array();
    for_each();
    reduce();
}

fn vector() {
    let vec = vec![1, 2, 3, 4, 5];
    for x in vec.iter() { println!("{}", x)}
}

fn hashmap() { 
    let mut h = HashMap::new();
    h.insert(String::from("hello"), String::from("world")); 

    for (k, v) in h.iter() { println!("{} {}", k, v)}
}

fn array() { 
    let a = [1, 2, 3, 4, 5];
    for x in a.iter() { println!("{}", x)}
}

fn for_each() { 
    let vec = vec![11, 22, 33, 44, 55].into_iter();
    vec.for_each(|x| println!("{}", x));
}

fn reduce() { 
    let vec = vec![1, 2, 3, 4, 5].into_iter();
    let x = vec.reduce(|base, x| base + x);
    println!("value {}", x.unwrap());
}