use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::Table;

use crate::calculator::{build_header, build_table_row};

pub fn print_table(z: u32) {
    let table_values = vec![
        build_table_row(z, &String::from("-a"), |x, _y| {
            additive_inverse(x, z).to_string()
        }),
        build_table_row(z, &String::from("a^-1"), |x, _y| {
            match multiplicative_inverse(x, z) {
                Some(v) => v.to_string(),
                None => String::from("x")
            }
        }),
    ];

    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(build_header("Inv.", z));

    for row in table_values {
        table.add_row(row);
    }

    println!("{}", table);
}

fn additive_inverse(x: u32, z: u32) -> u32 {
    (z - x) % z
}

fn multiplicative_inverse(x: u32, z: u32) -> Option<u32> {
    for xi in 1..z {
        if x * xi % z == 1 {
            return Some(xi);
        }
    }

    None
}