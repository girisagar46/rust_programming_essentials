mod mytypes;

// With `use` we can use `Color` in the code.
use mytypes::{Color, HouseLocation};

pub fn run_demo() {
    _demo_simple_enums();
    _demo_enum_with_data();
    _demo_using_option_enum();
    _demo_using_result_enum();
}

pub fn _demo_simple_enums() {
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;
    fn colors(color: Color) {
        match color {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
            Color::Unknown => println!("Color is unknown at the moment."),
        }
    }

    colors(red);
    colors(green);
    colors(blue);
}

pub fn _demo_enum_with_data() {
    let h1: HouseLocation = HouseLocation::Number(32);
    let h2: HouseLocation = HouseLocation::Name(String::from("Test"));
    let h3: HouseLocation = HouseLocation::Unknown;

    fn houses(house_location: HouseLocation) {
        match house_location {
            HouseLocation::Number(n) => println!("house number is {}", n),
            HouseLocation::Name(s) => println!("house name is {}", s),
            HouseLocation::Unknown => println!("house location is unknown at the moment."),
        }
    }
    houses(h1);
    houses(h2);
    houses(h3);
    let size: usize = std::mem::size_of::<HouseLocation>(); // Size of house location in bytes
    println!("size of HouseLocation in bytes: {}", size);
}

pub fn _demo_using_option_enum() {
    /*
       Docs: https://doc.rust-lang.org/std/option/
    */
    fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> {
        // Option is from `std::option::Option`
        if h <= 23 && m <= 59 && s <= 59 {
            let secs = h * 3600 + m * 60 + s;
            Some(secs)
        } else {
            None
        }
    }

    let sec1: Option<u32> = sec_of_day(23, 59, 59);
    println!("Unwrapped sec: {}", sec1.unwrap_or(0));

    let sec2: Option<u32> = sec_of_day(25, 15, 59);
    println!("Unwrapped sec: {}", sec2.unwrap_or(0))
}

pub fn _demo_using_result_enum() {
    /*
       Docs: https://doc.rust-lang.org/std/result/enum.Result.html
    */
    fn division(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
        if divisor == 0.0 {
            Err("division by zero is not possible.")
        } else {
            Ok(dividend / divisor)
        }
    }

    println!("division: {:?}", division(15.0, 10.0));
    println!("division: {:?}", division(15.0, 0.0));

    let res: Result<i32, std::num::ParseIntError>;
    res = i32::from_str_radix("111", 2); // Parse the binary string
    match res {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("error is: {}", e),
    }

    // res2 raises an error,so we use `unwrap_or` to show default value
    let res2 = i32::from_str_radix("Hello", 10);
    println!("{:?}", res2.unwrap_or(-1));
}
