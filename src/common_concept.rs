// use core::prelude;

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

    // numeric data type and operations
    let _sum = 5 + 10;
    let _difference = 95.4 - 4.7;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.3;
    let _floored = 2 / 3;
    let _remainder = 43 % 5;

    //  boolean type
    let _t = true;
    let _f = false;

    // character type
    let _c = 'z';
    let _heart_eyed_cat = '🎃';

    // tuple type
    let tup: (u32, f64, char) = (500, 3.6, '🍾');
    let (_x, _y, z) = tup;
    println!("{z}");

    // array type
    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let a = [3; 5];
    println!("Array a is: {:?}", a);

    // statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    another_function();

    // multiple loops
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
      println!("the value is: {}", a[index]);

      index += 1;
    }

    // for loop
    for element in a {
      println!("the value is: {}", element);
    }
}

// function
fn another_function() {
    println!("Another function.");
}
