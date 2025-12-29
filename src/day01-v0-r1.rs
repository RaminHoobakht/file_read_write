use std::fs::File;
use std::io::{ErrorKind, Write};

fn main() {
    println!("\n");

    let data_file_result = File::open("data.dat");

    let mut data_file = match data_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("data.dat") {
                Ok(fc) => fc,
                Err(err) => panic!("error in create file: {err:?}"),
            },
            _ => panic!("error in open file: {err:?}"),
        },
    };

    match data_file.write_all(b"This is my data to write to the data.dat file ...") {
        Ok(_) => println!("data was wrote to the file successfully ..."),
        Err(err) => eprintln!("error to write to the file: {err:?}"),
    }

    println!("\n The End ...\n");
}
