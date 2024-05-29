use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cmp;

fn main() -> Result<(), &'static str>{
    match day_three_part_two() {
	Ok(s) => println!("{s:?}"),
	Err(e) => println!("Error in calculation. {e:?}")
    }
    Ok(())
}

fn day_one_part_one() -> Result<String, String> {
    let file:io::Result<File> = File::open("puzzle-input/Day1");
    let mut contents = String::new();

    match file {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }  


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

fn day_one_part_two() -> Result<String, String> {
    let file:io::Result<File> = File::open("puzzle-input/Day1");
    let mut contents = String::new();

    match file {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }  

    
//	Ok(f) => f.read_to_string(&mut contents)
    //	Err(e) => contents.replace_range(.., "")

//    let contents = String::from("R8, R4, R4, R8");
    
    let directions = contents.split(", ").collect::<Vec<&str>>();

    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut xdir:i32 = 0;
    let mut ydir:i32 = 1;

    let mut temp:i32;
    
    let mut turn: char;
    let mut val: String;

    let mut positions:Vec<(i32,i32)> = Vec::new();

    positions.push((0,0));
    
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

	for _ in 0..move_value {
	    x = x + xdir;
	    y = y + ydir;

	    let pos = (x,y);
	    if positions.contains(&pos) {
		let d = x.abs() + y.abs();
		//println!("{:?}", positions);
		return Ok(format!("Distance is {d}"));
	    }
	    positions.push(pos);
	}
	
	
	
    }
    Err("position not found".to_string())
}

fn day_two_part_one() -> Result<String, String> {
    let file:io::Result<File> = File::open("puzzle-input/Day2");
    let mut contents = String::new();
    match file {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }

    let lines = contents.lines();

    let mut keys:Vec<i32> = Vec::new();

    let keypad:Vec<Vec<i32>> = vec![vec![1,4,7],vec![2,5,8],vec![3,6,9]];

    fn get_key(v:&Vec<Vec<i32>>,x:usize,y:usize) -> i32 {
	match v.get(x) {
	    Some(v2) => {
		match v2.get(y) {
		    Some(val) => *val,
		    None => -1
		}
	    },
	    None => -1
	}
    }

    
    let mut x:usize = 1;
    let mut y:usize = 1;
    for l in lines {
	
	for c in l.chars() {
	    match c {
		'U' => {
		    if y != 0 {
			y = y - 1;
		    }
		},
		'D' => y = cmp::min(y + 1,2),
		'L' => {
		    if x != 0 {
			x = x - 1
		    }
		},
		'R' => x = cmp::min(x + 1,2),
		_   => (),

	    }
//	    println!("x: {x}, max x or 2: {m}, y:{y}");
	}
//	println!("push {x}{y}");
	keys.push(get_key(&keypad, x, y));
    }
    
    return Ok(format!("{keys:?}"));
}

fn day_two_part_two() -> Result<String, String> {
    let file:io::Result<File> = File::open("puzzle-input/Day2");
    let mut contents = String::new();
    match file {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }

    let lines = contents.lines();

    let mut keys:Vec<char> = Vec::new();

    let keypad:Vec<Vec<char>> = vec![vec!['!','!','5','!','!'],vec!['!','2','6','A','!'],vec!['1','3','7','B','D'],vec!['!','4','8','C','!'],vec!['!','!','9','!','!']];

    fn get_key(v:&Vec<Vec<char>>,x:usize,y:usize) -> char {
	match v.get(x) {
	    Some(v2) => {
		match v2.get(y) {
		    Some(val) => *val,
		    None => '!'
		}
	    },
	    None => '!'
	}
    }

    
    let mut x:i32 = -2;
    let mut y:i32 = 0;

    let mut old_x:i32;
    let mut old_y:i32;
    
    for l in lines {
	
	for c in l.chars() {
	    old_x = x;
	    old_y = y;
	    match c {
		'U' => y = y - 1,
		'D' => y = y + 1,
		'L' => x = x - 1,
		'R' => x = x + 1,
		_   => (),
	    }
	    if x.abs() + y.abs() > 2 {
		x = old_x;
		y = old_y;
	    }
//	    println!("x: {x}, max x or 2: {m}, y:{y}");
	}
//	println!("push {x}{y}");
	keys.push(get_key(&keypad, (x+2) as usize, (y+2) as usize));
    }
    let keys = keys.iter().cloned().collect::<String>();
    return Ok(keys);
}

fn day_three_part_one() -> Result<String, String> {
    let mut contents = String::new();
    match File::open("puzzle-input/Day3") {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }
    
    let mut valid:i32 = 0; 
    
    for l in contents.lines() {
	let numbers:Vec<i32> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();

	let v1:i32;
	let v2:i32;
	let v3:i32;
	
	match numbers.get(0) {
	    Some(x) => v1 = *x,
	    None    => v1 = -100
	}

	match numbers.get(1) {
	    Some(x) => v2 = *x,
	    None    => v2 = -100
	}

	match numbers.get(2) {
	    Some(x) => v3 = *x,
	    None    => v3 = -100
	}

//	println!("v1: {v1}, v2: {v2}, v3: {v3}");
	
	if (v1 < v2 + v3) && (v2 < v1+v3) && (v3 < v1 + v2) {
	    valid = valid + 1;
	}
    }
    return Ok(valid.to_string());
}
fn day_three_part_two() -> Result<String, String> {
    let mut contents = String::new();
    match File::open("puzzle-input/Day3") {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }

    let contents:Vec<Vec<i32>> = contents.lines().map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect();
    let num_triples = contents.len()/3;

    let mut triples:Vec<Vec<i32>> = Vec::new();

    for i in 0..3 {
	for j in 0..num_triples {
	    let start = j * 3;
	    triples.push(
		vec![
		    *contents.get(start).unwrap().get(i).unwrap(),
		    *contents.get(start + 1).unwrap().get(i).unwrap(),
		    *contents.get(start + 2).unwrap().get(i).unwrap()
		]
	    );
	}
    }

    let mut valid = 0;
    let length = triples.len();
  //  println!("Num triangles to check is: {length}");
    for triplet in triples {
//	println!("Checking triplet: {triplet:?}");
	let v1 = *triplet.get(0).unwrap();
	let v2 = *triplet.get(1).unwrap();
	let v3 = *triplet.get(2).unwrap();

	if (v1 < v2 + v3) && (v2 < v1 + v3) && (v3 < v1 + v2) {
	    valid = valid + 1;
	}
    }
    
    return Ok(valid.to_string());
}
