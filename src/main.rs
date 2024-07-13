fn main() {
    // Arrays
    // let array: &[u32] = &[1, 2, 3, 4, 5];
    // println!("{}", array[0]);
    // println!("{}", array[1]);
    // println!("{}", array[2]);
    // println!("{}", array[3]);
    // println!("{}", array[4]);

    // Tuples, Tuple Structs and Type Alias
    // struct MyTuple(bool, u32, f64, i32);
    // type TypeTuple = (bool, u32, f64, i32);
    // let tuples: TypeTuple = (true, 38, 234.4, -6);
    // println!("{}", tuples.0);
    // println!("{}", tuples.1);
    // println!("{}", tuples.2);
    // println!("{}", tuples.3);

    // Structs
    // struct MyStruct {
    //     username: String,
    //     age: u32,
    //     height: f32,
    //     is_single: bool,
    // }

    // let person = MyStruct {
    //     username: String::from("johndoe1"),
    //     age: 32,
    //     height: 6.4,
    //     is_single: false,
    // };

    // println!("{}", person.username);
    // println!("{}", person.age);
    // println!("{}", person.height);
    // println!("{}", person.is_single);

    // Enums
    // enum CardinalPoints {
    //     North,
    //     East,
    //     South,
    //     West,
    // }

    // let a = CardinalPoints::East;
    // let b = CardinalPoints::North;
    // let c = CardinalPoints::West;
    // let d = CardinalPoints::South;

    // Practicing Conditional Statements
    // if let CardinalPoints::East = a {
    //     println!("Hello East")
    // }
    // if let CardinalPoints::West = c {
    //     println!("Hello West")
    // }
    // if let CardinalPoints::North = b {
    //     println!("Hello North")
    // }
    // if let CardinalPoints::South = d {
    //     println!("Hello South")
    // } else {
    //     println!("End of Statement")
    // }

    // Tagged Unions
    // enum Shape {
    //     Square { sides: f64 },
    //     Rectangle { width: f64, height: f64 },
    //     Circle { radius: f64 },
    // }

}
