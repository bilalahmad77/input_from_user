use std::io;
fn main() {
    println!("Enter any data");
    let mut inputdata=String::new(); //will be saved on heap
    io::stdin().read_line(&mut inputdata).expect("Failed to read line");
    let int_data:u32=inputdata.trim().parse().expect("Please type a number!");
    println!("Converted Integer:{:?}",int_data);
}