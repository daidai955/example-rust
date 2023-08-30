pub fn run() {
    println!("------ string-demo  ------");
    let a = "hello".to_string();
    println!("a: {}", a);

    let mut name = String::from("dang");
    name.push_str(" dang");
    println!("name: {}", name);

    let a = "hello".to_string(); 
    let b = "world".to_string();

    let c = a + &b[..];

    // error , a is moved
    // println!("c: {}, a: {}", c, a)
    println!("c: {}, b: {}", c, b)
}
