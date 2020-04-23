#[macro_use]
extern crate lazy_static;

pub mod ez_logger;
pub mod timestamp;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
