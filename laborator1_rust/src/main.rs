fn main() {
    for i in 2..100 {
        let result = is_prime(i);
        if result {
            println!("Number {} is prime!", i);
        } else {
            println!("Number {} is not prime!", i);
        }
    }

    for i in 1..100 {
        for j in 1..100 {
            if is_coprime(i, j) {
                println!("{} and {} are coprime!", i, j);
            } else {
                println!("{} and {} are non-coprime", i, j);
            }
        }
    }

    beer_sing(99, 99);
}

fn is_prime(number: i32) -> bool {
    let mut i = 2i32;
    loop {
        if number % i == 0 {
            return false;
        }
        if i >= (number as f64).sqrt() as i32 {
            return true;
        }
        i = i + 1;
    }
}

fn is_coprime(value1: u32, value2: u32) -> bool {
    cmmdc_recursive(value1, value2) == 1
}

fn cmmdc_recursive(value1: u32, value2: u32) -> u32 {
    if value2 != 0 {
        return cmmdc_recursive(value2, value1 % value2);
    }

    return value1;
}

fn beer_sing(beer_count: i32, initial_beer_count: i32) {
    if beer_count > 1 {
        println!("{} bottles of beer on the wall,", beer_count);
        println!("{} bottles of beer.", beer_count);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall.", beer_count - 1);
        println!();
    }
    if beer_count == 1 {
        println!("{} bottle of beer on the wall,", beer_count);
        println!("{} bottle of beer.", beer_count);
        println!("Take one down, pass it around,");
        println!("No bottles of beer on the wall.");
        println!();
    }
    if beer_count == 0 {
        println!("No bottles of beer on the wall,");
        println!("No bottles of beer.");
        println!("Go to the store, buy some more");
        println!("{} bottles of beer on the wall.", initial_beer_count);
    } else {
        beer_sing(beer_count - 1, initial_beer_count);
    }
}
