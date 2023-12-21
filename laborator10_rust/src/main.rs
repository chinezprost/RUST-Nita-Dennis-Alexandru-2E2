use std::collections::HashMap;

use std::cell::RefCell;

struct Cache 
{
    cache_object: RefCell<HashMap<u64, bool>>,
}

impl Cache {
    fn new() -> Self 
    {
        Cache 
        {
            cache_object: RefCell::new(HashMap::new())
        }
    }

    fn is_value_prime(&self, _value: u64) -> bool 
    {
        if _value < 2 
        {
            return false;
        }

        for index in 2..=(_value as f64).sqrt() as u64 
        {
            if _value % index == 0 
            {
                return false;
            }
        }

        return true;
    }

    fn process(&self, _value: u64) -> (bool, bool) 
    {
        let mut cache_object = self.cache_object.borrow_mut();

        if let Some(&result_var) = cache_object.get(&_value) 
        {
            return (result_var, true);
        }
        else 
        {
            let result_var = self.is_value_prime(_value);
            cache_object.insert(_value, result_var);

            return (result_var, false);
        }
    }
}

fn main() 
{
    let cache_object = Cache::new();

    while true // or loop, but to be honest i think i like while true more
    {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Error: Couldn't read line.");

        let _value: u64 = match input.trim().parse() 
        {
            Ok(_value) => _value,
            Err(_) => 
            {
                println!("Invalid input.");
                continue;
            }
        };

        let (result_var, from_cache) = cache_object.process(_value);

        if result_var 
        {
            println!("{} is a prime number! You can enter another value:", _value);
        } 
        else
        {
            println!("{} is a non-prime number! You can enter another value:", _value);
        }

        if from_cache
        {
            println!("Result is cached.");
        }
        else
        {
            println!("Value wasn't cached.");
        }
    }
}