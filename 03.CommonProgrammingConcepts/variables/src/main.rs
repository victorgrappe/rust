fn main() {
    // Constant
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS}");

    // Mutable variable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    {
        x = x * 2;
        println!("The value of x is {x}");
    }
    println!("The value of x is {x}");

    // Immutable variable
    let spaces_str = "   ";
    let spaces_num = spaces_str.len();
    println!("There are {spaces_num} spaces in [{spaces_str}]");

    // Shadowing immutable variable ("let" creates a new variable but with same name, more convenient ?)
    let spaces = "   ";
    let spaces = spaces.len();
    println!(
        "There are {spaces} spaces in [cannot reference the variable so I don't like shadowing]"
    );
    // spaces = spaces.len(); // Comilation error because not allowed to mutate the variable type
}
