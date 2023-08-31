use std::fs::File;
use std::io::{self};
// use std::io::ErrorKind;
//

struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    // let file_result = File::open("hello.txt");

    // let file  = match file_result {
    //     Ok(result) => {
    //         result
    //     },
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(result) => result,
    //             Err(error) => panic!("Problem creating the file: {:?}", error)
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         },
    //     },
    // };

    // println!("Enter run file is {:?}", file);

    // let file_result = File::open("hello.txt").expect("Failed to open hello.txt");
    // println!("Enter run file is {:?}", file_result);
    
    let d = Point{x: 5, y: 10};
    println!("d.x = {}", d.x);

}

// 将错误信息传递给调用者
fn read_file() -> Result<File, io::Error> {
    let file_result = File::open("hello.txt")?;
    Ok(file_result)
}
