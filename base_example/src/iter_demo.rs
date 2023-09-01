pub fn run() {
    println!("=== Iter Demo ===");
    let v1 = vec![100, 2, 3];
    let v1_iter = v1.iter();

    // show v1_iter.next(). custom iterator
    // if let Some(v1) = v1_iter.next() {
    //     println!("v1: {} {:?}", v1, v1_iter);
    // }
    // if let Some(v1) = v1_iter.next() {
    //     println!("v1: {} {:?}", v1, v1_iter);
    // }
    // if let Some(v1) = v1_iter.next() {
    //     println!("v1: {} {:?}", v1, v1_iter);
    // }
    // if let Some(v1) = v1_iter.next() {
    //     println!("v1: {} {:?}", v1, v1_iter);
    // }

    // for v1 in v1_iter {
    //     println!("v1: {}", v1);
    // }

    let sum = v1_iter.sum::<i32>();
    println!("sum: {}", sum);
}
