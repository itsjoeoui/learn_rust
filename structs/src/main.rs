struct User {
    name: String,
    age: u32,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn cat_fit(&self, shape: &Rectangle) -> bool {
        self.width >= shape.width && self.height >= shape.height
    }
    fn hello() {
        // this is an associate function
        println!("Hello Rectangle")
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("Joey"),
        age: 19,
    };

    if user1.active {
        print_user(&user1)
    }

    user1.active = false;

    let (user2, mut user1) = clone_user(user1);

    user1.name = String::from("Joey2");
    print_user(&user1);
    print_user(&user2);

    // Rectangle example
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    Rectangle::hello();

    println!("The area of rect1 is {}", rect1.area());
    let square1 = Rectangle::square(55);
    println!("The area of rect1 is {}", square1.area());

    println!("The square can fit in rect1: {}", rect1.cat_fit(&square1));
}

fn clone_user(user: User) -> (User, User) {
    (
        User {
            active: false,
            name: user.name.clone(),
            ..user // this only works when user is a value, not a reference
        },
        user,
    )
}

fn print_user(user: &User) {
    println!("The user's name is {}, age is {}", user.name, user.age);
}
