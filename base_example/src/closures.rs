#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Invertory {
    shirts: Vec<ShirtColor>,
}

impl Invertory {
    fn giveaway(&self, user_prefernce: Option<ShirtColor>) -> ShirtColor {
        user_prefernce.unwrap_or_else(|| self.most_stocked())
        // user_prefernce.unwrap_or(self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red >= num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn run() {
    println!("------ closures run -------");

    let list = vec![1, 2, 3];
    let show_borrows = || println!("List {:?} ", list);

    show_borrows();
    println!("List {:?} ", list);

    let store = Invertory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with prefernce : {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with prefernce : {:?} gets {:?}",
        user_pref2, giveaway2
    );

    run_sort_by_key();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn run_sort_by_key() {
    let mut list = vec![
        Rectangle {
            width: 10,
            height: 20,
        },
        Rectangle {
            width: 5,
            height: 5,
        },
        Rectangle {
            width: 40,
            height: 60,
        },
    ];

    list.sort_by_key(|r| r.width * r.height);
    println!("list : {:?}", list);
    // Why sort_by_key use FnMut ? becuase it will change the list

    // 因为String不具备Copy trait, 所以当save.push(save_string)时，save_string被move了
    // 而sort_by_key是FnMut,所以会报错
    // 可以修改成一个具有Copy trait的类型，让闭包去捕获到可变引用
    // let mut save = vec![];
    // let save_string = String::from("save");
    // // let mut save_index = 1;
    // list.sort_by_key(|r| {
    //     save.push(save_string);
    // //  save += 1;
    //     r.width * r.height
    // });
}

// pub enum Options<T> {
//     None,
//     Some(T),
// }
//
// impl<T> Options<T> {
//     fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Options::Some(t) => t,
//             Options::None => f(),
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_mini_closure() {
//         assert_eq!(mini_closure(5), 5);
//     }
// }
