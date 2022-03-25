#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CEC control register"]
    pub cec_cr: crate::Reg<cec_cr::CEC_CR_SPEC>,
    #[doc = "0x04 - CEC configuration register"]
    pub cec_cfgr: crate::Reg<cec_cfgr::CEC_CFGR_SPEC>,
    #[doc = "0x08 - CEC Tx data register"]
    pub cec_txdr: crate::Reg<cec_txdr::CEC_TXDR_SPEC>,
    #[doc = "0x0c - CEC Rx Data Register"]
    pub cec_rxdr: crate::Reg<cec_rxdr::CEC_RXDR_SPEC>,
    #[doc = "0x10 - CEC Interrupt and Status Register"]
    pub cec_isr: crate::Reg<cec_isr::CEC_ISR_SPEC>,
    #[doc = "0x14 - CEC interrupt enable register"]
    pub cec_ier: crate::Reg<cec_ier::CEC_IER_SPEC>,
}
#[doc = "CEC_CR register accessor: an alias for `Reg<CEC_CR_SPEC>`"]
pub type CEC_CR = crate::Reg<cec_cr::CEC_CR_SPEC>;
#[doc = "CEC control register"]
pub mod cec_cr;
#[doc = "CEC_CFGR register accessor: an alias for `Reg<CEC_CFGR_SPEC>`"]
pub type CEC_CFGR = crate::Reg<cec_cfgr::CEC_CFGR_SPEC>;
#[doc = "CEC configuration register"]
pub mod cec_cfgr;
#[doc = "CEC_TXDR register accessor: an alias for `Reg<CEC_TXDR_SPEC>`"]
pub type CEC_TXDR = crate::Reg<cec_txdr::CEC_TXDR_SPEC>;
#[doc = "CEC Tx data register"]
pub mod cec_txdr;
#[doc = "CEC_RXDR register accessor: an alias for `Reg<CEC_RXDR_SPEC>`"]
pub type CEC_RXDR = crate::Reg<cec_rxdr::CEC_RXDR_SPEC>;
#[doc = "CEC Rx Data Register"]
pub mod cec_rxdr;
#[doc = "CEC_ISR register accessor: an alias for `Reg<CEC_ISR_SPEC>`"]
pub type CEC_ISR = crate::Reg<cec_isr::CEC_ISR_SPEC>;
#[doc = "CEC Interrupt and Status Register"]
pub mod cec_isr;
#[doc = "CEC_IER register accessor: an alias for `Reg<CEC_IER_SPEC>`"]
pub type CEC_IER = crate::Reg<cec_ier::CEC_IER_SPEC>;
#[doc = "CEC interrupt enable register"]
pub mod cec_ier;
