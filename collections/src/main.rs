struct User {
    username: String,
    sign_in_count: u64,
}

fn main() {
    let v: Vec<u32> = Vec::new();
    println!("{:?}", v);

    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    v.push(1);
    v.push(1);
    v.push(1);
    println!("{:?}", v);

    // this gives us the reference to the 3rd element
    let third: &i32 = &v[3];
    println!("{}", third);
    println!("{}", third);

    // this is a safer way that returns an Option
    let third: Option<&i32> = v.get(3);
    // in case when there is no value, this will not cause a panic

    match third {
        Some(num) => println!("The third element is {}", num),
        None => println!("There is no third element"),
    }

    // assume we have the following vector. What will happen?
    // Since we are holding an immutable reference, and using it later.
    // During the period of holding the reference, we are not allowed to mutate the vector.
    // this is why the v.push(6) will panic

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        // we need to first dereference it then we can update the value
        *i += 50;
    }

    let num: Option<u32> = Some(5);
    match num {
        Some(n) => println!("Num: {}", n),
        None => println!("There is no num"),
    }
    num;

    let u: Option<User> = Some(User {
        username: String::from("lmao"),
        sign_in_count: 1,
    });
    match u {
        // ref is a must here since User does not implement the Copy trait
        Some(ref user) => println!("User: {}", user.username),
        None => println!("There is no user"),
    }
    u;
}
