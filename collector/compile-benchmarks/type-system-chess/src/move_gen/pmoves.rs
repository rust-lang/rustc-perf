use crate::{
    board_rep::{
        board::{
            idx::{IdxBoard, RunIdxBoard},
            BoardTy, CellEn, Empty, Filled,
        },
        color::{Black, ColorEn, White},
        piece::{Bishop, ColoredPiece, King, Knight, Pawn, PieceEn, Queen, Rook},
        square::{offset::MaybeSquare, AllSqs, SquareTy},
    },
    util::Bool,
};

use super::{
    bishop::{BishopMoveSqs, RunBishopMoveSqs},
    castle::{KingsideCastle, QueensideCastle, RunKingsideCastle, RunQueensideCastle},
    king::{KingMoveSqs, RunKingMoveSqs},
    knight::{KnightMoveSqs, RunKnightMoveSqs},
    list::{AppendMaybeMove, MLNil, MoveListTy, RunAppendMaybeMove, SLCons, SLNil, SquareListTy},
    ml_from_sl::{PMoveLFromSqs, RunPMoveLFromSqs},
    pawn::{PawnMoves, RunPawnMoves},
    queen::{QueenMoveSqs, RunQueenMoveSqs},
    rook::{RookMoveSqs, RunRookMoveSqs},
};

pub(crate) trait RunPMovesForSq<B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, ML: MoveListTy>:
    SquareTy
{
    type Output: MoveListTy;
}
pub(crate) type PMovesForSq<S, B, MoverC, EP, ML> =
    <S as RunPMovesForSq<B, MoverC, EP, ML>>::Output;

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForSq<B, MoverC, EP, ML> for S
where
    B: RunIdxBoard<S>,
    S: RunPMovesForTypeAtSq<B, MoverC, IdxBoard<B, S>, EP, ML>,
{
    type Output = <S as RunPMovesForTypeAtSq<B, MoverC, IdxBoard<B, S>, EP, ML>>::Output;
}

pub(crate) trait RunPMovesForTypeAtSq<
    B: BoardTy,
    MoverC: ColorEn,
    Type: CellEn,
    EP: MaybeSquare,
    ML: MoveListTy,
>: SquareTy
{
    type Output: MoveListTy;
}

impl<S: SquareTy, B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, MoverC, Empty, EP, ML> for S
{
    type Output = ML;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, White, Filled<ColoredPiece<P, Black>>, EP, ML> for S
{
    type Output = ML;
}
impl<S: SquareTy, B: BoardTy, P: PieceEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, Black, Filled<ColoredPiece<P, White>>, EP, ML> for S
{
    type Output = ML;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Knight, C>>, EP, ML> for S
where
    S: RunKnightMoveSqs,
    KnightMoveSqs<S>: RunPMoveLFromSqs<B, S, C, Knight, ML>,
{
    type Output = PMoveLFromSqs<KnightMoveSqs<S>, B, S, C, Knight, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<King, C>>, EP, ML> for S
where
    S: RunKingMoveSqs,
    KingMoveSqs<S>: RunPMoveLFromSqs<B, S, C, King, ML>,
{
    type Output = PMoveLFromSqs<KingMoveSqs<S>, B, S, C, King, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Rook, C>>, EP, ML> for S
where
    S: RunRookMoveSqs<B>,
    RookMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Rook, ML>,
{
    type Output = PMoveLFromSqs<RookMoveSqs<S, B>, B, S, C, Rook, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Bishop, C>>, EP, ML> for S
where
    S: RunBishopMoveSqs<B>,
    BishopMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Bishop, ML>,
{
    type Output = PMoveLFromSqs<BishopMoveSqs<S, B>, B, S, C, Bishop, ML>;
}
impl<S: SquareTy, B: BoardTy, C: ColorEn, EP: MaybeSquare, ML: MoveListTy>
    RunPMovesForTypeAtSq<B, C, Filled<ColoredPiece<Queen, C>>, EP, ML> for S
where
    S: RunQueenMoveSqs<B>,
    QueenMoveSqs<S, B>: RunPMoveLFromSqs<B, S, C, Queen, ML>,
{
    type Output = PMoveLFromSqs<QueenMoveSqs<S, B>, B, S, C, Queen, ML>;
}
impl<S: SquareTy, B: BoardTy, ML: MoveListTy, EP: MaybeSquare>
    RunPMovesForTypeAtSq<B, White, Filled<ColoredPiece<Pawn, White>>, EP, ML> for S
where
    S: RunPawnMoves<B, White, EP, ML>,
{
    type Output = PawnMoves<S, B, White, EP, ML>;
}
impl<S: SquareTy, B: BoardTy, ML: MoveListTy, EP: MaybeSquare>
    RunPMovesForTypeAtSq<B, Black, Filled<ColoredPiece<Pawn, Black>>, EP, ML> for S
where
    S: RunPawnMoves<B, Black, EP, ML>,
{
    type Output = PawnMoves<S, B, Black, EP, ML>;
}

pub(crate) trait RunPMoves<MoverC: ColorEn, EP: MaybeSquare, WK: Bool, WQ: Bool, BK: Bool, BQ: Bool>:
    BoardTy
{
    type Output: MoveListTy;
}
pub(crate) type PMoves<B, MoverC, EP, WK, WQ, BK, BQ> =
    <B as RunPMoves<MoverC, EP, WK, WQ, BK, BQ>>::Output;

impl<B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, WK: Bool, WQ: Bool, BK: Bool, BQ: Bool>
    RunPMoves<MoverC, EP, WK, WQ, BK, BQ> for B
where
    B: RunPMovesForSqL<AllSqs, MoverC, EP>,
    B: RunKingsideCastle<MoverC, WK, BK>,
    B: RunQueensideCastle<MoverC, WK, BK>,
    <B as RunPMovesForSqL<AllSqs, MoverC, EP>>::Output:
        RunAppendMaybeMove<KingsideCastle<B, MoverC, WK, BK>>,
    AppendMaybeMove<
        <B as RunPMovesForSqL<AllSqs, MoverC, EP>>::Output,
        KingsideCastle<B, MoverC, WK, BK>,
    >: RunAppendMaybeMove<QueensideCastle<B, MoverC, WK, BK>>,
{
    type Output = AppendMaybeMove<
        AppendMaybeMove<
            <B as RunPMovesForSqL<AllSqs, MoverC, EP>>::Output,
            KingsideCastle<B, MoverC, WK, BK>,
        >,
        QueensideCastle<B, MoverC, WK, BK>,
    >;
}

pub(crate) trait RunPMovesForSqL<L: SquareListTy, MoverC: ColorEn, EP: MaybeSquare>:
    BoardTy
{
    type Output: MoveListTy;
}
pub(crate) type PMovesForSqL<B, L, MoverC, EP> = <B as RunPMovesForSqL<L, MoverC, EP>>::Output;

impl<B: BoardTy, MoverC: ColorEn, EP: MaybeSquare> RunPMovesForSqL<SLNil, MoverC, EP> for B {
    type Output = MLNil;
}
impl<B: BoardTy, MoverC: ColorEn, EP: MaybeSquare, S: SquareTy, Next: SquareListTy>
    RunPMovesForSqL<SLCons<S, Next>, MoverC, EP> for B
where
    S: RunPMovesForSq<B, MoverC, EP, PMovesForSqL<B, Next, MoverC, EP>>,
    B: RunPMovesForSqL<Next, MoverC, EP>,
{
    type Output = PMovesForSq<S, B, MoverC, EP, PMovesForSqL<B, Next, MoverC, EP>>;
}
