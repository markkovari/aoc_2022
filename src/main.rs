mod _1;
mod _2;
mod _3;

use _1::{get_1_first, get_1_second};
use _2::{get_2_first, get_2_second};
use _3::{get_3_first, get_3_second};

fn main() {
    println!("1st day =>");
    println!("\t {}", get_1_first());
    println!("\t {}", get_1_second());
    println!("2st day =>");
    println!("\t {}", get_2_first());
    println!("\t {}", get_2_second());
    println!("3rd day =>");
    println!("\t {}", get_3_first());
    println!("\t {}", get_3_second());
}
