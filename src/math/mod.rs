pub mod combinations;
pub mod gcd;
pub mod number_theory;
pub mod prime;

pub use number_theory::{
    fast_power::fast_power,
    egcd::egcd,
    invmod::invmod,
    chinese_remainder_theorem::crt,
};
pub use combinations::combinations;
pub use gcd::gcd;
pub use prime::{
    next_prime::next_prime,
    get_prime::get_prime,
    is_prime::is_prime,
};
