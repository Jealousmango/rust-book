# Variables
## Chapter 3.1

> In Rust, the compiler guarantees that when you state that a value won’t change, it really won’t change. That means that when you’re reading and writing code, you don’t have to keep track of how and where a value might change. Your code is thus easier to reason through.

> ...you can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```
> Shadowing is different from marking a variable as ```mut```, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the ```let``` keyword. By using ```let```, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.