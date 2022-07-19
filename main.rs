fn main() {
    const VIN: usize = 16;
    let mut calc_count: u32 = 0;
    let mut result_buffer: Vec<usize> = vec![];
    let mut hex = String::new();
    println!("Enter hex value: ");
    use std::io;

    if let Err(_) = io::stdin().read_line(&mut hex) {
        println!("Big problom!: you didn't enter data properly bro. Learn how to use terminal first before using my program");
    }

    for character in hex.trim().to_lowercase().chars().rev() {
        result_buffer.push(hex_to_denary(character) * usize::pow(VIN, calc_count));
        calc_count += 1;
    }
    let result: usize = result_buffer.into_iter().sum();
    println!("The corresponding decimal value is {result}");
}

fn hex_to_denary(char: char) -> usize {
    match char {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => {
            panic!("{char} is an unrecognized hex character");
            //404
        }
    }
}
