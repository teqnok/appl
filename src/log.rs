use std::fs;
macro_rules! info {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}
pub fn main() {
    info!("Hello, world");
}
