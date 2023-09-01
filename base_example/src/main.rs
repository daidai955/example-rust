use std::rc::Rc;

pub mod closures;
pub mod error_demo;
pub mod grep;
pub mod grep_utils;
pub mod iter_demo;
pub mod lifecycle_demo;
pub mod string_demo;
pub mod struct_demo;
pub mod trait_demo;
pub mod vec_demo;

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;

    // en: r1 and s are both valid from this point forward
    println!("r1: {}", r1);

    let d = &mut s;
    println!("d: {}", d);

    let mut text = "hello".to_string();

    let text_two = text;

    text = "nihao".to_string();

    println!("text: {}, text_two: {}", text, text_two);

    // // error, if use r1, will get error, because r1 is immutable reference
    // println!("r1: {}, d: {}", r1, d);
    // dangle();
    // vec pop push

    rc();
    str_test("world");
    struct_demo::run();
    vec_demo::run();
    string_demo::run();
    error_demo::run();
    trait_demo::run();
    lifecycle_demo::run();
    grep::run();
    closures::run();
    iter_demo::run();
}

fn rc() {
    let rd = Rc::new("hello".to_string());
    let re = rd.clone();
    let rf = rd.clone();
    // print rd re rf address
    println!("rd: {:p}, re: {:p}, rf: {:p}", rd, re, rf);
}

fn str_test(l: &str) {
    let s = "str_test fn: hello";
    let result = format!("s {}, {}", s, l);
    println!("{}", result);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, [2, 3]);
}

// // 悬垂引用: 等内存被回收后，引用的内存地址变量还存在，但是内存已经被回收了
// // en: dangling reference
// // fn dangle() -> &String {
// //     let s = String::from("hello");
// //     &s
// // }
//
// // fn first_word(s: &String) -> usize {
// //     let bytes = s.as_bytes();
//
// //     for (i, &item) in bytes.iter().enumerate() {
// //         if item == b' ' {
// //             return i;
// //         }
// //     }
// //     s.len()
// // }
//
// // 如何让struct等具有copy trait, 使得可以在函数中使用, 但是不会被drop
// // en: how to make struct have copy trait, so that can use in function, but not drop
