use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
};

fn main() {
    println!("\n");

    let file_name = "data.dat";

    // open a file in read and write mode. and it will be created
    // if not exist by using OpenOptions
    let data_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(file_name);

    // checking the Result Object in match block
    // it return the file if everything goes true
    // and print the error message and exit the program
    // if there is any error
    let mut data_file = match data_file_result {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error in open {} file: {}", file_name, err);
            panic!("IO Error was occurre ...");
        }
    };

    // write data to the file and check the Result
    // Object as same as befor
    match data_file.write_all(b"I am writing to the file ...") {
        Ok(()) => match data_file.flush() {
            Ok(()) => (),
            Err(err) => eprintln!("error writing data to the file: {err}"),
        },
        Err(err) => panic!("error in writing to the file: {err}"),
    }

    data_file
        .seek(SeekFrom::Start(0))
        .expect("error in set cursor position");

    // create a data as buffer to read data from file
    let mut my_data = String::new();

    // read data from data.dat file and check the Result Object in
    // match block. if everything goes right print the file context and
    // its length. except print the corresponding error and exit the program
    match data_file.read_to_string(&mut my_data) {
        Ok(length) => println!("file content: {my_data}\nlength: {length}"),
        Err(err) => panic!("error in reading from file: {err}"),
    }

    println!("\n The End ... (0.0.1)\n");
}
