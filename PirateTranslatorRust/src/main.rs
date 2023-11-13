use std::io;

const VOWELS:&str = "aouåeiyäö";

fn is_vowel(c:char) -> bool
{
	return VOWELS.to_lowercase().contains(c);
}

fn main() 
{
	let mut _answer:String = String::new();
	loop 
	{
		print!("\x1B[2J\x1B[1;1H");
		
		println!("Yarr ohoy matey! Welocme to me pirate translator!");
		println!("Do be typey '.exit' to leave me program! Yarr");
		println!("Input: ");
		
		_answer = get_user_input();
		
		if _answer.trim_end().len() > 0
		{
			if _answer.to_lowercase().trim_end() == ".exit"
			{
				break;
			}
			
			let mut _out = translate(_answer);
			println!("{_out}");
		}
		else 
		{
			println!("Oy, put something in me translator!");
		}
		
		// Pause the current itteration
		get_user_input();
	}
	
	println!("Good bye, matey!");
	
}

fn translate(string_to_translate: String) -> String
{
	let mut temp_output = String::new();
	
	for c in string_to_translate.chars()
	{
		if is_vowel(c) || !c.is_alphabetic()
		{
			temp_output.push(c);
			continue;
		}
		
		// This is not very nicely done, but I can't get rust to accept my charactes into strings.
		if c.is_uppercase()
		{
			temp_output.push(c);
			temp_output.push('O');
			temp_output.push(c);
		}
		else 
		{
			temp_output.push(c);
			temp_output.push('o');
			temp_output.push(c);
		}		
	}
	
	return temp_output;
}

fn get_user_input() -> String 
{
	let mut _input:String = String::new();
	
	match io::stdin().read_line(&mut _input) 
	{
		Ok(_n) => 
		{
			return _input;
		}
		Err(error) => 
		{
			println!("error: {error}");
			return String::new();
		}
	}
}