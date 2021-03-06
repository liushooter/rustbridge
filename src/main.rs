use std::num::ParseIntError;
use std::string::String;

enum GameType {
    SinglePlayer,
    MultiPlayer(u32),
}

#[derive(PartialEq, Debug)] // necessary to compare enum values
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");

    let name = "Ashley";
    let age = 30;
    println!("Hi, {}! You are {} years old.", name, age);

    let mut apples = 100;
    apples += 50;
    println!("I have {} apples", apples);
    println!("Lots: {}", add_fifty(30));

    let height = 500u32;
    match height {
        0...150 => println!("You're too small to go on the rollercoaster."),
        150...200 => println!("You may go on the rollercoaster!"),
        _ => {
            println!("You're too tall to go on the rollercoaster.");
        }
    }

    // array
    let mut color = [255, 0, 255];
    color[0] = 100;
    // println!("The color is {}", color); // error
    println!("The color is {:?}", color); // debug
    println!("The color is {:#?}", color); // pretty debug

    // Vectors
    let mut prices = vec![30, 100, 2];
    prices[0] = 25;
    prices.push(40);
    println!("All the prices are: {:?}", prices);

    // looping
    for i in 1..10 {
        println!("Num {}", i);
    }

    let names = vec!["Carol", "Jake", "Marylou", "Bruce"];
    for name in names.iter() {
        println!("Hi {}!", name);
    }

    // Iterators
    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("i = {}", i);
    }

    for j in (0..10).map(|x| x * x) {
        println!("j = {}", j);
    }

    let sum = (0..10).fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);

    // Enums
    let light = TrafficLight::Green;
    match light {
        TrafficLight::Red => println!("STOP!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go go go!"),
    }

    let game = GameType::MultiPlayer(4);

    match game {
        GameType::SinglePlayer => println!("How about solitaire?"),
        GameType::MultiPlayer(2) => println!("How about checkers?"),
        GameType::MultiPlayer(4) => println!("How about bridge?"),
        GameType::MultiPlayer(num) => println!("How about {}-player tag?", num),
    }

    // Options
    let numstr = "6";
    let num = numstr.parse::<i32>();
    let num = num.expect("should have a number");
    println!("num + 5 = {}", num + 5);

    let numstr = "florp";
    let num = numstr.parse::<i32>();
    let answer = match num {
        Ok(n) => n + 5,
        Err(_) => 0,
    };

    println!("Answer is {}", answer);

    let aws = add_five_to_string("23".to_owned());
    // expected struct `std::string::String`, found reference

    println!("add_five_to_string Answer is {:?}", aws.unwrap());

}

fn add_fifty(n: i32) -> i32 {
    n + 50
}

fn add_five_to_string(s: String) -> Result<i32, std::num::ParseIntError> {
    let ans = s.parse::<i32>()? + 5;
    Ok(ans)
}

fn err() {
    // RUST_BACKTRACE=1 cargo run
    panic!("raise err")
}

#[test]
fn new_person_setup() {
    // cargo test
    let light = TrafficLight::Yellow;
    assert!(light == TrafficLight::Yellow);
    assert_eq!(light, TrafficLight::Yellow);
    assert_ne!(light, TrafficLight::Red);
}

#[test]
#[ignore]
fn not_implemented_yet() {
    assert_eq!(true, false);
}
