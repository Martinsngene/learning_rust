Difference between immutable variables and constants:

1. You can't use `mut` with constants. Also, they are not just immutable by default, they are always immutable.

2. Unlike immutable variables, constants are declared using the `const` keyword and not `let` and you my annotate the type when the variable is declared.

3. Constants can be declared in any scope even in the global scope.

4. The value of constants must be a known expression not any other value that is only known at runtime like a function call.

Features of Constants:

- The name of a constant variable must all be in capital letters and underscores should be used inbetween words.

Shadowing:
A variable is shadowed when it's name is used to declare another variable with the let keyword and it value. e.g

```
let x = 3;

let x = x + 1;

println!("x is: {}", x) // 4
```

The second variable is said to have shadowed the first. When the value of x is read, the second value is what we'll get.

Differences Between Mutable Mariables And Shadowing A Variable:

- If you shadow a variable, the variable will not remain mutable once the operations performed on it is complete. But if you make a variable mutable, it remains mutable using the `mut` keyword, it remains mutable throughout the program.

- When a variable is shadowed, the type previously annotated to it can be changed, this is possible because shadowing is actually creating a new variable with the same name as the first. But this is not true for a mutable variable.

NOTE: A clear importance of shadowing is that it allows us to reuse variable names and won't have to create many more variables during our programs.
