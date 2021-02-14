pub mod hosting;
pub mod serving;

//this is now available from outside and it brings hosting into scope
pub use crate::front_of_house::hosting as housting;

fn test() {
    housting::get_check();
}
