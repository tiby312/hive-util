pub enum PieceType {
    Queen,
    Grasshopper,
    Spider,
    Ant,
    Beetle,
    Mosquito,
    Ladybug,
    Pillbug,
}

// fn main(){
//     process_game(r##"wS1;bS1 wS1-;wQ -wS1;bQ bS1/;wG1 -wQ;bG1 \bQ;wG1 bQ\;bG2 bQ/;wA1 wQ\;bA1 bG2/;wA1 bG2\;bA1 \bG2;wQ \wS1;bA1 bG2/;wQ /bG1"##);
// }

pub struct Piece {
    team: u32,
    piece_type: PieceType,
    piece_ident: u32,
}

pub enum Offset {
    TopLeft,
    BottomLeft,
    Left,
    Right,
    TopRight,
    BottomRight,
}

pub struct Move {
    piece: Piece,
    target: (Piece, Offset),
}

enum Movement {
    Horizontal,
    UP,
    Down,
}
fn parse_movement(ch: char) -> Option<Movement> {
    Some(match ch {
        '\\' => Movement::UP,
        '-' => Movement::Horizontal,
        '/' => Movement::Down,
        _ => return None,
    })
}

pub trait Doopa {
    fn next_fmt(&mut self) -> Result<char, std::fmt::Error>;
}
impl Doopa for std::str::Chars<'_> {
    fn next_fmt(&mut self) -> Result<char, std::fmt::Error> {
        self.next().ok_or_else(|| std::fmt::Error)
    }
}

pub trait Digi {
    fn digi(self) -> Result<u32, std::fmt::Error>;
}
impl Digi for char {
    fn digi(self) -> Result<u32, std::fmt::Error> {
        self.to_digit(10).ok_or_else(|| std::fmt::Error)
    }
}

fn parse_command(a: &str) -> Result<Move, std::fmt::Error> {
    let mut moves = vec![];
    let mut k = a.chars();

    let team1 = k.next_fmt()?.digi()?;

    let src = match k.next_fmt()? {
        'Q' => (PieceType::Queen, 0),
        'S' => (PieceType::Spider, k.next_fmt()?.digi()?),
        'G' => (PieceType::Grasshopper, k.next_fmt()?.digi()?),

        'A' => (PieceType::Ant, k.next_fmt()?.digi()?),
        'B' => (PieceType::Beetle, k.next_fmt()?.digi()?),
        _ => {
            return Err(std::fmt::Error);
        }
    };

    if let Some('_') = k.next() {
    } else {
        return Err(std::fmt::Error);
    }

    let c = k.next().ok_or_else(|| std::fmt::Error)?;
    if let Some(m) = parse_movement(c) {}

    let team2 = k
        .next()
        .ok_or_else(|| std::fmt::Error)?
        .to_digit(10)
        .ok_or_else(|| std::fmt::Error)?;

    let dest = match k.next().ok_or_else(|| std::fmt::Error)? {
        'Q' => (PieceType::Queen, 0),
        'S' => {
            let d = k
                .next()
                .ok_or_else(|| std::fmt::Error)?
                .to_digit(10)
                .ok_or_else(|| std::fmt::Error)?;
            (PieceType::Spider, d)
        }
        'G' => {
            let d = k
                .next()
                .ok_or_else(|| std::fmt::Error)?
                .to_digit(10)
                .ok_or_else(|| std::fmt::Error)?;
            (PieceType::Grasshopper, d)
        }

        'A' => {
            let d = k
                .next()
                .ok_or_else(|| std::fmt::Error)?
                .to_digit(10)
                .ok_or_else(|| std::fmt::Error)?;
            (PieceType::Ant, d)
        }
        'B' => {
            let d = k
                .next()
                .ok_or_else(|| std::fmt::Error)?
                .to_digit(10)
                .ok_or_else(|| std::fmt::Error)?;
            (PieceType::Beetle, d)
        }
        _ => {
            return Err(std::fmt::Error);
        }
    };

    let dest = Piece {
        team: team2,
        piece_type: dest.0,
        piece_ident: dest.1,
    };
    Ok(Move {
        piece: Piece {
            team: team1,
            piece_type: src.0,
            piece_ident: src.1,
        },
        target: (dest, offset),
    })
    // let first = piece_string.chars().next()?;
    // let last = piece_string.chars().rev().next()?;
    // let dir = if "\\-/".contains(first) {
    //     piece_string = &piece_string[1..];
    //     Some(match first {
    //         '\\' => Direction::NW,
    //         '-' => Direction::W,
    //         '/' => Direction::SW,
    //         _ => return None,
    //     })
    // } else if "\\-/".contains(last) {
    //     piece_string = &piece_string[..piece_string.len() - 1];
    //     Some(match last {
    //         '/' => Direction::NE,
    //         '-' => Direction::E,
    //         '\\' => Direction::SE,
    //         _ => return None,
    //     })
    // } else {
    //     None
    // };

    // let mut chars = piece_string.chars();
    // let color = match chars.next()? {
    //     'w' => Color::White,
    //     'b' => Color::Black,
    //     _ => return None,
    // };
    // let bug = Bug::from_char(chars.next()?)?;
    // let bug_num = if Bug::initial_quantity()[bug as usize] > 1 {
    //     char::to_digit(chars.next()?, 10)? as u8
    // } else if chars.next().is_some() {
    //     return None;
    // } else {
    //     1
    // };
    // Some((color, bug, bug_num, dir))

    //let tokens = move_string.split(' ').collect::<Vec<_>>();
    //let (color, bug, bug_num, dir) = self.parse_piece_name(tokens[0]).ok_or_else(err)?;
}

fn process_game(a: &str) {
    for a in a.split(";") {
        parse_command(a);
    }

    // Base;WhiteWins;Black[8];wS1;bS1 wS1-;wQ -wS1;bQ bS1/;wG1 -wQ;bG1 \bQ;wG1 bQ\;bG2 bQ/;wA1 wQ\;bA1 bG2/;wA1 bG2\;bA1 \bG2;wQ \wS1;bA1 bG2/;wQ /bG1

    //TODO parse it, replace piece ids with positions.
    //instead of bug1 to next to bug2, instead:
    //piece at pos1 to next to piece at pos2.
}

pub struct Stackable {
    level: u8,
}
pub struct PieceInfo {
    pos: [u8; 2],
}

pub struct Piece<'a> {
    typ: PieceType,
    piece_info: &'a PieceInfo,
}

pub struct Team {
    queen: [PieceInfo; 1],
    grasshopper: [PieceInfo; 3],
    grasshopper_meta: [Stackable; 3],
    spider: [PieceInfo; 2],
    ant: [PieceInfo; 3],
}
impl Team {
    fn all_pieces(&self) -> [&[PieceInfo]; 2] {
        let q = &self.queen;
        let a = &self.ant;
        [q, a]
    }
}

// pub enum Rel{
//     Horizontal,
//     Above,
//     Below
// }
// pub enum Dir{
//     Left,
//     Right
// }

// pub enum Turn {
//     Place(u8, Bug),
//     Move(u8, u8),
//     Pass,
// }
