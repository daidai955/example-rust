// 实现继承？平衡的设计
// 今天的主题是 trait对象， 就是下面的<Box<dyn Draw>>
// https://kaisery.github.io/trpl-zh-cn/ch17-02-trait-objects.html

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components.iter().for_each(|item| item.draw());
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox height:  {}", self.height);
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Button witdh: {}", self.width);
    }
}

pub fn run() {
    let components = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 32,
                height: 4,
                label: String::from("OK"),
            }),
        ],
    };

    components.run();
}

// 类比于下面的trait bound来说，则是同个类型
// pub struct Screen2<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen2<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         self.components.iter().for_each(|item| item.draw());
//     }
// }

//  Define the Draw interface (similar to Rust's trait)
//interface Draw {
//    draw(): void;
//}

//// Define the Draw interface (similar to Rust's trait)
//class ScreenStruct {
//    components: Draw[];

//    constructor(components: Draw[]) {
//        this.components = components;
//    }

//    run() {
//        this.components.forEach(component => component.draw());
//    }
//}

//// Define the SelectBox class
//class SelectBox implements Draw {
//    width: number;
//    height: number;
//    options: string[];

//    constructor(width: number, height: number, options: string[]) {
//        this.width = width;
//        this.height = height;
//        this.options = options;
//    }

//    draw() {
//        console.log(`SelectBox height: ${this.height}`);
//    }
//}

//// Define the Button class
//class Button implements Draw {
//    width: number;
//    height: number;
//    label: string;

//    constructor(width: number, height: number, label: string) {
//        this.width = width;
//        this.height = height;
//        this.label = label;
//    }

//    draw() {
//        console.log(`Button width: ${this.width}`);
//    }
//}

//// Define the run function
//function run() {
//    const components = new ScreenStruct([
//        new SelectBox(75, 10, ["Yes", "Maybe", "No"]),
//        new Button(32, 4, "OK")
//    ]);

//    components.run();
//}

//// Run the code
//run();
