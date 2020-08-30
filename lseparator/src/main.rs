use std::io;
use std::time::SystemTime;

fn main() {
    let mut input = String::new();

    loop {
        let last = SystemTime::now();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let elapsed = last.elapsed().unwrap().as_millis();

                if elapsed > 500 {
                    println!("-------------------------");
                }

                print!("got: {}", input);

                input.clear();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
