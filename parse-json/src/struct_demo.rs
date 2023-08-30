#[allow(unused)]
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    fn update_email(&mut self, email: String) {
        self.email = email;
    }

    fn show(&self) {
        println!("username: {}, email: {}", self.username, self.email);
    }

    fn active(&self) -> &str {
        if self.active {
            return "Yes， active";
        }
        "No, not active"
    }
}

pub fn run() {
    let mut user = User::new("hello".to_string(), "start".to_string());

    // enum option useCase
    let some_test = Some(1);
    if let Some(item) = some_test {
        println!("item: {}", item);
    } else {
        println!("ddd is None");
    }

    // User::show(&user);

    user.show();
    user.update_email("end".to_string());
    user.show();

    // fn 中method与field相同，可以直接被调用
    // en: method and field are the same in fn, can be called directly
    println!("active: {}", user.active());
}
