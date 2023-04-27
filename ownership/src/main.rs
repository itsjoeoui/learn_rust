fn main() {
    // give away ownership, String gets motified, and returned back ownership
    let s1 = String::from("hello");
    let s1 = add_world(s1);
    println!("{}", s1);

    // mutable reference
    let mut s2 = String::from("Hello");

    // This part is not good
    // let r1 = &mut s2; // first mutable reference to s2
    // let r2 = &mut s2; // second mutable reference to s2 (NOT GOOD!)
    // you cannot have 2 mutable references at the same time
    // this is to prevent data racing problems.
    // println!("{} {}", r1, r2);

    // why does this work?
    // motify_mut will take the owenership of the mutable reference to s2
    // after the function returns, the mutable reference is dropped
    motify_mut(&mut s2);
    // this second call to motify_mut will create a new mutable reference
    // to s2 which is completely separated from the first mut ref
    motify_mut(&mut s2);
    println!("{}", s2);

    println!("{}", calculate_len(&s2));
}

fn calculate_len(s: &String) -> usize {
    // this is the same as s.len()
    // because rust will
    (*s).len()
}

fn motify_mut(s: &mut String) {
    s.push_str(", world!");
}

fn add_world(mut s: String) -> String {
    s.push_str(", world!");
    return s;
}
