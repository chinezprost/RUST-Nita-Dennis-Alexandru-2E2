use std::{fs, io, str, collections::HashMap};

fn print_error(error: Error)
{
    match error
    {
        Error::IOError => println!("IOError"),
        Error::NotAscii => println!("NotAscii"),

    }
}
fn main()
{
    read_file_print_longest();
    let cypher_result = rot_13_cipher(String::from("mama se duce la piata\n sa cumpere mere si pere"));
    println!();
    println!("Original Text:➡️\nmama se duce la piata\n sa cumpere mere si pere\n");
    println!("ROT-13 Text:➡️");
    match cypher_result
    {
        Ok(cypher_result) => println!("{}", cypher_result),
        Err(cypher_result) => print_error(cypher_result),
    }
    println!();

    let mut p3_input_file = match fs::read_to_string("input_string.txt")
    {
        Ok(p3_input_file) => p3_input_file,
        Err(_) => String::from("Failed to read from FILE"),
    };
    p3_input_file = match replace_abbrevations(p3_input_file)
    {
        Ok(p3_input_file) => p3_input_file,
        Err(_) => String::from("Failed to read from FILE"),
    };
    println!("{}", p3_input_file);

    p4();

}

enum Error {
    IOError,
    NotAscii,
}

//P1)

fn read_file_print_longest() -> Result<(), Error> {
    let file_open = match fs::read_to_string("file.txt") {
        Ok(file_open) => file_open,
        Err(_) => return Err(Error::IOError),
    };

    let mut longest_line_chars = String::from("");
    let mut longest_line_bytes = String::from("");

    let mut max_bytes = 0usize;
    let mut max_chars = 0usize;

    for readed_line in file_open.lines() {
        if readed_line.len() > max_bytes {
            max_bytes = readed_line.len();
            longest_line_bytes = String::from(readed_line);
        }

        if readed_line.chars().count() > max_chars {
            max_chars = readed_line.chars().count();
            longest_line_chars = String::from(readed_line);
        }
    }

    println!(
        "Max chars line: {} with {} chars.",
        longest_line_chars, max_chars
    );
    println!(
        "Max bytes line: {} with {} bytes.",
        longest_line_bytes, max_bytes
    );

    Ok(())
}

fn is_ascii(value: char) -> bool
{
    if value > 0 as char && value < 127 as char
    {
        return true;
    }
    return false;
}
//p2)
fn rot_13_cipher(input_string: String) -> Result<String, Error>
{
    let mut return_string = String::from("");
    let mut new_char = 0 as char;

    for input_string_char in input_string.chars()
    {
        if !is_ascii(input_string_char)
        {
            return Err(Error::NotAscii);
        }

        if input_string_char >= 'a' && input_string_char <= 'z'
        {
            new_char = ((input_string_char as u8 - 'a' as u8 + 13) % 26 + 'a' as u8) as char;
        }
        else if input_string_char >= 'A' && input_string_char <= 'Z'
        {
            new_char = ((input_string_char as u8 - 'A' as u8 + 13) % 26 + 'A' as u8) as char;
        } 
        else if input_string_char != '\r' as char
        { 
            new_char = input_string_char;
        } 
        return_string.push(new_char);
    }

    Ok(return_string)



}


//p3
fn replace_abbrevations(input_text: String) -> Result<String, Error>
{
    let mut computed_string = String::from("");
    //let mut abbrevations_array: [[String; 32]; 32] = Default::default();
    let mut abbrevations_hashmap: HashMap<String, String> = HashMap::new();



    let abbrevations_file = match fs::read_to_string("file.txt")
    {
        Ok(file_open) => file_open,
        Err(_) => return Err(Error::IOError),
    };
    for lines in abbrevations_file.lines()
    {
        let mut word = String::from("");
        let mut abb = String::from("");
        let mut after_space = 0;
        for chars in lines.chars()
        {
            if chars == ' '
            {
                after_space = 1;
                continue;
            }
            if after_space == 1
            {
                word.push(chars);
            }
            else 
            {
                abb.push(chars);
            }
        }
        abbrevations_hashmap.insert(word, abb);
    }

    for splited_string in input_text.split(" ")
    {
        let mut pushed = 0;
        for(word, abb) in &abbrevations_hashmap
        {
            if splited_string.trim() == word.trim()
            {
                computed_string.push_str(abb);
                pushed = 1;
                break;
            }
        }
        if pushed == 0
        {
            computed_string.push_str(splited_string);
        }

        computed_string.push(' ');
    }

    Ok(computed_string)

}


fn p4() -> Result<(), Error>
{
    let host_file = match fs::read_to_string(r"C:\Windows\System32\drivers\etc\hosts")
    {
        Ok(host_file) => host_file,
        Err(_) => return Err(Error::IOError),
    };

    for lines in host_file.lines()
    {
        let mut index = 0;
        
        if lines.len() > 0 && lines.chars().next().unwrap() == '#'
        {
            continue;
        }
        for words in lines.split(" ")
        {
            if index == 0
            {
                print!("{} => ", words);
            }
            else 
            {
                println!("{}", words);
            }
            index += 1;
            if index == 2
            {
                break;
            }
        }
    }

    Ok(())

}

