mod calculator;

fn main() {
    let z = parse_bound();

    println!("Addition field:");
    calculator::addition::print_table(z);
    println!("Multiplication field:");
    calculator::multiplication::print_table(z);
    println!("Inverse:");
    calculator::inverse::print_table(z);
}

fn parse_bound() -> u32 {
    let bound_str = std::env::args().nth(1).expect("No bound given");
    let bound = bound_str.trim().parse::<u32>().unwrap();
    if bound <= 0 {
        eprintln!("Bound must be positive");
    }

    bound
}
