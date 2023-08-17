
use std::io::{Result, Write, Read};
use std::fs::File;

fn write_string_to_file(
  data: &str,
  path: &str
) ->
std::io::Result<()> {
  println!("write_string_to_file");
  let mut file = File::create(path)?;
  file.write_all(data.as_bytes())?;
  Ok(())
}

fn main() -> Result<()> {
    // Initialize a string, named `data`
    let data = "Hello, world!".to_string();

    println!("{}", data.to_string());
    println!("data will be written to {}", "./.out.txt");
    // Create empty file named "output.txt", and
    // write `data` into it (via extenal function).
    write_string_to_file(&data, "./out.txt")?;

    // read the contents of the file
    // and compare the data with `data`
    let mut rfile = File::open("./out.txt")?;
    let mut contents = String::new();
    rfile.read_to_string(&mut contents)?;
    if data == contents.to_string() {
      println!("The data was sucessfully written to the file!");
    } else {
      println!("It looks like the data was not sucessfully written to the file.")
    }
    Ok(())
}
