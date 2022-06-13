use converter::Ieee754;
use std::io::{self};

mod converter;

fn main() -> io::Result<()> {
    println!("Please enter a float number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let float_input: f32 = input.trim().parse().unwrap();

    let mut binary_format: Ieee754 = Ieee754::new(float_input);

    let pre_decimalplace: String =
        binary_format.convert_pre_decimalplace_to_binary_string(Option::None);

    let decimalplace: String = binary_format.convert_decimalplace_to_binary_string();

    let binary_str: String = pre_decimalplace + "." + &decimalplace;
    println!("Binary representation:\t{binary_str}");

    let normalized_str: String = binary_format.normalize(binary_str);
    println!("Normalized:\t\t{normalized_str}");

    let bias: String = binary_format.get_bias();
    println!("Calculated bias:\t{bias}");

    let ieee: String = binary_format.get_ieee754(normalized_str, bias);
    println!("IEEE754 representation:\t{ieee}");

    Ok(())
}
