#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x04 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x0c - sample time register 1"]
    pub smpr1: crate::Reg<smpr1::SMPR1_SPEC>,
    #[doc = "0x10 - sample time register 2"]
    pub smpr2: crate::Reg<smpr2::SMPR2_SPEC>,
    #[doc = "0x14 - injected channel data offset register x"]
    pub jofr1: crate::Reg<jofr::JOFR_SPEC>,
    #[doc = "0x18 - injected channel data offset register x"]
    pub jofr2: crate::Reg<jofr::JOFR_SPEC>,
    #[doc = "0x1c - injected channel data offset register x"]
    pub jofr3: crate::Reg<jofr::JOFR_SPEC>,
    #[doc = "0x20 - injected channel data offset register x"]
    pub jofr4: crate::Reg<jofr::JOFR_SPEC>,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub htr: crate::Reg<htr::HTR_SPEC>,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub ltr: crate::Reg<ltr::LTR_SPEC>,
    #[doc = "0x2c - regular sequence register 1"]
    pub sqr1: crate::Reg<sqr1::SQR1_SPEC>,
    #[doc = "0x30 - regular sequence register 2"]
    pub sqr2: crate::Reg<sqr2::SQR2_SPEC>,
    #[doc = "0x34 - regular sequence register 3"]
    pub sqr3: crate::Reg<sqr3::SQR3_SPEC>,
    #[doc = "0x38 - injected sequence register"]
    pub jsqr: crate::Reg<jsqr::JSQR_SPEC>,
    #[doc = "0x3c - injected data register x"]
    pub jdr1: crate::Reg<jdr::JDR_SPEC>,
    #[doc = "0x40 - injected data register x"]
    pub jdr2: crate::Reg<jdr::JDR_SPEC>,
    #[doc = "0x44 - injected data register x"]
    pub jdr3: crate::Reg<jdr::JDR_SPEC>,
    #[doc = "0x48 - injected data register x"]
    pub jdr4: crate::Reg<jdr::JDR_SPEC>,
    #[doc = "0x4c - regular data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
}
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMPR1 register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "JOFR register accessor: an alias for `Reg<JOFR_SPEC>`"]
pub type JOFR = crate::Reg<jofr::JOFR_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod jofr;
#[doc = "HTR register accessor: an alias for `Reg<HTR_SPEC>`"]
pub type HTR = crate::Reg<htr::HTR_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "LTR register accessor: an alias for `Reg<LTR_SPEC>`"]
pub type LTR = crate::Reg<ltr::LTR_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "SQR1 register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "JSQR register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "JDR register accessor: an alias for `Reg<JDR_SPEC>`"]
pub type JDR = crate::Reg<jdr::JDR_SPEC>;
#[doc = "injected data register x"]
pub mod jdr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "regular data register"]
pub mod dr;
