use _1::{get_1_first, get_1_second};
use _2::{get_2_first, get_2_second};
use _3::{get_3_first, get_3_second};
use _4::{get_4_first, get_4_second};
use _5::{get_5_first, get_5_second};

mod _1;
mod _2;
mod _3;
mod _4;
mod _5;

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
    println!("4th day =>");
    println!("\t {}", get_4_first());
    println!("\t {}", get_4_second());
    println!("5th day =>");
    println!("\t {}", get_5_first());
    println!("\t {}", get_5_second());
}
