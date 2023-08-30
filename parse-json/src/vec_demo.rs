#[derive(Debug)]
enum Test {
    Int(i32),
    Float(f64),
    Text(String),
}
pub fn run() {
    // let mut v = vec![1, 2, 3, 4, 5];
    // if use mutable reference, can't change v, because v cmp  possible new memory, now first reference value not exist
    // let first = &v[0];
    // v.push(8);
    // println!("v: {:?}", v);

    // Add &mut, can change v
    let mut v = vec![1, 100, 20];
    for i in &mut v {
        *i += 50;
    }

    println!("v: {:?}", v);

    // save different type value, use enum
    let test_vec = vec![
        Test::Int(1),
        Test::Float(2.0),
        Test::Text("hello".to_string()),
    ];

    println!("test_vec: {:?}", test_vec);
}
