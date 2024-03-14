// #![feature(test)]
// extern crate test;

/// # A Simple Addition
/// 
/// Adds two integers.
/// 
/// # Arguments
/// 
/// - *a* the first term, needs to be i32
/// - *b* the second term, also a i32
/// 
/// ## Returns
/// The sum of *a* and *b*.
/// 
/// # Panics
/// The addition is not done safely, overflows will panic!
/// 
/// # Examples
/// 
/// ```rust
/// assert_eq!(crate_test::my_add(1, 1), 2);
/// ```
pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}

#[macro_use]
extern crate bencher;

#[cfg(test)] 
mod tests { 
    use super::*;
    // use test::Bencher;

    #[test] 
    fn this_works() { 
        assert_eq!(my_add(1, 1), 2); 
    }
    
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn this_does_not_work() {
        assert_eq!(my_add(std::i32::MAX, std::i32::MAX), 0);
    }

    use bencher::Bencher;
    // #[bench]
    fn how_fast(b: &mut Bencher) {
        b.iter(|| my_add(42, 42))
    }



    
}

