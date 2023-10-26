use std::fs::File;
use std::io;
use std::io::Read;

// fn read_uname_from_file(fname: &str) -> Result<String, io::Error> {
//     let mut file = match File::open(fname) {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut uname = String::new();
//     let result = match file.read_to_string(&mut uname) {
//         Ok(_) => Ok(uname),
//         Err(e) => return Err(e),
//     };
//     result
// }

fn read_uname_from_file(fname: &str) -> Result<String, io::Error> {
    let mut uname = String::new();
    File::open(fname)?.read_to_string(&mut uname);
    Ok(uname)
}

fn main() {
    // let file = File::open("1.txt").unwrap();
    // let file = File::open("1.txt").expect("not found");
    let uname = read_uname_from_file("1.txt").unwrap();
    println!("{uname}");
}
