use std::{fs::File, io::ErrorKind, net::AddrParseError};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match &greeting_file_result {
    //     Ok(file) => {
    //         println!("{:#?}", file);
    //     }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => println!("File doesn't exists so File is Created"),
    //             Err(error) => println!(
    //                 "File doesn't exists, error occured while creating the file: {error:?}"
    //             ),
    //         },
    //         _ => println!("Some other error occured while fetching the file: {error:?}"),
    //     },
    // };

    let greeting_file = &mut greeting_file_result.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.txt").unwrap_or_else(|error| {
                panic!("{error:?}");
            })
        } else {
            panic!("{error:?}");
        }
    });

    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1..2"
        .parse()
        .expect("Hardcoded IP address should be valid");
}
