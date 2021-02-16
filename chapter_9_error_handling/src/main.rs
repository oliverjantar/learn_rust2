mod recovable_errors;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    recovable_errors::run();
    //panic!("Major error!");

    Ok(())
}

/*

[profile.release]
panic = 'abort'

use to immediatelly end the program
os then cleans up memory


*/
