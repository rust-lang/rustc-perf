#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    pub gcr: crate::Reg<gcr::GCR_SPEC>,
    #[doc = "0x04..0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub cha: CH,
    #[doc = "0x24..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub chb: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - AConfiguration register 1"]
    pub cr1: crate::Reg<self::ch::cr1::CR1_SPEC>,
    #[doc = "0x04 - AConfiguration register 2"]
    pub cr2: crate::Reg<self::ch::cr2::CR2_SPEC>,
    #[doc = "0x08 - AFRCR"]
    pub frcr: crate::Reg<self::ch::frcr::FRCR_SPEC>,
    #[doc = "0x0c - ASlot register"]
    pub slotr: crate::Reg<self::ch::slotr::SLOTR_SPEC>,
    #[doc = "0x10 - AInterrupt mask register2"]
    pub im: crate::Reg<self::ch::im::IM_SPEC>,
    #[doc = "0x14 - AStatus register"]
    pub sr: crate::Reg<self::ch::sr::SR_SPEC>,
    #[doc = "0x18 - AClear flag register"]
    pub clrfr: crate::Reg<self::ch::clrfr::CLRFR_SPEC>,
    #[doc = "0x1c - AData register"]
    pub dr: crate::Reg<self::ch::dr::DR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub mod ch;
#[doc = "GCR register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global configuration register"]
pub mod gcr;
