use crate::calculator::{calculate_table};

pub fn print_table(z: u32) {
    let table = calculate_table("(*)", z, |x: u32, y: u32| -> u32 {
        (x * y) % z
    });

    println!("{}", table);
}
