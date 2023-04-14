fn main(){
    random::gen_numeric();
    random::gen_numeric_range();
    random::gen_numeric_distribution().unwrap();
    random::gen_random_values_of_t();
    random::gen_alphanumeric_distribution(10);
    random::gen_alphanumeric_from_charset();

    vector_sort();
}

#[macro_use] extern crate function_name;

macro_rules! function_path {() => (concat!(
    module_path!(), "::", function_name!()
))}

mod random {
    use rand::{thread_rng, Rng};
    

    #[named]
    pub fn gen_numeric() {
        println!("\n{}:", function_path!().to_uppercase());

        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        println!("Random u8: {}", n1);
        println!("Random u16: {}", n2);
        println!("Random u32: {}", rng.gen::<u32>());
        println!("Random i32: {}", rng.gen::<i32>());
        println!("Random float: {}", rng.gen::<f64>());
    }

    #[named]
    pub fn gen_numeric_range() {
        println!("\n{}:", function_path!().to_uppercase());
        use rand::distributions::{Distribution, Uniform};

        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(0..10));
        println!("Float: {}", rng.gen_range(0.0..10.0));
        let die = Uniform::from(1..7);

        loop {
            let throw = die.sample(&mut rng);
            println!("Roll the die: {}", throw);
            if throw == 6 {
                break;
            }
        }
    }

    use rand_distr::NormalError;
    #[named]
    pub fn gen_numeric_distribution() -> Result<(), NormalError> {
        println!("\n{}:", function_path!().to_uppercase());

        use rand_distr::{Distribution, Normal};

        let mut rng = thread_rng();
        let normal = Normal::new(2.0, 3.0)?;
        let v = normal.sample(&mut rng);
        println!("{} is from a N(2, 9) distribution", v);
        Ok(())
    }

    
    #[named]
    pub fn gen_random_values_of_t() {
        println!("\n{}:", function_path!().to_uppercase());
        
        use rand::distributions::{Distribution, Standard};

        #[allow(dead_code)]
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Distribution<Point> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
                let (rand_x, rand_y) = rng.gen();
                Point {
                    x: rand_x,
                    y: rand_y,
                }
            }
        }
        let mut rng = rand::thread_rng();
        let rand_tuple = rng.gen::<(i32, bool, f64)>();
        let rand_point: Point = rng.gen();
        println!("Random tuple: {:?}", rand_tuple);
        println!("Random Point: {:?}", rand_point);
    }

    
    
    #[named]
    pub fn gen_alphanumeric_distribution(len: usize) {
        println!("\n{}:", function_path!().to_uppercase());
        
        use rand::distributions::Alphanumeric;
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
    
        println!("{}", rand_string);
    }

    #[named]
    pub fn gen_alphanumeric_from_charset() {
        println!("\n{}:", function_path!().to_uppercase());

        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";
        const PASSWORD_LEN: usize = 30;
        let mut rng = rand::thread_rng();
    
        let password: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
    
        println!("{:?}", password);
    }

}

#[named]
fn vector_sort(){
    println!("\n{}:", function_path!().to_uppercase());

    let mut int_vec = vec![1, 5, 10, 2, 15];
    int_vec.sort();
    assert_eq!(int_vec, vec![1, 2, 5, 10, 15]);

    let mut float_vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    float_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(float_vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {name: String, age: u32}
    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort people by derived natural order (Name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);

    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]);
}
