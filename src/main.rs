use lazy_static::lazy_static;
use qtictac_ai::*;
use regex::Regex;

fn main() {
    let mut b = BoardState::new();
    let mut input = String::new();
    let stdin = std::io::stdin();

    while !b.classic().has_winner() {
        println!("{}", render_board(&b).unwrap());
        let mover = ['X', 'O'][b.next_mov() as usize % 2];
        if b.has_cycle() {
            println!("{} must resolve the cycle!", mover);
            loop {
                input.clear();
                stdin.read_line(&mut input).unwrap();
                match two_num_from_input(&input) {
                    Some((sq, mov)) => {
                        let mov = Move::Collapse {
                            sq: sq - 1,
                            mov: mov - 1,
                        };
                        if b.is_valid(mov) {
                            b.do_move(mov);
                            break;
                        }
                        println!("Invalid move!");
                    }
                    _ => {}
                }
            }
        } else {
            println!("{}'s move!", mover);
            loop {
                input.clear();
                stdin.read_line(&mut input).unwrap();
                match two_num_from_input(&input) {
                    Some((sq1, sq2)) => {
                        let mov = Move::Quantum(sq1 - 1, sq2 - 1);
                        if b.is_valid(mov) {
                            b.do_move(mov);
                            break;
                        }
                        println!("Invalid move!");
                    }
                    _ => {}
                }
            }
        }
    }
    match (b.classic().x_wins(), b.classic().o_wins()) {
        (true, true) => println!("Tie game!"),
        (true, false) => println!("X wins!"),
        (false, true) => println!("O wins!"),
        (false, false) => unreachable!(),
    }
}

fn two_num_from_input(input: &str) -> Option<(u8, u8)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([1-9])[ ,-:_|]*([1-9])").unwrap();
    }
    match RE.captures(&input) {
        Some(cap) => Some((cap[1].parse::<u8>().unwrap(), cap[2].parse::<u8>().unwrap())),
        None => None,
    }
}

use colored::{Color, Colorize};
use std::fmt::{self, Write};
fn render_board(b: &BoardState) -> Result<String, fmt::Error> {
    fn slice(b: &BoardState, buf: &mut String, sq: u8, row: usize) -> fmt::Result {
        // slice tells us whether its the top, bottom, or middle line of 3 char tall section
        if b.classic().is_o(sq) {
            write!(buf, " {} ", O_ASCII[row])?;
        } else if b.classic().is_x(sq) {
            write!(buf, " {} ", X_ASCII[row])?;
        } else {
            // render quantum
            for col in 0..3 {
                let mov = 3 * row + col;
                if !b.quantum().is(mov as u8, sq) {
                    write!(buf, "   ")?;
                } else {
                    use self::Color::*;
                    write!(
                        buf,
                        " {}",
                        format!("{}{}", ["X", "O"][mov % 2], mov + 1).color(
                            [
                                Red,
                                Green,
                                Yellow,
                                Blue,
                                Magenta,
                                Cyan,
                                White,
                                Black,
                                BrightGreen
                            ][mov]
                        )
                    )?;
                }
            }
            write!(buf, " ")?;
        }
        Ok(())
    }
    let mut buf = String::new();

    writeln!(buf, "          |          |           ")?;
    for big_row in 0..3 {
        for small_row in 0..3 {
            slice(b, &mut buf, 3 * big_row + 0, small_row)?;
            write!(buf, "|")?;
            slice(b, &mut buf, 3 * big_row + 1, small_row)?;
            write!(buf, "|")?;
            slice(b, &mut buf, 3 * big_row + 2, small_row)?;
            writeln!(buf, "")?;
        }
        if big_row < 2 {
            writeln!(buf, "__________|__________|___________")?;
            writeln!(buf, "          |          |           ")?;
        }
    }

    writeln!(buf, "          |          |           ")?;
    Ok(buf)
}

static X_ASCII: [&str; 3] = [" XX  XX ", "  XXXX  ", " XX  XX "];

static O_ASCII: [&str; 3] = [" OOOOOO ", " OO  OO ", " OOOOOO "];
