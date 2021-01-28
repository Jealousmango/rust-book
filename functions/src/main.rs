fn main() {
    another_function(5, 6);

    let x = five();
    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this in mind as you explore function return values and expressions next.
    };
    println!("The value of y is: {}", y);

    let t = plus_one(9);
    println!("The value of t is: {}", t);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
