fn main() {
    // let array: &[u32] = &[1, 2, 3, 4, 5];
    // let text: &str = "Hello";

    let tuples: (bool, u32, f64, i32) = (true, 38, 234.4, -6);
    // println!("{}", array[0]);
    // println!("{}", array[1]);
    // println!("{}", array[2]);
    // println!("{}", array[3]);
    // println!("{}", array[4]);
    println!("{}", tuples.0);
    println!("{}", tuples.1);
    println!("{}", tuples.2);
    println!("{}", tuples.3);
}
