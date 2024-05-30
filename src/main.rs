use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::cmp;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() -> Result<(), &'static str>{
    match day_four_part_two() {
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


    let mut valid = 0;
    for i in 0..3 {
		for j in 0..num_triples {
	    	let start = j * 3;

			let v1 = *contents.get(start).unwrap().get(i).unwrap();
			let v2 = *contents.get(start + 1).unwrap().get(i).unwrap();
			let v3 = *contents.get(start + 2).unwrap().get(i).unwrap();

			if (v1 < v2 + v3) && (v2 < v1+v3) && (v3 < v1 + v2) {
				valid = valid + 1;
			}
		}
	
    }    
    return Ok(valid.to_string());
}

fn day_four_part_one() -> Result<String, String> {
    let mut contents = String::new();
    match File::open("puzzle-input/Day4") {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }

    let mut sum:i32 = 0;
    for line in contents.lines() {
	//	let l:String = line.chars().filter(|c| (*c != '-') && (*c != ']')).collect::<String>();
	let l:String = line.to_string();
	
	let mut parts = l.split("[");

	let mut encrypted_name:Vec<String> = parts.next().unwrap().split("-").map(|s| s.to_string()).collect();

	let sector_id:i32 = encrypted_name.pop().expect("read sector id").parse().unwrap();

	let encrypted_name:String = encrypted_name.join("");
	let encrypted_name = encrypted_name.chars().filter(|c| (*c != '-')).collect::<String>();
	
	let checksum:Vec<char> = parts.next().unwrap().chars().collect();

	let mut dict:HashMap<char, i32> = HashMap::new();

	for c in encrypted_name.chars() {
	    *dict.entry(c).or_insert(0) +=1;	
	}

	let mut counting_vector:Vec<_> = dict.iter().map(|(a,b)| (a, b)).collect();

	fn my_compare((c1,v1):&(&char, &i32),(c2,v2):&(&char,&i32) ) -> Ordering {
	    match v1.cmp(v2) {
		Ordering::Greater => Ordering::Less,
		Ordering::Less => Ordering::Greater,
		Ordering::Equal => c1.cmp(c2)		    
	    }
	}
	
	counting_vector.sort_by(my_compare);
//	print!("{counting_vector:?}, ");
	let counting_vector = counting_vector.into_iter().map(|(a, _)| *a).collect::<Vec<char>>();

	if counting_vector.len() >= 5 {
	    let mut valid = true;
	    for i in 0..5 {
		if counting_vector[i] != checksum[i] {
		    valid = false;
		}
	    }
	    if valid {
		sum += sector_id;
	    }	    
	}
	
//        println!("{counting_vector:?}");
    }

    return Ok(format!("{sum}"));
}

fn day_four_part_two() -> Result<String, String> {
    let mut contents = String::new();
    match File::open("puzzle-input/Day4") {
	Ok(mut f) => _ = f.read_to_string(&mut contents),
	Err(e) => return Err(format!("Error with file opening. {e:?}"))	    
    }

    fn get_true_name(encrypted_name:String, sector_id:i32) -> String {
	let shift = sector_id % 26;
	fn shift_char(c:char, shift:i32) -> char {
	    if c == '-' {
		return ' ';
	    } else {
		
		return char::from_u32(((((c as u32) + (shift as u32) - ('a' as u32))) % 26) + ('a' as u32)).unwrap();  
	    }
	}
	return encrypted_name.chars().map(|c| shift_char(c,shift)).collect::<String>();
    }

//    let name = get_true_name("qzmt-zixmtkozy-ivhz".to_string(),343);
//    println!("{name}");
//    return Err("Uh oh".to_string());
    
    for line in contents.lines() {
	//	let l:String = line.chars().filter(|c| (*c != '-') && (*c != ']')).collect::<String>();
	let l:String = line.to_string();
	
	let mut parts = l.split("[");
	

	let mut encrypted_name:Vec<String> = parts.next().unwrap().split("-").map(|s| s.to_string()).collect();

	let sector_id:i32 = encrypted_name.pop().expect("read sector id").parse().unwrap();

	let original_encrypted_name:String = encrypted_name.join("-");
	let encrypted_name = original_encrypted_name.chars().filter(|c| (*c != '-')).collect::<String>();
	
	let checksum:Vec<char> = parts.next().unwrap().chars().collect();

	let mut dict:HashMap<char, i32> = HashMap::new();

	for c in encrypted_name.chars() {
	    *dict.entry(c).or_insert(0) +=1;	
	}

	let mut counting_vector:Vec<_> = dict.iter().map(|(a,b)| (a, b)).collect();

	fn my_compare((c1,v1):&(&char, &i32),(c2,v2):&(&char,&i32) ) -> Ordering {
	    match v1.cmp(v2) {
		Ordering::Greater => Ordering::Less,
		Ordering::Less => Ordering::Greater,
		Ordering::Equal => c1.cmp(c2)		    
	    }
	}
	
	counting_vector.sort_by(my_compare);
	//	print!("{counting_vector:?}, ");
	
	let counting_vector = counting_vector.into_iter().map(|(a, _)| *a).collect::<Vec<char>>();

	if counting_vector.len() >= 5 {
	    let mut valid = true;
	    for i in 0..5 {
		if counting_vector[i] != checksum[i] {
		    valid = false;
		}
	    }
	    if valid {
		let true_name = get_true_name(original_encrypted_name, sector_id);
		println!("{true_name}: {sector_id}")
	    }	    
	}
	
//        println!("{counting_vector:?}");
    }

    return Ok(format!("Done"));

    
    
}
