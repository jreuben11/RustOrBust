

fn main() {
    fp::closures();
    fp::iterators();

    smart_pointers::on_heap();
    smart_pointers::ref_counting();
    smart_pointers::interior_mutability();
    smart_pointers::multiple_owner_mut();
    smart_pointers::circular_ref_prevention();

    concurrency::thread_spawn();
    concurrency::message_passing();
    concurrency::shared_state();

    oop::encapsulation();
    oop::duck_typing();
    oop::states_as_types();

    pattern_matching::patterns_everywhere();

    advanced::safety_off();
    advanced::trait_patterns();
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

    }
    pub fn ref_counting(){
        use std::rc::Rc;
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }
        use List::*;
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    pub fn interior_mutability(){
        pub trait Messenger {
            fn send(&self, msg: &str);
        }
        pub struct LimitTracker<'a, T: Messenger> {
            messenger: &'a T,
            value: usize,
            max: usize,
        }
        impl<'a, T> LimitTracker<'a, T>
        where
            T: Messenger,
        {
            pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }
            }
            pub fn set_value(&mut self, value: usize) {
                self.value = value;
                let percentage_of_max = self.value as f64 / self.max as f64;
                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger
                        .send("Urgent warning: You've used up over 90% of your quota!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger
                        .send("Warning: You've used up over 75% of your quota!");
                } else {
                    println!("blah");
                }
            }
        }

        use std::cell::RefCell;
        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }
        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                println!("sent:{}", message); 
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }

        let mock_messenger = MockMessenger::new();
        let mut mocked_limit_tracker = LimitTracker::new(&mock_messenger, 100);
        mocked_limit_tracker.set_value(95);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    pub fn multiple_owner_mut(){
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }
        use List::*;

        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("a before = {:?}", a);
        println!("b before = {:?}", b);
        println!("c before = {:?}", c);
        *value.borrow_mut() += 10;
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    pub fn circular_ref_prevention(){
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }
        

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

pub mod concurrency { //CH16
    pub fn thread_spawn(){
        use std::thread;
        use std::time::Duration;

        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }); 
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();

        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        handle.join().unwrap();
    }

    pub fn message_passing(){
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
    
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
    
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        // let received = rx.recv().unwrap();
        // println!("Got: {}", received);
        for received in rx {
            println!("Got: {}", received);
        }
    }

    pub fn shared_state(){
        use std::sync::{Arc,Mutex};
        use std::thread;

        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![]; // vector for holding thread handles, so we can join on all
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }

}

pub mod oop { //CH17
    pub fn encapsulation (){
        pub struct AveragedCollection {
            list: Vec<i32>,
            average: f64,
        }
        impl AveragedCollection {
            pub fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }
            pub fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    }
                    None => None,
                }
            }
            pub fn average(&self) -> f64 {
                self.average
            }
            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }

        let mut ac = AveragedCollection{list:vec!(), average:0f64};
        for i in 1..5 {
            ac.add(i);
        }
        println!("average:{}", ac.average());
        


    }

    pub fn duck_typing (){
        pub trait Draw {
            fn draw(&self);
        }
        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }
        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
        #[derive(Debug)]
        pub struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }
        impl Draw for Button {
            fn draw(&self) {
                println!("draw: {:?}", &self);
            }
        }
        #[derive(Debug)]
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        } 
        impl Draw for SelectBox {
            fn draw(&self) {
                println!("draw: {:?}", &self);
            }
        }

        let screen = Screen {
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
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
    
        screen.run();

    }

    pub fn states_as_types(){
        pub struct Post {
            content: String,
        }
        pub struct DraftPost {
            content: String,
        }
        pub struct PendingReviewPost {
            content: String,
        }
        
        
        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }
            pub fn content(&self) -> &str {
                &self.content
            }
        }
        
        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }
            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost {
                    content: self.content,
                }
            }
        }

        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }
        }

        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

pub mod pattern_matching { //CH18
    pub fn patterns_everywhere(){
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }

        let (x, y, z) = (1, 2, 3);

        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);

        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            1..=5 => println!("one through five"),
            _ => println!("anything"),
        }

        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        let Point { x, y } = p;
        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }

        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }

        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);

        struct Point3 {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point3 { x: 0, y: 0, z: 0 };
        match origin {
            Point3 { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }

        let num = Some(4);
        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }

        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }

        enum Message2 {
            Hello { id: i32 },
        }  
        let msg = Message2::Hello { id: 5 };  
        match msg {
            Message2::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message2::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message2::Hello { id } => println!("Found some other id: {}", id),
        }
        
    }
  
}

pub mod advanced { // CH19

    pub fn safety_off(){

        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        let r3 = 0x012345usize as *const i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        unsafe fn dangerous() {}
        unsafe {
            dangerous();
        }

        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            use std::slice;
            let len = values.len();
            let ptr = values.as_mut_ptr();
            assert!(mid <= len);
        
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);
        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }

        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }

        static mut COUNTER: u32 = 0;
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        add_to_count(3);
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }

        unsafe trait Foo {
            // methods go here
        }
        unsafe impl Foo for i32 {
            // method implementations go here
        }
    }

    pub fn trait_patterns(){
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        pub struct Counter {}
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                Some(1)
            }
        }
        let mut c = Counter{};
        let i = c.next().unwrap_or_default();
        println!("{}",i);

        use std::ops::Add;
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Add for Point {
            type Output = Point;
            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
        struct Millimeters(u32);
        struct Meters(u32);
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }
        let x = Millimeters{0:10} + Meters{0:1};
        println!("length:{}", x.0);

        use std::fmt;
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        impl OutlinePrint for Point {}
        let p = Point{x:1,y:2};
        println!("{p}");
        p.outline_print();

        struct Wrapper(Vec<String>);
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }
        let w = Wrapper(vec![String::from("NewType"), String::from("Pattern!")]);
        println!("w = {}", w);

        type Thunk = Box<dyn Fn() + Send + 'static>; // on heap multi-trait contract
        fn takes_long_type(f: Thunk) {
            f();
        }
        fn returns_long_type() -> Thunk {
            Box::new(|| println!("hi"))
        }
        let f: Thunk = returns_long_type();
        takes_long_type(f);

        fn add_one(x: i32) -> i32 {
            x + 1
        }
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);

        enum Status {
            Value(u32),
            Stop,
        }
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
        let c = returns_closure();
        println!("{}",c(1));
    }

    

}
