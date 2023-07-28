use super::{
    list::{AppendMaybeMove, MoveListTy, RunAppendMaybeMove, SLCons, SLNil, SquareListTy},
    MaybeMove, Move, NoMove, SomeMove,
};
use crate::board_rep::{
    board::{
        idx::{IdxBoard, IdxBoardRank, RunIdxBoardRank, RunIdxRank},
        BoardTy, CellEn, Empty, Filled,
    },
    color::{Black, ColorEn, White},
    piece::{ColoredPiece, PieceEn},
    square::{file::FileEn, rank::RankEn, Square, SquareTy},
};

pub(crate) trait RunPMoveLFromSqs<
    B: BoardTy,
    Start: SquareTy,
    MoverC: ColorEn,
    MoverP: PieceEn,
    ML: MoveListTy,
>: SquareListTy
{
    type Output: MoveListTy;
}
pub(crate) type PMoveLFromSqs<L, B, Start, MoverC, MoverP, ML> =
    <L as RunPMoveLFromSqs<B, Start, MoverC, MoverP, ML>>::Output;

impl<B: BoardTy, Start: SquareTy, MoverC: ColorEn, MoverP: PieceEn, ML: MoveListTy>
    RunPMoveLFromSqs<B, Start, MoverC, MoverP, ML> for SLNil
{
    type Output = ML;
}
impl<
        Rest: SquareListTy,
        End: SquareTy,
        B: BoardTy,
        Start: SquareTy,
        MoverC: ColorEn,
        MoverP: PieceEn,
        ML: MoveListTy,
    > RunPMoveLFromSqs<B, Start, MoverC, MoverP, ML> for SLCons<End, Rest>
where
    End: RunPMoveFromSq<B, Start, MoverC, MoverP>,
    Rest: RunPMoveLFromSqs<B, Start, MoverC, MoverP, ML>,
    PMoveLFromSqs<Rest, B, Start, MoverC, MoverP, ML>:
        RunAppendMaybeMove<<End as RunPMoveFromSq<B, Start, MoverC, MoverP>>::Output>,
{
    type Output = AppendMaybeMove<
        PMoveLFromSqs<Rest, B, Start, MoverC, MoverP, ML>,
        <End as RunPMoveFromSq<B, Start, MoverC, MoverP>>::Output,
    >;
}

pub(crate) trait RunPMoveFromSq<B: BoardTy, Start: SquareTy, MoverC: ColorEn, MoverP: PieceEn>:
    SquareTy
{
    type Output: MaybeMove;
}

impl<R: RankEn, F: FileEn, B: BoardTy, Start: SquareTy, MoverC: ColorEn, MoverP: PieceEn>
    RunPMoveFromSq<B, Start, MoverC, MoverP> for Square<R, F>
where
    B: RunIdxBoardRank<R>,
    IdxBoardRank<B, R>: RunIdxRank<F>,
    Square<R, F>: RunPMoveFromSqContents<Start, MoverC, MoverP, IdxBoard<B, Square<R, F>>>,
{
    type Output = <Square<R, F> as RunPMoveFromSqContents<
        Start,
        MoverC,
        MoverP,
        IdxBoard<B, Square<R, F>>,
    >>::Output;
}

pub(crate) trait RunPMoveFromSqContents<
    Start: SquareTy,
    MoverC: ColorEn,
    MoverP: PieceEn,
    Contents: CellEn,
>: SquareTy
{
    type Output: MaybeMove;
}

impl<End: SquareTy, Start: SquareTy, MoverC: ColorEn, MoverP: PieceEn>
    RunPMoveFromSqContents<Start, MoverC, MoverP, Empty> for End
{
    type Output = SomeMove<Move<Start, End, ColoredPiece<MoverP, MoverC>>>;
}
impl<End: SquareTy, Start: SquareTy, MoverP: PieceEn, P: PieceEn>
    RunPMoveFromSqContents<Start, White, MoverP, Filled<ColoredPiece<P, White>>> for End
{
    type Output = NoMove;
}
impl<End: SquareTy, Start: SquareTy, MoverP: PieceEn, P: PieceEn>
    RunPMoveFromSqContents<Start, Black, MoverP, Filled<ColoredPiece<P, Black>>> for End
{
    type Output = NoMove;
}
impl<End: SquareTy, Start: SquareTy, MoverP: PieceEn, P: PieceEn>
    RunPMoveFromSqContents<Start, White, MoverP, Filled<ColoredPiece<P, Black>>> for End
{
    type Output = SomeMove<Move<Start, End, ColoredPiece<MoverP, White>>>;
}
impl<End: SquareTy, Start: SquareTy, MoverP: PieceEn, P: PieceEn>
    RunPMoveFromSqContents<Start, Black, MoverP, Filled<ColoredPiece<P, White>>> for End
{
    type Output = SomeMove<Move<Start, End, ColoredPiece<MoverP, Black>>>;
}
