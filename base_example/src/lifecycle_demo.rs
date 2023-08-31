pub fn run() {
    println!("--- Lifecycle Demo ---");
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    let a = "hello";

    // let b ;
    // {
    //     let c= "world";
    //     b = &c;
    // }

    let result;

    {
        let b = "world";
        result = longest(a, b);
    }

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
