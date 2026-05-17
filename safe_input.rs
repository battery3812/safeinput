use std::io;

pub fn safegets() -> Result<String, std::io::Error> {
    let mut input_buffer = String::new();

    match io::stdin().read_line(&mut input_buffer) {
        Ok(_) => Ok(input_buffer.trim().to_string()),
        Err(error) => Err(error),
    }
}

pub fn safeprint(hello: &Result<String, std::io::Error>) {
    match hello {
        Ok(str) => println!("{str}"),
        Err(error) => println!("Error: {}", error),
    };
}

pub fn repeat() {
    safeprint(&safegets());
}
