#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - data input register"]
    pub dinr: crate::Reg<dinr::DINR_SPEC>,
    #[doc = "0x0c - data output register"]
    pub doutr: crate::Reg<doutr::DOUTR_SPEC>,
    #[doc = "0x10 - key register 0"]
    pub keyr0: crate::Reg<keyr0::KEYR0_SPEC>,
    #[doc = "0x14 - key register 1"]
    pub keyr1: crate::Reg<keyr1::KEYR1_SPEC>,
    #[doc = "0x18 - key register 2"]
    pub keyr2: crate::Reg<keyr2::KEYR2_SPEC>,
    #[doc = "0x1c - key register 3"]
    pub keyr3: crate::Reg<keyr3::KEYR3_SPEC>,
    #[doc = "0x20 - initialization vector register 0"]
    pub ivr0: crate::Reg<ivr0::IVR0_SPEC>,
    #[doc = "0x24 - initialization vector register 1"]
    pub ivr1: crate::Reg<ivr1::IVR1_SPEC>,
    #[doc = "0x28 - initialization vector register 2"]
    pub ivr2: crate::Reg<ivr2::IVR2_SPEC>,
    #[doc = "0x2c - initialization vector register 3"]
    pub ivr3: crate::Reg<ivr3::IVR3_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DINR register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "data input register"]
pub mod dinr;
#[doc = "DOUTR register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "data output register"]
pub mod doutr;
#[doc = "KEYR0 register accessor: an alias for `Reg<KEYR0_SPEC>`"]
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "KEYR1 register accessor: an alias for `Reg<KEYR1_SPEC>`"]
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "KEYR2 register accessor: an alias for `Reg<KEYR2_SPEC>`"]
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "KEYR3 register accessor: an alias for `Reg<KEYR3_SPEC>`"]
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "IVR0 register accessor: an alias for `Reg<IVR0_SPEC>`"]
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "IVR1 register accessor: an alias for `Reg<IVR1_SPEC>`"]
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "IVR2 register accessor: an alias for `Reg<IVR2_SPEC>`"]
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "IVR3 register accessor: an alias for `Reg<IVR3_SPEC>`"]
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
#[doc = "initialization vector register 3"]
pub mod ivr3;
