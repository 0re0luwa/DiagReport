use std::env;
use std::fs::File;
use std::io::{self, BufRead};


//main method responsible for printing all the results of Load diagnostics and chack life support
fn main()
{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2
    {
    	eprintln!("Usage: {} <file_path>", args[0]);
    	return;
    }

    let file_path = &args[1];

    match load_diagnostics(file_path)
    {
        Ok(lines) => {
            for line in &lines {
                println!("{}", line);
            }

        let life_support = check_life_support(lines);
        println!("Loading diagnostics...");
        println!("O2 Generator computed...");
        println!("CO2 Scrubber rate computed...");
        println!("Life Support rate: {}", life_support);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }



}

fn load_diagnostics(file_path: &str) -> io::Result<Vec<String>>
{
    let filehandle = File::open(file_path)?;     // open the file
    let reader = io::BufReader::new(filehandle);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect(); // store the contents of the file in a list

    Ok(lines)

}

fn check_life_support(mut binary_strings: Vec<String>) -> u32
{

	if binary_strings.is_empty()
	{
		return 0;
	}
	
    // clone the list of binary string because it will be used for CO2 computation later
	let binary_strings2 = binary_strings.clone(); 

    // max length
    let max_length_of_binary = binary_strings[0].len();
    
    let mut position = 0;

    // Outer loop to go through the positions of the binary string
    while binary_strings.len() > 1 && position <= max_length_of_binary
    {

        let mut zero = 0;
        let mut one = 0;

        // inner loop to go through each binary string
        for binary_num in &binary_strings
        {
            if let Some(binary_at_position) = binary_num.chars().nth(position){

            	if binary_at_position == '0'
            	{
                	zero += 1;
            	}
            	else if binary_at_position == '1'
            	{
                	one += 1;
            	}
            }
        }


        // Computation for the O2 generator
        if one >= zero
        {
            binary_strings.retain(|s| s.chars().nth(position) == Some('1'));
        }

        else 
        {
            binary_strings.retain(|s| s.chars().nth(position) == Some('0'));
        }
        
        position += 1;

    }

    //Converting from binary to decimal
    let o2 = &binary_strings[0];
    
    let decimal_o2 = u32::from_str_radix(o2, 2).unwrap_or(0);
    
    
	// Reset for CO2 computations
    let mut position = 0;
    
    let mut binary_strings = binary_strings2;
    
    // Outer loop to go through the postions of the binary strin
    while binary_strings.len() > 1 && position <= max_length_of_binary
    {

        let mut zero = 0;
        let mut one = 0;

        // Inner loop to got through each binary string
        for binary_num in &binary_strings
        {
            if let Some(binary_at_position) = binary_num.chars().nth(position){

            	if binary_at_position == '0'
            	{
                	zero += 1;
            	}
            	else if binary_at_position == '1'
            	{
                	one += 1;
            	}
            }
        }

        // Computation for CO2 Scrubber
        if one < zero
        {
            binary_strings.retain(|s| s.chars().nth(position) == Some('1'));
        }

        else 
        {
            binary_strings.retain(|s| s.chars().nth(position) == Some('0'));
        }
        
        position += 1;

    }
    
    // Converting from binary to decimal
    let co2 = &binary_strings[0];

    let decimal_co2 = u32::from_str_radix(co2, 2).unwrap_or(0);

    // Life support rate computation
    decimal_o2 * decimal_co2
}
