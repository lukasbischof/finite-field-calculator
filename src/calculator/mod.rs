use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Color, Table, Cell, Attribute};

pub mod addition;
pub mod multiplication;
pub mod inverse;

fn build_table<F>(z: u32, op: F) -> Vec<Vec<Cell>> where F: Fn(u32, u32) -> u32 {
    (0..z).map(|y| {
        build_table_row(z, &y, |x, y| {
            op(x, *y)
        })
    }).collect()
}

fn build_table_row<F, T>(z: u32, y: &T, op: F) -> Vec<Cell> where
    F: Fn(u32, &T) -> T,
    T: std::fmt::Display
{
    let mut row = Vec::from_iter((0..z).map(|x| {
        Cell::new(op(x, y).to_string())
    }));
    row.insert(0, blue_cell(&y.to_string()));
    row
}

fn build_header(sign: &str, z: u32) -> Vec<Cell> {
    let mut header = vec![
        blue_cell(sign)
    ];
    let map = Vec::from_iter((0..z).map(|z| { blue_cell(&z.to_string()) }));
    header.extend(map);
    header
}

fn calculate_table<F>(sign: &str, z: u32, op: F) -> Table where F: Fn(u32, u32) -> u32 {
    let table_values: Vec<Vec<Cell>> = build_table(z, op);

    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(build_header(sign, z));

    for row in table_values {
        table.add_row(row);
    }

    table
}

fn blue_cell(content: &str) -> Cell {
    Cell::new(content).fg(Color::Blue).add_attribute(Attribute::Bold)
}