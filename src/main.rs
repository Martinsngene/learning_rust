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

    // Referential Types
    // struct ConfigImm {
    //     port: u16,
    // }

    // let config1: ConfigImm = ConfigImm { port: 8080 };

    // let config_reference: &ConfigImm = &config1;

    // println!("Using Port: {}", config_reference.port);

    // Immutable References
    // One value can be referenced immutably multiple times.
    // let val: i32 = 10;
    // let val1: &i32 = &val;
    // let val2: &i32 = &val;

    // println!("{val1} and {val2} should have the same value");

    // Mutable References
    // struct ConfigMut {
    //     port: u16,
    // }

    // let mut config2: ConfigMut = ConfigMut { port: 3000 };

    // Before mutable reference
    // println!("Using Port: {} before mutable reference", config2.port);

    // Create a mutatble reference
    // let config_mut_reference: &mut ConfigMut = &mut config2;

    // Make use of the reference, let's say in some other part of the program
    // config_mut_reference.port = 4000;

    // After mutable reference
    // println!("Using Port: {} after mutable refernce", config2.port);

    // You can only have one mutable reference to a value simultaneously
    // The code below will throw an error if you uncomment mut_val2
    // let mut mut_val: i32 = 13;
    // let mut_val1: &mut i32 = &mut mut_val;
    // let mut_val2: &mut i32 = &mut mut_val;

    // Before changing value
    // println!("{}", mut_val1);

    // Assign 19 to change the value from the referenced variable
    // *mut_val1 = 19;

    // Assign 19 to change the value from the referenced variable
    //  *mut_val2 = 16;

    // After changing value
    // println!("{}", mut_val1);
    // println!("{}", mut_val2);

    // Dereferencing Mutable References
    // asterics (*) is the dereferencing operator.
    // In the code below, the type i32 is copyable. Not all types are copyable. E.g String is not
    // A copyable type is one from which we can create a new value simply by copying all the types.

    // i32 is copyables
    // let val: i32 = 11;
    // let ref_val: &i32 = &val;

    // Before Dereferencing
    // println!("{}", ref_val);

    // let _val2: i32 = *ref_val;

    // After Dereferencing
    // println!("{}", ref_val);

    // String is not copyable because it contains a pointer to some memory in a heap.
    // let some_string: String = "Hello".to_string();

    // let copy_string: &String = &some_string;

    // Throws an error
    // let ref_string: String = *copy_string;

    // The fix
    // let ref_string: String = copy_string.clone();

    // println!("{}", ref_string)

    // Lifetimes defines what part of your code a referential type can be used safely.
    // This is usually infered for us by the compiler automatically but we can also specify it ourselves.
    // &'lifetime for immutable references and &'lifetime mut for mutable references.

    // Practice Exercise
    // let mut n: i32 = 99;
    // let mut _r: &mut i32 = &mut n;
    // *_r += 1;
    // println!("{n}");

    // Variables And Mutability: The 'let' keyword is very powerful.
    // Variable are immutable by default in rust. to declare a mutable variable, use the 'mut' keyword.
    // Variable Declaration And Initialization
    // let _x: i16 = 2; // Type Ascription: Inform the compiler on the type you want the variable to have.
    // println!("{}", _x);

    // Mutable variable
    // let mut _y: i16 = 4;
    // println!("Value before reassign: {_y}");
    // Reassign a value to _y
    // _y = 5;
    // println!("Value after reassign: {_y}");
    // Scope: I rustlike in other languages, global variables are globally accessible.
    // Variables in a scope can only be accessed in that scope.

    // New Scope
    {
        // _x variable exists globally but because of scoping in rust,
        // it is different from the one in this scope which is marked by the curly braces.
        // NOTE: if _x is mutable, the value in the scope will update the global value.
        // let _x: i16 = 19;
        // let _z: i16 = 19;
        // println!("{}", _x);
    }

    // Shadowing: This allows us to redefine a variable
    // let a: i32 = 4;
    // println!("{a}");
    // let a: i32 = 9;
    // println!("{a}");

    // Patterns: These are not language construct,they help you acheive things like tuple destructuring.
    // Structs can also be destructured

    // let (x, y) = (11, 12);
    // println!("{x}");
    // println!("{y}");

    // Struct Destructuring
    // struct Person {
    //     name: &'static str,
    //     age: i16,
    //     likes_cookies: bool,
    // }

    // let person: Person = Person {
    //     name: "John",
    //     age: 34,
    //     likes_cookies: false,
    // };

    // let Person { .. } = person;

    // println!("{}", person.likes_cookies);
    // println!("{}", person.age);
    // println!("{}", person.name);

    // Conditional Statements
    // let val = 1;
    // if val < 2 {
    //     println!("{}", val)
    // } else if val > 5 {
    //     println!("val is above average")
    // } else {
    //     println!("Val is too low")
    // }

    // Looping

    // Initialize a variable for the index
    // let mut i: u32 = 10;

    // The Loop
    // loop {

    //     // Condition to break loop
    //     if i == 0 {
    //         break;
    //     }

    //     // Print out the index
    //     println!("{i}...");

    //     // Statement to achieve condition
    //     i -= 1
    // }
    // println!("Launch!!!");

    // While Loop

    // while i != 0 {
    //     // Print out the index
    //     println!("{i}...");

    //     // Statement to achieve condition
    //     i -= 1
    // }
    // println!("Launch!!!");

    // For Loop

    for i in (1..=10).rev() {
        // To skip odd numbers
        if i % 2 != 0 {
            continue;
        }
        // Print out the index
        println!("{i}...");
    }
    println!("Launch!!!");
}
