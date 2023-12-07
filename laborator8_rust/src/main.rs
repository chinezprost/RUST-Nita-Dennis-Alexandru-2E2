use std::{fs, io, str, collections::HashMap, cmp, io::Write};

fn main() {
    solve_problem();
}



fn solve_problem()
{
    let input_file = match fs::read_to_string("input_file.txt")
    {
        Ok(input_file) => input_file,
        Err(_) => String::from("Can't read from file.")
    };

    let mut max_length = 0;

    let mut map: HashMap<String, i32> = HashMap::new();

    for i in input_file.split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace())
    {
        if i.is_empty()
        {
            continue;
        }
        max_length = cmp::max(max_length, i.len() as u32);
        *map.entry(i.to_string().to_lowercase()).or_default() += 1;
    }

    let mut sort_vec: Vec<(&String, &i32)> = map.iter().collect();
    sort_vec.sort_by(|x, y| x.1.cmp(y.1));

    for(key, value) in sort_vec.iter().rev()
    {
        match write!(std::io::stdout(),"{:width$} => {}\n", key, value, width = max_length as usize)
        {
            Ok(_) => (),
            Err(error) => eprintln!("Can't write to stdout: {}", error),
        }
    }
}