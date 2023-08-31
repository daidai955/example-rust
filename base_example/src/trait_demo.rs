use std::fmt::Display;

pub trait Area {
    fn get_area(&self) -> f64;
    fn get_area_text(&self, extra: &str) -> String {
        format!("Area is {} {}", self.get_area(), extra)
    }
}

struct Goods<T> {
    width: f64,
    height: f64,
    item: T,
}

impl<T> Area for Goods<T> {
    fn get_area(&self) -> f64 {
        self.width * self.height 
    }
}

impl<T: Display> Goods<T> {
    fn get_values(&self, item: T)-> String {
        format!("{} {} {}", self.width, item, self.item)
    }
}

pub fn run() {
    let goods = Goods {
        width: 30.0,
        height: 20.0,
        item: 10.0,
    };

    println!("area_text {}", goods.get_area_text("hello"));
    println!("get_values {}", goods.get_values(10.0));

}

pub fn notify<T: Area>(item: T) {
    println!("{}", item.get_area());
}
