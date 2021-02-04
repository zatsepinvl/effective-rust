mod restaurant;

use restaurant::back_of_house::cook;
use restaurant::serve;

fn main() {
    cook();
    serve();
}