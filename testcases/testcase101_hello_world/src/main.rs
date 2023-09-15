fn main() {
    println!("Nice to meet you");

    let mut name = String::new();

    println!("What's your name?");
    std::io::stdin().read_line(&mut name)
        .expect("Can not see your name");

    println!("Hello, {}!", name);
}
