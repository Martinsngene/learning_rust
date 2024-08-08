fn main() {
    // Make variable mutable by adding `mut`
    let mut x: i32 = 5;

    println!("The value of x is: {}", x);
    // Variable is mutable so it can be updated
    x = 6;

    const MAX_POINTS: u32 = 100_000;

    println!("MAX_POINTS: {} and x: {}", MAX_POINTS, x);

    // Shadow the x variable
    let x = x + 1;

    let x = x * 2;
   
    println!("The value of x is: {}", x);

    // This works because of shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    // Doesn't work because of type difference in the two expressions.
    // Type inferred is a string
    // let mut space = "    ";

    // Type inferred is a number
    // space = space.len();
    // println!("Space: {}", space);
}
