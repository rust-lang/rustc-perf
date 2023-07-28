#![allow(clippy::type_complexity)]
#![recursion_limit = "256"]

mod board_rep;
mod move_gen;
mod state;
mod ui;
mod util;
mod values;

fn main() {
    use ui::prelude::*;
    use move_gen::{moves::Moves, list::MoveListTy};
    use state::EmptyCa;

    type B = Board<
        //        AA  BB  CC  DD  EE  FF  GG  HH
        BoardRank<__, __, __, __, __, __, __, __>, // 1
        BoardRank<__, __, __, __, WP, __, WP, __>, // 2
        BoardRank<__, __, __, __, __, __, __, __>, // 3
        BoardRank<__, WR, __, __, __, BP, __, BK>, // 4
        BoardRank<WK, WP, __, __, __, __, __, BR>, // 5
        BoardRank<__, __, __, BP, __, __, __, __>, // 6
        BoardRank<__, __, BP, __, __, __, __, __>, // 7
        BoardRank<__, __, __, __, __, __, __, __>, // 8
    >;
    type S = State<White, B, NoSquare, EmptyCa>;
    type Ms = Moves<S>;

    assert_eq!(Ms::reify().0.len(), 14);
}
