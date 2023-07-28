#![allow(unused)]

use std::fmt::{write, Debug, Display};

use crate::board_rep::{board::CellEn, square::offset::rank};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Color {
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => "White",
                Color::Black => "Black",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ColoredPiece {
    pub(crate) piece: Piece,
    pub(crate) color: Color,
}
impl Display for ColoredPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match (self.piece, self.color) {
            (Piece::Pawn, Color::White) => 'P',
            (Piece::Pawn, Color::Black) => 'p',
            (Piece::Bishop, Color::White) => 'B',
            (Piece::Bishop, Color::Black) => 'b',
            (Piece::Knight, Color::White) => 'N',
            (Piece::Knight, Color::Black) => 'n',
            (Piece::Rook, Color::White) => 'R',
            (Piece::Rook, Color::Black) => 'r',
            (Piece::Queen, Color::White) => 'Q',
            (Piece::Queen, Color::Black) => 'q',
            (Piece::King, Color::White) => 'K',
            (Piece::King, Color::Black) => 'k',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}
impl Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R1 => write!(f, "1"),
            Self::R2 => write!(f, "2"),
            Self::R3 => write!(f, "3"),
            Self::R4 => write!(f, "4"),
            Self::R5 => write!(f, "5"),
            Self::R6 => write!(f, "6"),
            Self::R7 => write!(f, "7"),
            Self::R8 => write!(f, "8"),
        }
    }
}
impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Square {
    pub(crate) rank: Rank,
    pub(crate) file: File,
}
impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.file, self.rank)
    }
}
impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.file, self.rank)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SquareList(pub Vec<Square>);
impl Display for SquareList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use File::*;
        use Rank::*;

        writeln!(f, "  A B C D E F G H ")?;
        for rank in [R1, R2, R3, R4, R5, R6, R7, R8] {
            write!(f, "{rank} ")?;
            for file in [A, B, C, D, E, F, G, H] {
                let sq = Square { rank, file };
                let inc = self.0.contains(&sq);
                let c = if inc { '#' } else { '.' };
                write!(f, "{c} ")?;
            }
            writeln!(f, "{rank}")?;
        }
        writeln!(f, "  A B C D E F G H ")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Cell {
    Empty,
    Filled(ColoredPiece),
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Filled(cp) => write!(f, "{cp}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoardRank {
    pub(crate) a: Cell,
    pub(crate) b: Cell,
    pub(crate) c: Cell,
    pub(crate) d: Cell,
    pub(crate) e: Cell,
    pub(crate) f: Cell,
    pub(crate) g: Cell,
    pub(crate) h: Cell,
}
impl BoardRank {
    fn file(&self, r: File) -> Cell {
        match r {
            File::A => self.a,
            File::B => self.b,
            File::C => self.c,
            File::D => self.d,
            File::E => self.e,
            File::F => self.f,
            File::G => self.g,
            File::H => self.h,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Board {
    pub(crate) r1: BoardRank,
    pub(crate) r2: BoardRank,
    pub(crate) r3: BoardRank,
    pub(crate) r4: BoardRank,
    pub(crate) r5: BoardRank,
    pub(crate) r6: BoardRank,
    pub(crate) r7: BoardRank,
    pub(crate) r8: BoardRank,
}
impl Board {
    fn rank(&self, r: Rank) -> &BoardRank {
        match r {
            Rank::R1 => &self.r1,
            Rank::R2 => &self.r2,
            Rank::R3 => &self.r3,
            Rank::R4 => &self.r4,
            Rank::R5 => &self.r5,
            Rank::R6 => &self.r6,
            Rank::R7 => &self.r7,
            Rank::R8 => &self.r8,
        }
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use File::*;
        use Rank::*;

        writeln!(f, "  A B C D E F G H ")?;
        for rank in [R1, R2, R3, R4, R5, R6, R7, R8] {
            write!(f, "{rank} ")?;
            for file in [A, B, C, D, E, F, G, H] {
                let sq = Square { rank, file };
                let cell = self.rank(rank).file(file);
                write!(f, "{cell} ")?;
            }
            writeln!(f, "{rank}")?;
        }
        write!(f, "  A B C D E F G H ")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SquareSetRank {
    pub(crate) a: bool,
    pub(crate) b: bool,
    pub(crate) c: bool,
    pub(crate) d: bool,
    pub(crate) e: bool,
    pub(crate) f: bool,
    pub(crate) g: bool,
    pub(crate) h: bool,
}
impl SquareSetRank {
    fn file(&self, r: File) -> bool {
        match r {
            File::A => self.a,
            File::B => self.b,
            File::C => self.c,
            File::D => self.d,
            File::E => self.e,
            File::F => self.f,
            File::G => self.g,
            File::H => self.h,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SquareSet {
    pub(crate) r1: SquareSetRank,
    pub(crate) r2: SquareSetRank,
    pub(crate) r3: SquareSetRank,
    pub(crate) r4: SquareSetRank,
    pub(crate) r5: SquareSetRank,
    pub(crate) r6: SquareSetRank,
    pub(crate) r7: SquareSetRank,
    pub(crate) r8: SquareSetRank,
}
impl SquareSet {
    fn rank(&self, r: Rank) -> &SquareSetRank {
        match r {
            Rank::R1 => &self.r1,
            Rank::R2 => &self.r2,
            Rank::R3 => &self.r3,
            Rank::R4 => &self.r4,
            Rank::R5 => &self.r5,
            Rank::R6 => &self.r6,
            Rank::R7 => &self.r7,
            Rank::R8 => &self.r8,
        }
    }
}
impl Display for SquareSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use File::*;
        use Rank::*;

        writeln!(f, "  A B C D E F G H ")?;
        for rank in [R1, R2, R3, R4, R5, R6, R7, R8] {
            write!(f, "{rank} ")?;
            for file in [A, B, C, D, E, F, G, H] {
                let sq = Square { rank, file };
                let cell = self.rank(rank).file(file);
                let char = if cell { '#' } else { '.' };
                write!(f, "{char} ")?;
            }
            writeln!(f, "{rank}")?;
        }
        write!(f, "  A B C D E F G H ")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CastleState {
    pub(crate) wk: bool,
    pub(crate) wq: bool,
    pub(crate) bk: bool,
    pub(crate) bq: bool,
}

impl Display for CastleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(4);
        if self.wk {
            res.push('K')
        };
        if self.wq {
            res.push('Q')
        };
        if self.bk {
            res.push('k')
        };
        if self.bq {
            res.push('q')
        };
        if res == "" {
            res.push('-')
        }
        write!(f, "{res}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct State {
    pub(crate) to_move: Color,
    pub(crate) pieces: Board,
    pub(crate) ep_square: Option<Square>,
    pub(crate) castle_state: CastleState,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.pieces)?;
        write!(
            f,
            "To move: {} | EP: {} | Castle: {}",
            self.to_move,
            if let Some(sq) = self.ep_square {
                format!("{sq}")
            } else {
                "-".to_string()
            },
            self.castle_state
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Move {
    pub(crate) from: Square,
    pub(crate) to: Square,
    pub(crate) piece: ColoredPiece,
    pub(crate) ep: Option<Square>,
    pub(crate) rook_from: Option<Square>,
    pub(crate) rook_to: Option<Square>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MoveList(pub Vec<Move>);
impl MoveList {
    pub fn destinations(&self) -> SquareList {
        SquareList(self.0.iter().map(|m| m.to).collect())
    }
}

#[derive(Debug)]
pub(crate) struct Offset {
    pub(crate) rank: i8,
    pub(crate) file: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    Ongoing,
    Draw,
    Checkmate(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EMResult {
    InvalidMove,
    Ongoing(State),
    Draw,
    Checkmate(Color),
}

impl EMResult {
    pub(crate) fn unwrap_state(&self) -> &State {
        match self {
            EMResult::InvalidMove => panic!("Expected state, got invalid move."),
            EMResult::Ongoing(s) => s,
            EMResult::Draw => panic!("Expected state, got draw."),
            EMResult::Checkmate(c) => panic!("Expected state, got checkmate by {c}."),
        }
    }
}

impl Display for EMResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EMResult::InvalidMove => write!(f, "Invalid move."),
            EMResult::Ongoing(s) => write!(f, "{s}"),
            EMResult::Draw => write!(f, "Draw by stalemate."),
            EMResult::Checkmate(c) => write!(f, "Checkmate - {c} wins!"),
        }
    }
}
