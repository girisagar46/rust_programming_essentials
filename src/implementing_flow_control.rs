pub fn run_demo() {
    _demo_if();
    _demo_match();
    _demo_loops();
    _demo_break_and_continue()
}

pub fn _demo_if() {
    let age: i32  = 18;

    // Simple if condition
    if age >= 18 {
        println!("The value of age is 18");
    }

    // This is not possible to do:
    // if age {
    //     println!("The value of age is {}", 18);
    // }
    // ^^^ expected `bool`, found integer

    // Simple if--else-if condition
    let height: f64 = 1.7;
    if height > 1.8 {
        println!("You are tall.");
    } else if height < 1.8 {
        println!("You are not tall!");
    }

    // Simple if--else-if--else condition
    let game_score = 500;
    if game_score > 300 {
        println!("You are good player");
    } else if game_score < 300 {
        println!("You are not good player");
    } else {
        println!("You are about to become a good player");
    }

    let msg = if age > 50 { "old" } else { "young" };
    println!("You are {}", msg);
}

pub fn _demo_match() {
    // match keyword is like a switch statement.

    let mut num: i32 = 100;
    match num {
        100 => println!("one hundred!"), // These are called `arms`
        200 => println!("two hundreds!"),
        _ => println!("anything"), // the default condition
    }

    // Matching ranges
    match num {
        25..=50 => println!("Between 25 & 50"), // Here upper limit is inclusive
        51..=100 => println!("Between 51 & 100"),
        _ => println!("anything"),
    }

    // matching multiple patterns
    match num {
        25 | 50 | 75 => println!("You have 25, 50 or 75 !"),
        100 | 200 => println!("You have 100, 200!"),
        _ => println!("Anything"),
    }

    // Can have if statements in each arm
    num = 23;
    match num {
        // These conditions are called match guards
        x if x < 50 => println!("The x is {}", x), // Here num is copied to x if condition matches
        x if x == 75 => println!("You have 75 !"),
        _ => println!("You have 95 !"),
    }

    // we can use match as an expression
    num = 100;
    let res = match num {
        x if x < 50 => "x is less than 50", // Here num is copied to x if condition matches
        x if x == 75 => "You have 75 !",
        _ => "You have something else!",
    };
    println!("Value of res of match expression: {}", res);

    num = 50;
    match num {
        x @ 20..=50 => println!("The value is {}", x),  // @ also assigns the value to x
        _ => println!("Anything"),
    }
}


pub fn _demo_loops() {

    // loop {
    //     println!("This loop will run forever!");
    // }

    // count needs to be mutable because we're increasing it inside while loop
    let mut count: i32 = 0;
    while count < 10 {
        println!("Count: {}", count);
        count += 1;
    }

    let arr: [i32;5] = [1, 2, 3, 4, 5];
    for elem in arr {
        println!("Array element: {}", elem);
    }

    // upper limit is exclusive
    for i in 0..5 {
        println!("{} ", i);
    }

    // upper limit is inclusive
    for i in 0..=5 {
        println!("{} ", i);
    }
}

pub fn _demo_break_and_continue() {
    let mut counter = 0;
    loop {
        println!("counter: {}", counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let arr = [1, 2, 3, 4, 5];
    for elem in arr {
        if elem == 3 {
            println!("Found 3. Search is complete.");
            break;
        }
        println!("{}", elem);
    }

    for elem in arr {
        if elem == 4 {
            println!("Found 4. Do not print it. Skiping.");
            continue;
        }
        println!("{}", elem);
    }

    'outer: loop {  // Here 'outer is a name given to the loop. It starts with '
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer;
        }
    }
    println!("End the outer loop");
}