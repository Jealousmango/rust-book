fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x); // 5
    x = 6;
    println!("The value of x is: {}", x); // 6
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS const: {}", MAX_POINTS);

    let shadow_example = 5;

    let shadow_example = shadow_example + 1;

    let shadow_example = shadow_example * 2;

    println!("The value of shadow_example is: {}", shadow_example); // 12
}
