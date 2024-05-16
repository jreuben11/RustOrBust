pub fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
    F: Fn(FIRST) -> SECOND,
    G: Fn(SECOND) -> THIRD,
{
    move |x| g(f(x))
}

macro_rules! compose {
    ($last:expr) => { $last };
    ($head:expr,$($tail:expr),+) => {
        compose_two($head, compose!($($tail),+))
    }
}

macro_rules! compose_alt {
    ($last:expr) => { $last };
    ($head:expr => $($tail:expr)=>+) => {
        compose_two($head, compose_alt!($($tail)=>+))
    }
}
