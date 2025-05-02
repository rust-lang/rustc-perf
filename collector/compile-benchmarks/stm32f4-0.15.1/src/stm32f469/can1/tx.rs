#[doc = "TIR register accessor: an alias for `Reg<TIR_SPEC>`"]
pub type TIR = crate::Reg<tir::TIR_SPEC>;
#[doc = "TX mailbox identifier register"]
pub mod tir;
#[doc = "TDTR register accessor: an alias for `Reg<TDTR_SPEC>`"]
pub type TDTR = crate::Reg<tdtr::TDTR_SPEC>;
#[doc = "mailbox data length control and time stamp register"]
pub mod tdtr;
#[doc = "TDLR register accessor: an alias for `Reg<TDLR_SPEC>`"]
pub type TDLR = crate::Reg<tdlr::TDLR_SPEC>;
#[doc = "mailbox data low register"]
pub mod tdlr;
#[doc = "TDHR register accessor: an alias for `Reg<TDHR_SPEC>`"]
pub type TDHR = crate::Reg<tdhr::TDHR_SPEC>;
#[doc = "mailbox data high register"]
pub mod tdhr;
