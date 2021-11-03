use std::borrow::Cow;
use std::env;

fn main() {
    println!("{:?}", env::args().nth(1));

    let fizzbuzz: &str = "FizzBuzz";
    let fizz: &str = fizzbuzz.get(0..4).unwrap();
    let buzz: &str = fizzbuzz.get(4..).unwrap();

    (1..100)
        .map(|i| match (i, i % 3 == 0, i % 5 == 0) {
            (_, true, true) => Cow::from(fizzbuzz),
            (_, true, false) => Cow::from(fizz),
            (_, false, true) => Cow::from(buzz),
            (i, false, false) => Cow::from(i.to_string()),
        })
        .for_each(|i| println!("{}", i));
}
