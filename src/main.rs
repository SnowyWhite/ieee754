use converter::Ieee754;
use std::io::{self};

mod converter;

fn main() -> io::Result<()> {
    println!("Please enter a float number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let f: f32 = input.trim().parse().unwrap();

    let mut container: Ieee754 = Ieee754::new(f);

    let predecimal: String = container.convert_predecimalplace_to_binary(Option::None);

    let decimal: String = container.convert_decimalplace_to_binary();

    let binary_str: String = predecimal + "." + &decimal;
    println!("Binary representation:\t{binary_str}");

    let normalized_str: String = container.normalize(binary_str);
    println!("Normalized:\t\t{normalized_str}");

    let bias: String = container.get_bias();
    println!("Calculated bias:\t{bias}");

    let ieee: String = container.get_ieee754(normalized_str, bias);
    println!("IEEE754 representation:\t{ieee}");

    Ok(())
}
