use std::fs;
use serde_derive::Deserialize;
use anyhow::Result;
#[derive(Deserialize)]


struct Student {
    student_name: String,
    student_phone: String,
    student_age: u8,
}

fn problem_1() -> Result<(), anyhow::Error> {
    let readed_string = fs::read_to_string("input.csv")?;
    if readed_string.is_empty() {
        return Err(anyhow::Error::msg("Empty file"))
    }

    let mut oldest: Student = Student {
        student_name: String::new(),
        student_phone: String::new(),
        student_age: 0,
    };
    let mut youngest: Student = Student {
        student_name: String::new(),
        student_phone: String::new(),
        student_age: 255,
    };

    for line in readed_string.lines() {
        let mut aux: Student = Student {
            student_name: String::new(),
            student_phone: String::new(),
            student_age: 0,
        };
        let mut iterator2 = 0;
        for p in line.split(",") {
            if iterator2 == 0 {
                aux.student_name = p.to_string();
            } else if iterator2 == 1 {
                aux.student_phone = p.to_string();
            } else if iterator2 == 2 {
                for c in p.chars() {
                    aux.student_age = aux.student_age * 10 + (c as u8 - '0' as u8);
                }
            }
            iterator2 += 1;
        }
        if aux.student_age > oldest.student_age 
        {
            oldest = aux;
        } else if aux.student_age < youngest.student_age 
        {
            youngest = aux;
        }
    }


    println!("Oldest student is {} with age {}", oldest.student_name, oldest.student_age);
    println!("Youngest student is {} with age {}",youngest.student_name, youngest.student_age);
    
    Ok(())
}

struct Canvas {
    canvas_width:u32,
    canvas_height:u32,
    canvas_object:[[char;50];10]
}

fn new_canvas() -> Canvas {
    let canvas_object = [[' ';50];10];
    Canvas {
        canvas_width: 50,
        canvas_height: 10,
        canvas_object: canvas_object,
    }
}

fn set_pixels(canvas: &mut Canvas, pixels: &[(u32, u32, u8)]) {
    for (x, y, c) in pixels {
        canvas.canvas_object[*x as usize][*y as usize] = c.clone() as char;
    }
}

fn print(canvas: Canvas) {
    for iterator in 0..canvas.canvas_height {
        for iterator2 in 0..canvas.canvas_width {
            print!("{}", canvas.canvas_object[iterator as usize][iterator2 as usize]);
        }
        println!();
    }
}

fn problem_3() -> Result<(), anyhow::Error> {
    let readed_string = fs::read_to_string("input.json")?;
    if readed_string.is_empty() 
    {
        return Err(anyhow::Error::msg("Error: File is empty!"));
    }

    let mut oldest: Student = Student 
    {
        student_name: String::new(),
        student_phone: String::new(),
        student_age: 0,
    };
    let mut youngest: Student = Student 
    {
        student_name: String::new(),
        student_phone: String::new(),
        student_age: 255,
    };

    for line in readed_string.lines() 
    {
        let deser_json: Student = serde_json::from_str(line)?;
        if deser_json.student_age > oldest.student_age {
            oldest = deser_json;
        } else if deser_json.student_age < youngest.student_age {
            youngest = deser_json;
        }
    }
    println!("Oldest student is {} with age {}", oldest.student_name, oldest.student_age);
    println!("Youngest student is {} with age {}", youngest.student_name, youngest.student_age);
    
    Ok(())
}

fn main() {
   let r :Result<(), anyhow::Error> = problem_1();
   if r.is_err(){
       println!("Error: {}",r.err().unwrap());
   }
    let mut canvas = new_canvas();
    let c = &mut canvas;
    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);
    print(canvas);
    let r1: Result<(), anyhow::Error> = problem_3();
   if r1.is_err()
   {
       println!("Error: {}", r1.err().unwrap());
   }
}