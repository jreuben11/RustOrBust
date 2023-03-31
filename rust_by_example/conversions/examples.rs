fn main() {
     
    from_and_into();
    tryfrom_and_tryinto();
    string_conversions();
}

fn from_and_into(){
    println!("\nFROM AND INTO:");
    // use std::convert::From; 

    #[derive(Debug)]
    struct Number {
        _value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { _value: item }
        }
    }


    let int = 5;
    let num1 = Number::from(int);
    println!("My number is {:?}", num1);

    // Try removing the type annotation -> compilation error
    let num2: Number = int.into();
    println!("My number is {:?}", num2);

}



fn tryfrom_and_tryinto() {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

fn string_conversions(){
    println!("\nSTRING CONVERSIONS:");
    let my_str: &str = "hello";
    let _my_string: String = String::from(my_str);   

    use std::fmt;

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}