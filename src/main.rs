use std::env;
use std::fs;

fn main() {
    let filename:String = get_file_name();
    println!("Reading portfolio file...: {} " , filename);
    

    let lines:Vec<String> = read_to_string_vector(filename);
    println!("Contents \n ");
    for line in lines.iter().clone() {
        println!("{}", line);
    }
    println!("\n a total of {} lines were read", lines.len());
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

fn get_total_value_and_line_number(contents:Vec<String>) -> (f64,i32) {
}
