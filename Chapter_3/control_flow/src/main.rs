fn main() {
    let number: i32 = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // Cannot do this: 
    // if number {
    // because if needs bools

    // So you can just check that the number isnt zero if that works for you
    if number != 0 {
        println!("Number was something other than zero");
    }


    // else ifs
    let number = 4;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisibly by 4, 3 or 2");
    }

    // Using if to define a let because it's an expression 
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // using each of Rusts looping keywords

    // loop will run forever until you tell it not to. It's a shorter while True
    loop {
        println!("Will loop forever until I get told to stop.");
        // tell it to stop
        break;
    }

    // Breaking out of an outer loop using a label. cool af.
    let mut count = 0;
    // label the outer loop as 'counting_up
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        // Loop forever until a break or something happens
        loop {
            println!("remining = {}", remaining);
            if remaining == 0 {
                // break the inner loop
                break;
            }
            if count == 2 {
                // break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    let mut counter = 0;

    // Loop and return a result from the loop with a break and store that in result
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    // standard while loop
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // use for to iterate over an array safely and do it in reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOVER!!!");
}