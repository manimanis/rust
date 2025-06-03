#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

struct User1 {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

#[derive(Debug)]
struct Point(i32, i32);

fn main() {
    let abdou = Person {
        first_name: String::from("Abdou"),
        last_name: String::from("MANI"),
        age: 12,
    };
    println!(
        "{:?}",
        Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 25
        }
    );
    println!("{:?}", abdou);
    println!("{} {}", abdou.first_name, abdou.last_name);

    let mut user = User::new(
        String::from("username"),
        String::from("email"),
        String::from("uri"),
    );
    println!("Hello, {} {}", user.username, user.active);
    user.deactivate();
    println!("Hello, {} {}", user.username, user.active);

    let username = String::from("other user");
    let email = String::from("other email");
    let uri = String::from("other uri");
    let active = true;

    let other_user = User1 {
        username,
        email,
        uri,
        active,
    };
    println!("Hello, {} {}", other_user.username, other_user.active);
    
    let pt = Point(200, 300);
    println!("{:?}", pt);
}
