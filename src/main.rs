use std::io::{stdin, stdout, Write};

fn main() -> std::io::Result<()> {
    let mut quit = false;

    loop {
        // write to stdout
        write!(stdout(), "> ")?;
        // flush everything ensuring all content reach destination
        stdout().flush()?;

        let mut buf = String::new();
        // read a line of input and append to the buffer
        // this line will wait for the "Enter" key to be pressed
        stdin().read_line(&mut buf)?;

        if quit {
            break;
        }

        quit = true;
    }

    Ok(())
}
