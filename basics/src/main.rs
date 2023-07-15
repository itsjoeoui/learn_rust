const DAYS: u32 = 7; // const is always immutable
                     // you must annotate the type of the constant
fn main() {
    let x = 6;
    println!("{}", x);
    let x = 5; // shawowing
    println!("{}", x);

    let x = "lmao"; // shawowing (can also be different type)
    println!("{}", x);

    let mut x = 7;
    println!("{}", x);
    x = 1;
    println!("{}", x);

    println!("There are {} days in a week", DAYS);

    // tuple type
    let coordinate: (u32, u32, u32) = (1, 2, 3);
    let (x, y, z) = coordinate;
    println!("The coordinate is ({}, {}, {})", x, y, z);

    println!("The first coordinate is {}", coordinate.0);

    let three_fives = [3; 5];
    println!("The array is {:?}", three_fives);

    let some_val = {
        let x = 2;
        x + 1
    };
    println!("The value is {}", some_val);

    let mut counter = 0;
    let twenty = loop {
        if counter == 10 {
            break counter * 2;
        }
        counter += 1;
    };
    println!("{}", twenty);

    for i in (1..=3).rev() {
        println!("{i}")
    }
}
