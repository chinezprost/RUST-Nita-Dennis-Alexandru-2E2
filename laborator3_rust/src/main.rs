#[warn(non_snake_case)]
enum Error
{
    Overflow,
    NonPositive,
}

enum ErrorChar
{
    NotASCII,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable
}



fn main()
{
    let number_to_be_checked:u16 = 65000;
    let mut option_result = next_prime(number_to_be_checked);
    while let Some(i) = option_result
    {
        println!("Bigger prime: {}", i);
        option_result = next_prime(i);
    }

    let number1 = 357913941;
    let number2 = 13;
    let mut result = check_addition_result_error(number1, number2);
    match result
    {
        Ok(result) => println!("{}", result),
        Err(result) => print_error2(result),
    }
    result = check_multiplication_result_error(number1, number2);
    match result
    {
        Ok(result) => println!("{}", result),
        Err(result) => print_error2(result),
    }

    let random_char = 250 as char;

    
    let mut result2 = to_uppercase(random_char);
    
    match result2
    {
        Ok(result2) => println!("{}", result2),
        Err(result2) => print_error(result2)
    }

    check_addition_panic(2, 3);
    check_multiplication_panic(3, 42);

    result2 = to_lowercase('D');
    match result2
    {
        Ok(result2) => println!("{}", result2),
        Err(result2) => print_error(result2)
    }
    let mut result3 = char_to_number_hex('F');
    match result3
    {
        Ok(result3) => println!("{}", result3),
        Err(result3) => print_error(result3)
    }
    result3 = char_to_number('5');
    match result3
    {
        Ok(result3) => println!("{}", result3),
        Err(result3) => print_error(result3)
    }
    let result4 = print_char('B');
    match result4
    {
        Ok(_) => (),
        Err(result4) => print_error(result4)
    }
    let result5 = check_perfect_square(64 as f64);
    match result5
    {
        Ok(result5) => println!("64 is p.p: {}", result5),
        Err(result5) => print_error2(result5)
    }

   


    

}

fn check_perfect_square(value: f64) -> Result<bool, Error>
    {
        if value < 0 as f64
        {
            return Err(Error::NonPositive);
        }

        let result = ((f64::sqrt(value) as i64) * (f64::sqrt(value) as i64)) as i64 == value as i64;
        Ok(result)
    }

//1)
fn is_prime(value: u16) -> bool
{
    let mut div: u16 = 16;
    while div < value 
    {
        if value % div == 0
        {
            return false;
        }
        div += 1;
    }
    return true;
}


fn next_prime(x: u16) -> Option<u16>
{
    let mut y: u16 = x;
    loop 
    {
        if y == std::u16::MAX
        {
            return None;
        }
        else 
        {
            if y != x && is_prime(y)
            {
                return Some(y);
            }
            else
            {
                y += 1;    
            }
        }
    }
}


//2)
fn check_addition_panic(value1: u32, value2: u32) -> u32
{
    let result: u64 = (value1 + value2) as u64;
    if result > std::u32::MAX as u64
    {
        println!("Couldn't add! Result bigger than u32! Panicking!");
        panic!();
    }
    

    return result as u32;
}

fn check_multiplication_panic(value1: u32, value2: u32) -> u32
{
    let result: u64 = (value1 * value2) as u64;
    if result > std::u32::MAX as u64
    {
        println!("Couldn't multiply! Result bigger than u32! Panicking!");
        panic!();
    }
    
    return result as u32;
}

//3)
fn check_addition_result_error(value1: u32, value2: u32) -> Result<u32, Error>
{
    let result: u64 = (value1 + value2) as u64;
    if result > std::u32::MAX as u64
    {
        return Err(Error::Overflow);
    }
    

    Ok(result as u32)
}

fn check_multiplication_result_error(value1: u32, value2: u32) -> Result<u32, Error>
{
    let result: u64 = (value1 as u64) * (value2 as u64);
    if result > std::u32::MAX as u64
    {
        return Err(Error::Overflow);
    }
    
    Ok(result as u32)
}

// fn is_letter(value: char) -> bool
// {
//     if (value >= 'a' && value <= 'z') || (value >= 'A' && value <= 'Z')
//     {
//         return true;
//     }
//     return false;
// }

fn is_uppercase(value: char) -> bool
{
    if value >= 'A' && value <= 'Z'
    {
        return true;
    }
    return false;
}

fn is_lowercase(value: char) -> bool
{
    if value >= 'a' && value <= 'z'
    {
        return true;
    }
    return false;
}

fn to_uppercase(value: char) -> Result<char, ErrorChar>
{
    if !is_lowercase(value)
    {
        return Err(ErrorChar::NotLetter);
    }
    Ok((value as u8 + 32) as char)

}

fn to_lowercase(value: char) -> Result<char, ErrorChar>
{
    if !is_uppercase(value)
    {
        return Err(ErrorChar::NotLetter);
    }
    Ok((value as u8 - 32) as char)
}

fn print_char(value: char) -> Result<(), ErrorChar>
{
    if value < 32 as char
    {
        return Err(ErrorChar::NotPrintable);
    }
    println!("Char is: {}", value);
    Ok(())
}

fn char_to_number(value: char) -> Result<u8, ErrorChar>
{
    if value < 0 as char || value > 127 as char
    {
        return Err(ErrorChar::NotASCII)
    }

    if value < '0' || value > '9'
    {
        return Err(ErrorChar::NotDigit)
    }

    Ok(value as u8)
}

fn char_to_number_hex(value: char) -> Result<u8, ErrorChar>
{
    if value < 0 as char || value > 127 as char
    {
        return Err(ErrorChar::NotASCII)
    }

    if value < '0' || (value > '9' && value < 'A') || value > 'F'
    {
        return Err(ErrorChar::NotBase16)
    }

    Ok(value as u8)
}

fn print_error(error: ErrorChar)
{
    match error
    {
        ErrorChar::NotASCII => println!("Not ASCII Error"),
        ErrorChar::NotDigit => println!("Not Digit Error"),
        ErrorChar::NotBase16 => println!("Not Base16 Error"),
        ErrorChar::NotLetter => println!("Not Letter Error"),
        ErrorChar::NotPrintable => println!("Not Printable Error")
    }
}

fn print_error2(error: Error)
{
    match error
    {
        Error::Overflow => println!("Overflow!"),
        Error::NonPositive => println!("Number is negative!")
    }
}



