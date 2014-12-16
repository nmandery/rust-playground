use std::io;

fn main(){
    if io::stdio::stdin_raw().isatty(){
        println!("Not pipe");
    }
    else{
        let mut reader = io::stdin();
        loop{
            match reader.read_line() {
                Ok(txt) => println!("Read: {}", txt),
                Err(e) => match e.kind{
                    io::EndOfFile => break,
                    _ => {
                        panic!("ERROR: {}", e)
                    }
                }
            }
        }
    }
}
