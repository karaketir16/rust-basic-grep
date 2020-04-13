use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {
    let args: Vec<String> = env::args().collect();
    finder(&args[1], (&args[2..]).to_vec());
}

fn finder(s: &String, filenames: Vec<String>) {
    for filename in filenames.iter(){
        if let Ok(lines) = read_lines(filename) {
            for (lineNumber, line) in lines.enumerate() {
                if let Ok(ln) = line {
                    if ln.contains(&s[..]) {
                        println!("{}:{} {}",filename, lineNumber + 1, ln);
                    }
                }
            }
        }
    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}