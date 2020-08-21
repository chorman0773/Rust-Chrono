
#![feature(min_const_generics,associated_type_bounds)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod clock;
pub mod ratio;
pub mod duration;
pub mod time_point;
pub mod prelude;

