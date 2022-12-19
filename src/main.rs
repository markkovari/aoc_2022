mod _1;
mod _10;
mod _11;
mod _12;
mod _13;
mod _14;
mod _15;
mod _16;
mod _2;
mod _3;
mod _4;
mod _5;
mod _6;
mod _7;
mod _8;
mod _9;

use _1::{get_1_first, get_1_second};
use _10::{get_10_first, get_10_second};
use _11::{get_11_first, get_11_second};
use _12::{get_12_first, get_12_second};
use _13::{get_13_first, get_13_second};
use _14::{get_14_first, get_14_second};
use _15::{get_15_first, get_15_second};
use _2::{get_2_first, get_2_second};
use _3::{get_3_first, get_3_second};
use _4::{get_4_first, get_4_second};
use _5::{get_5_first, get_5_second};
use _6::{get_6_first, get_6_second};
use _7::{get_7_first, get_7_second};
use _8::{get_8_first, get_8_second};
use _9::{get_9_first, get_9_second};

use crate::_16::solve_16;

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
    println!("6th day =>");
    println!("\t {}", get_6_first());
    println!("\t {}", get_6_second());
    println!("7th day =>");
    println!("\t {}", get_7_first());
    println!("\t {}", get_7_second());
    println!("8th day =>");
    println!("\t {}", get_8_first());
    println!("\t {}", get_8_second());
    println!("9th day =>");
    println!("\t {}", get_9_first());
    println!("\t {}", get_9_second());
    println!("10th day =>");
    println!("\t {}", get_10_first());
    println!("{}", get_10_second());
    println!("11th day =>");
    println!("\t {}", get_11_first());
    println!("\t {}", get_11_second());
    println!("12th day =>");
    println!("\t {}", get_12_first());
    println!("\t {}", get_12_second());
    println!("13th day =>");
    println!("\t {}", get_13_first());
    println!("\t {}", get_13_second());
    println!("14th day =>");
    println!("\t {}", get_14_first());
    println!("\t {}", get_14_second());
    println!("15th day =>");
    println!("\t {}", get_15_first());
    println!("\t {}", get_15_second());
    println!("16th day =>");
    let (first_16, second_16) = solve_16().unwrap();
    println!("\t {}", first_16);
    println!("\t {}", second_16);
}
