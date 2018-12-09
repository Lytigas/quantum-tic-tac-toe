use qtictac_ai::*;

fn main() {
    let mut b = BoardState::new();

    b.do_move(Move::Quantum(0, 1)); // 1
    b.do_move(Move::Quantum(7, 8)); // 2
    b.do_move(Move::Quantum(2, 4)); // 3
    b.do_move(Move::Quantum(4, 5)); // 4
    b.do_move(Move::Quantum(5, 6)); // 5
    b.do_move(Move::Quantum(1, 2)); // 6
    b.do_move(Move::Quantum(4, 8)); // 7
    b.do_move(Move::Quantum(3, 7)); // 8
    b.do_move(Move::Quantum(0, 6)); // 9
    println!("{}", render_board(&b).unwrap());
    b.do_move(Move::Collapse { sq: 4, mov: 2 });
    println!("{}", render_board(&b).unwrap());
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
