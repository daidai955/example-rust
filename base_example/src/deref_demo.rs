use std::ops::Deref;

struct DerefDemo<T>(T);

impl<T> DerefDemo<T> {
    fn new(x: T) -> Self {
        DerefDemo(x)
    }
}

impl<T> Deref for DerefDemo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn run() {
    println!("-------- deref_demo.rs ---------");

    let x = 5;
    let y = DerefDemo::new(5);

    // 需要将y 从DerefDemo类型解引用得到值， 才能和x比较
    //assert_eq!(x, y);
    assert_eq!(x, *y);
}
