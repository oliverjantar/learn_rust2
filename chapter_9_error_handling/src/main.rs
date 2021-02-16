mod recovable_errors;
mod test;

use std::error::Error;
use test::Guess;

fn main() -> Result<(), Box<dyn Error>> {
    recovable_errors::run();
    //panic!("Major error!");

    let x: u32 = 4;

    Ok(())
}

fn test() {
    let x = Guess::new(32);
    let value = x.value(); // cannot use x.value as it's private
}
/*

[profile.release]
panic = 'abort'

use to immediatelly end the program
os then cleans up memory


*/
