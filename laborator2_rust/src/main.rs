
fn main() {
    let mut s = String::from("");
    let mut s2 = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_reference_n(&mut s2, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
    println!();
    print!("{}", s2);
    let mut result = String::from("");
    let ref_result = &mut result;

    println!("");

    add_space(ref_result, 40);
    add_str(ref_result,  "I 💚");
    add_str(ref_result,  "\n");
    add_space(ref_result, 40);
    add_str(ref_result,  "RUST.");
    add_str(ref_result,  "\n");
    add_space(ref_result, 4);
    add_str(ref_result,  "Most");
    add_space(ref_result, 12);
    add_str(ref_result,  "crate");
    add_space(ref_result, 6);
    add_integer(ref_result, 306_437_986);
    add_space(ref_result, 11);
    add_str(ref_result,  "and");

    add_space(ref_result, 5);
    add_str(ref_result,  "lastest");

    add_space(ref_result, 9);
    add_str(ref_result,  "is");

    add_str(ref_result,  "\n");
    add_space(ref_result, 9);
    add_str(ref_result,  "downloaded");

    add_space(ref_result, 8);
    add_str(ref_result,  "has");

    add_space(ref_result, 13);
    add_str(ref_result,  "downloads");

    add_space(ref_result, 5);
    add_str(ref_result,  "the");

    add_space(ref_result, 9);
    add_str(ref_result,  "version");

    add_space(ref_result, 4);
    add_float(ref_result, 2.038);

    add_str(ref_result, ".");

    println!("{} ", ref_result);

}

fn add_chars_n(mut value: String, c: char, i: u32) -> String
{
    let mut string_to_concat: String = String::from("");
    for _ in 0..i
    {
        string_to_concat.push(c);
    }
    value.push_str(&string_to_concat);
    return value;
}

fn add_chars_reference_n(value: &mut String, c: char, i: u32)
{
    let mut string_to_concat: String = String::from("");
    for _ in 0..i
    {
        string_to_concat.push(c);
    }
    value.push_str(&string_to_concat);
}

fn get_digit_count(mut value: i32) -> i32
{
    let mut digit_count: i32 = 0;
    while value > 0
    {
        value /= 10;
        digit_count += 1;
    }
    return digit_count;
}

fn add_space(value: &mut String, spaces: u32)
{
    for _ in 0..spaces
    {
        value.push(' ');
    }
}

fn add_str(value: &mut String, input: &str)
{
    value.push_str(&input);
}

fn add_integer(value: &mut String, integer: i32)
{
    let mut digits_added: i32 = 0;
    let number_of_digits: i32 = get_digit_count(integer);
    let base: i32 = 10;
    
    for i in 0..number_of_digits
    {
        if i % 3 == 0 && i > 0
        {
            value.push('_');
        }
        let division_number = i32::pow(base, (number_of_digits - digits_added - 1) as u32);
        digits_added += 1;
        value.push((((integer / division_number) % 10i32) as u8 + ('0' as u8)) as char); 
    }
    let string_size: i32 = value.len() as i32;
    
}

fn add_float(value: &mut String, float: f32)
{
    let string_to_add = float.to_string();
    value.push_str(&string_to_add);
}
