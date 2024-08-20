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
This representation gives helps determine the range of numbers that can be stored in the respective lengths for both signed and unsigned integers.

For signed integers, variants can store numbers from `-(2^n-1) to 2^n-1 - 1` inclusive, where n is the number of bits that variant uses.

Variants of unsigned integers can store numbers from `0 to 2^n-1`
