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
    #[doc = "0x14 - AHB2 peripheral reset register"]
    pub ahb2rstr: crate::Reg<ahb2rstr::AHB2RSTR_SPEC>,
    #[doc = "0x18 - AHB3 peripheral reset register"]
    pub ahb3rstr: crate::Reg<ahb3rstr::AHB3RSTR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - APB1 peripheral reset register"]
    pub apb1rstr: crate::Reg<apb1rstr::APB1RSTR_SPEC>,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: crate::Reg<apb2rstr::APB2RSTR_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - AHB1 peripheral clock register"]
    pub ahb1enr: crate::Reg<ahb1enr::AHB1ENR_SPEC>,
    #[doc = "0x34 - AHB2 peripheral clock enable register"]
    pub ahb2enr: crate::Reg<ahb2enr::AHB2ENR_SPEC>,
    #[doc = "0x38 - AHB3 peripheral clock enable register"]
    pub ahb3enr: crate::Reg<ahb3enr::AHB3ENR_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    pub apb1enr: crate::Reg<apb1enr::APB1ENR_SPEC>,
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    pub apb2enr: crate::Reg<apb2enr::APB2ENR_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    pub ahb1lpenr: crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>,
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    pub ahb2lpenr: crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>,
    #[doc = "0x58 - AHB3 peripheral clock enable in low power mode register"]
    pub ahb3lpenr: crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: crate::Reg<apb1lpenr::APB1LPENR_SPEC>,
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    pub apb2lpenr: crate::Reg<apb2lpenr::APB2LPENR_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x70 - Backup domain control register"]
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    #[doc = "0x74 - clock control & status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0x80 - spread spectrum clock generation register"]
    pub sscgr: crate::Reg<sscgr::SSCGR_SPEC>,
    #[doc = "0x84 - PLLI2S configuration register"]
    pub plli2scfgr: crate::Reg<plli2scfgr::PLLI2SCFGR_SPEC>,
    #[doc = "0x88 - RCC PLL configuration register"]
    pub pllsaicfgr: crate::Reg<pllsaicfgr::PLLSAICFGR_SPEC>,
    #[doc = "0x8c - RCC Dedicated Clock Configuration Register"]
    pub dckcfgr: crate::Reg<dckcfgr::DCKCFGR_SPEC>,
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
#[doc = "AHB2RSTR register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3RSTR register accessor: an alias for `Reg<AHB3RSTR_SPEC>`"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
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
#[doc = "AHB2ENR register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3ENR register accessor: an alias for `Reg<AHB3ENR_SPEC>`"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
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
#[doc = "AHB2LPENR register accessor: an alias for `Reg<AHB2LPENR_SPEC>`"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "AHB3LPENR register accessor: an alias for `Reg<AHB3LPENR_SPEC>`"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>;
#[doc = "AHB3 peripheral clock enable in low power mode register"]
pub mod ahb3lpenr;
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
#[doc = "PLLI2SCFGR register accessor: an alias for `Reg<PLLI2SCFGR_SPEC>`"]
pub type PLLI2SCFGR = crate::Reg<plli2scfgr::PLLI2SCFGR_SPEC>;
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;
#[doc = "PLLSAICFGR register accessor: an alias for `Reg<PLLSAICFGR_SPEC>`"]
pub type PLLSAICFGR = crate::Reg<pllsaicfgr::PLLSAICFGR_SPEC>;
#[doc = "RCC PLL configuration register"]
pub mod pllsaicfgr;
#[doc = "DCKCFGR register accessor: an alias for `Reg<DCKCFGR_SPEC>`"]
pub type DCKCFGR = crate::Reg<dckcfgr::DCKCFGR_SPEC>;
#[doc = "RCC Dedicated Clock Configuration Register"]
pub mod dckcfgr;
