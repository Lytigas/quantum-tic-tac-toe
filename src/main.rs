use ansi_escapes::EraseScreen;
use lazy_static::lazy_static;
use qtictac_ai::*;
use regex::Regex;

fn main() {
    let mut b = BoardState::new();
    let mut input = String::new();
    let stdin = std::io::stdin();
    println!(include_str!("../instructions.txt"));
    stdin.read_line(&mut input).unwrap(); // wait for user acknowledgement

    while !b.classic().game_is_over() {
        // array literal thats indexed by the move
        let mover = ['X', 'O'][b.next_mov() as usize % 2];
        let mut has_tried = false;
        loop {
            println!("{}", EraseScreen);
            println!("{}", render_board(&b).unwrap()); // this call uses a format syntax, where each "{}" is replaced with the corresponding argument
            if has_tried {
                print!("Invalid move! ");
            }
            println!(
                "{}{}",
                mover,
                [
                    "'s move. (\"square1, square2\")",
                    " must resolve the cycle! (\"square, move to collapse to\")"
                ][if b.has_cycle() { 1 } else { 0 }]
            );
            input.clear();
            stdin.read_line(&mut input).unwrap();
            match two_num_from_input(&input) {
                Some((first, second)) => {
                    let mov = if b.has_cycle() {
                        Move::Collapse {
                            sq: first - 1,
                            mov: second - 1,
                        }
                    } else {
                        Move::Quantum(first - 1, second - 1)
                    };
                    if b.is_valid(mov) {
                        b.do_move(mov);
                        break;
                    }
                }
                _ => {}
            }
            has_tried = true;
        }
    }
    println!("{}", EraseScreen);
    println!("{}", render_board(&b).unwrap());
    match (b.classic().x_wins(), b.classic().o_wins()) {
        (true, true) | (false, false) => println!("Tie game!"),
        (true, false) => println!("X wins!"),
        (false, true) => println!("O wins!"),
    }
}

fn two_num_from_input(input: &str) -> Option<(u8, u8)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([1-9])[ ,-:_|]*([1-9])").unwrap();
    }
    match RE.captures(&input) {
        // unwrap is ok becuase anything that matches the regex should parse correctly
        Some(cap) => Some((cap[1].parse::<u8>().unwrap(), cap[2].parse::<u8>().unwrap())),
        None => None,
    }
}

use colored::{Color, Colorize};
use std::fmt::{self, Write};
// the Result type and `?` throughout this method are just for handling IO errors and can be ignored
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
                    // bring the colors into scope for formatting with them
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
                                BrightRed,
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

    writeln!(buf, "1         |2         |3          ")?;
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
            writeln!(
                buf,
                "{}         |{}         |{}          ",
                big_row * 3 + 4,
                big_row * 3 + 5,
                big_row * 3 + 6
            )?;
        }
    }

    writeln!(buf, "          |          |           ")?;
    // write the cycle if the board has it
    if b.has_cycle() {
        write!(buf, "Cycle:")?;
        b.cycle().for_each(|sq| write!(buf, " {}", sq + 1).unwrap());
    }
    Ok(buf)
}

static X_ASCII: [&str; 3] = [" XX  XX ", "  XXXX  ", " XX  XX "];

static O_ASCII: [&str; 3] = [" OOOOOO ", " OO  OO ", " OOOOOO "];
