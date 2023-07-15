enum Status {
    New,
    Online(u16, u16, u16, u16),
    Paused,
}

impl Status {
    fn is_online(&self) -> bool {
        match self {
            Status::Online(_, _, _, _) => true,
            Status::New => false,
            Status::Paused => false,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let server = Status::New;
    if server.is_online() {
        println!("Server is online");
    }
    let server = Status::Paused;
    if server.is_online() {
        println!("Server is online");
    }
    let server = Status::Online(127, 0, 0, 1);

    if server.is_online() {
        println!("Server is online");
    }

    let server = Some(Status::New);

    if server.is_some() {
        println!("Server is present");
        match server {
            Some(Status::New) => println!("Server is new"),
            Some(Status::Online(_, _, _, _)) => println!("Server is online"),
            Some(Status::Paused) => println!("Server is paused"),
            None => (),
        }
    }

    if let Some(s) = server {
        println!("{}", s.is_online());
    }
}
