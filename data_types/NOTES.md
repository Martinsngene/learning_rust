In Rust, there are two subsets of data types:

1. Scalar
2. Compound

Scalar Types:
A scalar type represents a single value. There are 4 primary scalar types:

- Integers
- Floating-Point Numbers
- Booleans
- Characters

Integers:
An Integer is a number without a fraction. In Rust, we have both signed and unsigned integers. The table below shows the values:

TYPES OF INTEGERS IN RUST

---

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

Signed And Unsigned Integers:

Signed or unsigned means if it is possible for the number to have a sign or not. Any number that will stay positive i.e does not need to have a sign is known as an Unsigned interger while any number that has the tendency to have a sign whether positive or negative is referred to as a Signed integer.

Signed integers are stored using the two's complement representation.

Two's Complement Representation
This representation helps determine the range of numbers that can be stored in the respective bit lengths for both signed and unsigned integers.

For signed integers, variants can store numbers from `-(2^n-1) to 2^n-1 - 1` inclusive, where n is the number of bits that variant uses.

Variants of unsigned integers can store numbers from `0 to (2^n)-1`

The table below shows the range of numbers that can be stored in different bit lenghts.

---

| Length | Signed `-(2^n-1) to 2^n-1 - 1`               | Unsigned `0 to (2^n)-1`         |
| ------ | -------------------------------------------- | ------------------------------- |
| 8-bit  | i8 -> -128 to 127                            | u8 -> 0 to 255                  |
| 16-bit | i16 -> -32,768 to 32,767                     | u16 -> 0 to 524,287             |
| 32-bit | i32 -> -2,147,483,648 to 2,147,483,647       | u32 -> 0 to 4,294,967,295       |
| 64-bit | i64 -> -9.22337204E+18 to 9.22337204E+18 - 1 | u64 -> 0 to 1.884467441E+19 - 1 |
