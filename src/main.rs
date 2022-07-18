fn main() {
    // constant value
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The constant value 'THREE_HOURS_IN_SECONDS' is: {}",
        THREE_HOURS_IN_SECONDS
    );

    // mutable values
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
