use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let filename:String = get_file_name();
    println!("Reading portfolio file...: {} " , filename);
    

    let lines: &mut Vec<String> = &mut read_to_string_vector(filename);
    println!("Contents \n ");
    for line in lines.iter().clone() {
        println!("{}", line);
    }
    println!("\n a total of {} lines were read", lines.len());

    let (total_value,total_index) = get_total_value_and_line_number(lines);
    let (stock_values,end_of_stock_values_index) = parse_stock_values(lines,total_index);
    println!("Total portfolio value is {}", total_value); 
}

fn get_file_name() -> String {
    let dir = env::current_dir().unwrap();
    let filename = String::from(dir.to_string_lossy()) + "/Y14555975.TXT";
    return filename;
}

fn read_to_string_vector(filename:String) -> Vec<String> {
    let contents:String = fs::read_to_string(filename).expect("uh oh. cannot read file");
    let lines:Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    return lines;
}

fn get_total_value_and_line_number(contents:&mut Vec<String>) -> (f64,usize) {
    let mut line_count : usize = 0;
    let mut total_value: f64 = 0.0;
    let mut total_value_line: &str;
    for (index,line) in contents.iter().enumerate(){
        if line.contains("Toplam Port"){
            total_value_line = &line;
            let splits: Vec<&str> = total_value_line.split_whitespace().filter(|s| !s.is_empty()).collect();
            if splits.len() < 4{
                continue;
            }
            total_value = splits[3].replace(',',"").parse::<f64>().expect("Could not parse the total value content");
            line_count = index;
            break;
        } 
    }
    return (total_value,line_count);
}

fn parse_stock_values(contents:&mut Vec<String>,total_value_index:usize) -> (HashMap<String,f64>,usize){
    let mut last_index= total_value_index;
    let mut stock_values:HashMap<String,f64> = HashMap::new();
    for (index,line) in contents.iter().enumerate(){
        if index < total_value_index { continue;}
    }

    return (stock_values,last_index);
}
