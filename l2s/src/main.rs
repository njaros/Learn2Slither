use playground::{PlayGround, Dir};
use std::error::Error;
use std::io::{stdout, stdin, Read, Write};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use trie::Trie;

/// Return new termios, old termios.
fn set_terminal() -> Result<(Termios, Termios), Box<dyn Error>> {
    let old_termios = Termios::from_fd(0)?;
    let mut new_termios = old_termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut new_termios).unwrap();
    Ok((new_termios, old_termios))
}

fn init_inputs() -> Trie {
    Trie::from(&vec![
        (vec![27, 91, 67], Dir::Right),
        (vec![27, 91, 65], Dir::Up),
        (vec![27, 91, 66], Dir::Down),
        (vec![27, 91, 68], Dir::Left),
        (vec![119], Dir::Up),
        (vec![100], Dir::Right),
        (vec![115], Dir::Down),
        (vec![97], Dir::Left)
    ])
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut playground = PlayGround::new(10, 10);
    let mut buffer = [0u8; 1];
    let stdout = stdout();
    let mut reader = stdin();
    let (mut new_termios, old_termios) = set_terminal()?;
    let mut trie = init_inputs();
    // println!("{playground}");
    playground.print_snake_view();
    while buffer[0] != 113 && playground.is_alive() {
        reader.read(&mut buffer)?;
        match trie.seek(buffer[0]) {
            None => {},
            Some(dir) => {
                playground.next(dir);
                playground.print_snake_view();
            }
        }
    }
    println!("{playground}");
    tcsetattr(0, TCSANOW, &old_termios).unwrap();
    
    Ok(())
}
