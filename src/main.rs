mod variable_types;
mod implementing_flow_control;

fn main() {
    println!("Hello, world!");

    println!("\nVariable Types====");
    variable_types::run_demo();
    println!("\n====End Variable Types");


    println!("\nImplementing flow control====");
    implementing_flow_control::run_demo();
    println!("\n====Implementing flow control");
}
