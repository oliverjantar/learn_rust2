mod recovable_errors;

fn main() {
    recovable_errors::run();
    //panic!("Major error!");
}

/*

[profile.release]
panic = 'abort'

use to immediatelly end the program
os then cleans up memory


*/
