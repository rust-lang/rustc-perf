#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - PLL configuration register"]
    pub pllcfgr: crate::Reg<pllcfgr::PLLCFGR_SPEC>,
    #[doc = "0x08 - clock configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x0c - clock interrupt register"]
    pub cir: crate::Reg<cir::CIR_SPEC>,
    #[doc = "0x10 - AHB1 peripheral reset register"]
    pub ahb1rstr: crate::Reg<ahb1rstr::AHB1RSTR_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - APB1 peripheral reset register"]
    pub apb1rstr: crate::Reg<apb1rstr::APB1RSTR_SPEC>,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x30 - AHB1 peripheral clock register"]
    pub ahb1enr: crate::Reg<ahb1enr::AHB1ENR_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    pub apb1enr: crate::Reg<apb1enr::APB1ENR_SPEC>,
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    pub ahb1lpenr: crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: crate::Reg<apb1lpenr::APB1LPENR_SPEC>,
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    pub apb2lpenr: crate::Reg<apb2lpenr::APB2LPENR_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x70 - Backup domain control register"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x74 - clock control & status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x80 - spread spectrum clock generation register"]
    pub sscgr: crate::Reg<sscgr::SSCGR_SPEC>,
    _reserved16: [u8; 0x08],
    #[doc = "0x8c - DCKCFGR register"]
    pub dckcfgr: crate::Reg<dckcfgr::DCKCFGR_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - DCKCFGR2 register"]
    pub dckcfgr2: crate::Reg<dckcfgr2::DCKCFGR2_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLLCFGR register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "CIR register accessor: an alias for `Reg<CIR_SPEC>`"]
pub type CIR = crate::Reg<cir::CIR_SPEC>;
#[doc = "clock interrupt register"]
pub mod cir;
#[doc = "AHB1RSTR register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "APB1RSTR register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2RSTR register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "AHB1 peripheral clock register"]
pub mod ahb1enr;
#[doc = "APB1ENR register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2ENR register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1LPENR register accessor: an alias for `Reg<AHB1LPENR_SPEC>`"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "APB1LPENR register accessor: an alias for `Reg<APB1LPENR_SPEC>`"]
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENR_SPEC>;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2LPENR register accessor: an alias for `Reg<APB2LPENR_SPEC>`"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "BDCR register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "clock control & status register"]
pub mod csr;
#[doc = "SSCGR register accessor: an alias for `Reg<SSCGR_SPEC>`"]
pub type SSCGR = crate::Reg<sscgr::SSCGR_SPEC>;
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "DCKCFGR register accessor: an alias for `Reg<DCKCFGR_SPEC>`"]
pub type DCKCFGR = crate::Reg<dckcfgr::DCKCFGR_SPEC>;
#[doc = "DCKCFGR register"]
pub mod dckcfgr;
#[doc = "DCKCFGR2 register accessor: an alias for `Reg<DCKCFGR2_SPEC>`"]
pub type DCKCFGR2 = crate::Reg<dckcfgr2::DCKCFGR2_SPEC>;
#[doc = "DCKCFGR2 register"]
pub mod dckcfgr2;
