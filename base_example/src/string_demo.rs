use std::collections::HashMap;

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

    // error , a is moved, can not use again
    // println!("c: {}, a: {}", c, a)
    println!("c: {}, b: {}", c, b);

    // hashmap
    let mut ages = HashMap::new();

    let kang = String::from("kangkang");
    let dang = String::from("dangdang");

    ages.insert(kang, 23);
    ages.insert(dang, 24);

    for (key, item) in &ages {
        println!("key: {}, item: {}", key, item);
    }

    // kang is move to ages, can not use again
    // println!("kang: {}", kang);

    // 如何去更新hashmap中的值, hard , soft , smart
}
