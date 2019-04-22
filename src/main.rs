fn main() {
    println!("Hello, world!");

    let name = "Ashley";
    let age = 30;
    println!("Hi, {}! You are {} years old.", name, age);

    let mut apples = 100;
    apples += 50;
    println!("I have {} apples", apples);
    println!("Lots: {}", add_fifty(100));


    let height = 167u32;
    if height < 150 {
        println!("You're too small to go on the rollercoaster.");
    } else if height < 200 {
        println!("You may go on the rollercoaster!");
    } else {
        println!("You're too tall to go on the rollercoaster.");
    }

}


fn add_fifty(n: i32) -> i32 {
    n + 50
}
