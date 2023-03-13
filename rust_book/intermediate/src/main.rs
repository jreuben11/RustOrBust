

fn main() {
    fp::closures();
    fp::iterators();

    smart_pointers::on_heap();
    smart_pointers::ref_counting();
}

mod fp { //CH13
    use std::thread;

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }
    
    struct Inventory {
        shirts: Vec<ShirtColor>,
    }
    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked()) //closure
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
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    fn shirt_pref (){
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };
    
        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );
    
        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    pub fn closures (){
        shirt_pref();

        fn  _add_one_v1   (x: u32) -> u32 { x + 1 }
        let _add_one_v2 = |x: u32| -> u32 { x + 1 };
        let  add_one_v3 = |x|             { x + 1 };
        let  add_one_v4 = |x|               x + 1  ;
        add_one_v3(1);
        add_one_v4(1);

        let list1 = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list1);
        let only_borrows = || println!("From closure: {:?}", list1);
        println!("Before calling closure: {:?}", list1);
        only_borrows();
        println!("After calling closure: {:?}", list1);

        let mut list2 = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list2);
        let mut borrows_mutably = || list2.push(7);
        borrows_mutably();
        println!("After calling closure: {:?}", list2);

        let list3 = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list3);
        thread::spawn(move || println!("From thread: {:?}", list3))
            .join()
            .unwrap();

        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let mut list4 = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        list4.sort_by_key(|r| r.width);
        println!("{:#?}", list4);

    }

    pub fn iterators(){
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Shoe {
        pub size: u32,
        pub style: String,
    }
    
    pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }


}
#[cfg(test)]
mod iterator_tests {
    #[test]
    fn manual_iterator() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    
    #[test]
    fn iterator_filter() {
        use crate::fp::*;
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// CH14
// cargo doc --open 

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod smart_pointers { //CH15
    pub fn on_heap() {
        let b = Box::new(5);
        println!("b = {}", b);

        let x = 5;
        let y = &x;
        let z = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *z);
        println!("{x}-{y}-{z}");

        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::*;
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        use std::ops::Deref;

        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        let a = 5;
        let b = MyBox::new(a);
        assert_eq!(5, a);
        assert_eq!(5, *b);

        fn hello(name: &str) {
            println!("Hello, {name}!");
        }
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        hello(&(*m)[..]);

        struct CustomSmartPointer {
            data: String,
        }
        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }
        {
            let c = CustomSmartPointer {
                data: String::from("my stuff"),
            };
            let d = CustomSmartPointer {
                data: String::from("other stuff"),
            };
            let e = CustomSmartPointer {
                data: String::from("some data"),
            };
            println!("CustomSmartPointers created.");
            drop(e);
            println!("CustomSmartPointer e explicitly dropped before the end of main.");
        }

        pub fn ref_counting(){

        }

    }
}