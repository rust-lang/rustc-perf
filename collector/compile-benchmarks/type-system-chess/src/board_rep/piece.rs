use crate::{values, util::{Bool, False, True}};

use super::color::ColorEn;
use std::marker::PhantomData;

pub(crate) trait PieceEn {
    fn reify() -> values::Piece;
}
pub(crate) struct Pawn;
pub(crate) struct Bishop;
pub(crate) struct Knight;
pub(crate) struct Rook;
pub(crate) struct Queen;
pub(crate) struct King;

impl PieceEn for Pawn {
    fn reify() -> values::Piece {
        values::Piece::Pawn
    }
}
impl PieceEn for Bishop {
    fn reify() -> values::Piece {
        values::Piece::Bishop
    }
}
impl PieceEn for Knight {
    fn reify() -> values::Piece {
        values::Piece::Knight
    }
}
impl PieceEn for Rook {
    fn reify() -> values::Piece {
        values::Piece::Rook
    }
}
impl PieceEn for Queen {
    fn reify() -> values::Piece {
        values::Piece::Queen
    }
}
impl PieceEn for King {
    fn reify() -> values::Piece {
        values::Piece::King
    }
}

pub(crate) trait ColoredPieceTy {
    fn reify() -> values::ColoredPiece;
}
pub(crate) struct ColoredPiece<P: PieceEn, C: ColorEn>(PhantomData<(P, C)>);

impl<P: PieceEn, C: ColorEn> ColoredPieceTy for ColoredPiece<P, C> {
    fn reify() -> values::ColoredPiece {
        values::ColoredPiece {
            piece: P::reify(),
            color: C::reify(),
        }
    }
}

pub(crate) trait RunPieceEq<P: PieceEn>: PieceEn {
    type Output: Bool;
}
pub(crate) type PieceEq<A, B> = <A as RunPieceEq<B>>::Output;

impl RunPieceEq<Pawn  > for Pawn   { type Output = True; }
impl RunPieceEq<Pawn  > for Bishop { type Output = False; }
impl RunPieceEq<Pawn  > for Knight { type Output = False; }
impl RunPieceEq<Pawn  > for Rook   { type Output = False; }
impl RunPieceEq<Pawn  > for Queen  { type Output = False; }
impl RunPieceEq<Pawn  > for King   { type Output = False; }
impl RunPieceEq<Bishop> for Pawn   { type Output = False; }
impl RunPieceEq<Bishop> for Bishop { type Output = True; }
impl RunPieceEq<Bishop> for Knight { type Output = False; }
impl RunPieceEq<Bishop> for Rook   { type Output = False; }
impl RunPieceEq<Bishop> for Queen  { type Output = False; }
impl RunPieceEq<Bishop> for King   { type Output = False; }
impl RunPieceEq<Knight> for Pawn   { type Output = False; }
impl RunPieceEq<Knight> for Bishop { type Output = False; }
impl RunPieceEq<Knight> for Knight { type Output = True; }
impl RunPieceEq<Knight> for Rook   { type Output = False; }
impl RunPieceEq<Knight> for Queen  { type Output = False; }
impl RunPieceEq<Knight> for King   { type Output = False; }
impl RunPieceEq<Rook  > for Pawn   { type Output = False; }
impl RunPieceEq<Rook  > for Bishop { type Output = False; }
impl RunPieceEq<Rook  > for Knight { type Output = False; }
impl RunPieceEq<Rook  > for Rook   { type Output = True; }
impl RunPieceEq<Rook  > for Queen  { type Output = False; }
impl RunPieceEq<Rook  > for King   { type Output = False; }
impl RunPieceEq<Queen > for Pawn   { type Output = False; }
impl RunPieceEq<Queen > for Bishop { type Output = False; }
impl RunPieceEq<Queen > for Knight { type Output = False; }
impl RunPieceEq<Queen > for Rook   { type Output = False; }
impl RunPieceEq<Queen > for Queen  { type Output = True; }
impl RunPieceEq<Queen > for King   { type Output = False; }
impl RunPieceEq<King  > for Pawn   { type Output = False; }
impl RunPieceEq<King  > for Bishop { type Output = False; }
impl RunPieceEq<King  > for Knight { type Output = False; }
impl RunPieceEq<King  > for Rook   { type Output = False; }
impl RunPieceEq<King  > for Queen  { type Output = False; }
impl RunPieceEq<King  > for King   { type Output = True; }

