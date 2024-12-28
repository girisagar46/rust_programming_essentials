pub fn run_demo() {
    demo_integers();
    demo_floats();
    demo_other_simple_types();
    demo_additional_techniques();
}

pub fn demo_integers() {

    // Signed integers are positive or negative whole numbers.
    // The type of signed integers is i8, i32, i64, i128.
    let a1: i32 = -12345;
    let a2: i32 = 0xFFFF;
    let a3: i32 = 0o777;
    let a4: i32 = 0b1010_1010;

    // Unsigned integers are positive whole numbers.
    let b1: u32 = 12345;

    let c: isize = 34545;
    println!("\n##Integers");
    println!("Numbers are {} {} {} {} {} {}", a1, a2, a3, a4, b1, c);
    println!("Numbers in reverse order are {5} {4} {3} {2} {1} {0}", c, b1, a4, a3, a2, a1);
    println!("isize is {} bytes", size_of::<isize>());
    println!("End Integers##\n");
}

pub fn demo_floats() {
    let a: f32 = 1.2345;
    let b: f64 = 1.23456789;
    println!("\n##Floats");
    println!("Floats are {} {}", a, b);
    println!("Floats upto 2dp {:.2}", a);
    println!("L-aligned {:<10.2}", a);
    println!("R-aligned {:>10.2}", a);
    println!("R-aligned with # filler {:#>10.2}", a);
    println!("f32 is {} bytes", size_of::<f32>());
    println!("f64 is {} bytes", size_of::<f64>());
    println!("End Floats##\n");
}

pub fn demo_other_simple_types() {
    let is_true: bool = true;
    println!("\n##Other Simple Types");
    println!("Boolean is {}", is_true);
    println!("Boolean is {} as number", true as isize);
    println!("true && true = {}", true && true);
    println!("true || true = {}", true || true);
    println!("!true = {}", !true);

    let emo: char = '😀';
    println!("Emoji char type is {}", emo);
    println!("End Other Simple Types##\n");
}

pub fn demo_additional_techniques() {
    println!("\n##Additional Techniques");
    let a = 20;  // immutable variable
    println!("a = {}", a);

    let mut b = 20;  // mutable variable
    b += 10;
    println!("mutable b = {}", b);

    let _pi = 3.14;   // unused variables which I might use later. Prefixing with _ to avoid warnings.

    let g = 9.8;
    let h = g as i32;  // Type casting;
    println!("Type casting {} to {}", g, h);

    let _num = "12345";
    let _num = 12345; // Can re-use variable name with different type.
    println!("Re-using variable name with different type {}", _num);

    const MAX_POINTS: u32 = 100_000;  // Constants. These are compile time constants.
    println!("MAX_POINTS = {}", MAX_POINTS);
    println!("End Additional Techniques##\n");
}