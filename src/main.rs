use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() -> Result<(), &'static str>{

    let result = day_one_part_one();
    match result {
	Ok(s) => println!("{s:?}"),
	Err(e) => println!("Error in calculation. {e:?}")
    }
    Ok(())
}

fn day_one_part_one() -> Result<String, String> {
    let file:io::Result<File> = File::open("puzzle-input/Day1");
    let mut contents = String::new();

    if let Ok(mut f) = file {
	let _ = f.read_to_string(&mut contents);
    } else if let Err(file_error) = file{
	let mut error:String = String::from("Error with file opening");
	error.push_str(&(file_error.to_string()));
	return Err(error);
    }
    
//	Ok(f) => f.read_to_string(&mut contents)
    //	Err(e) => contents.replace_range(.., "")    
    let directions = contents.split(", ").collect::<Vec<&str>>();

    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut xdir:i32 = 0;
    let mut ydir:i32 = 1;

    let mut temp:i32;
    
    let mut turn: char;
    let mut val: String;
    for direction in directions {
	turn = direction.as_bytes()[0] as char;
	val = direction.to_string();
	val.remove(0);
	if turn == 'L' {
	    temp = xdir;
	    xdir = -ydir;
	    ydir = temp;
	} else {
	    temp = xdir;
	    xdir = ydir;
	    ydir = -temp;
	}
	let move_value:i32;
	let result =  val.parse::<i32>();
	if let Ok(x) = result {
	    move_value = x as i32;
	} else {
	    return Err("error reading move".to_string());
	}

	x = x + xdir * move_value;
	y = y + ydir * move_value;
    }
    let length:i32 = x.abs() + y.abs();
    Ok(length.to_string())
}
