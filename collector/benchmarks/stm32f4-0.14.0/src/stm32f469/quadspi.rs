#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - device configuration register"]
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x08 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - flag clear register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x10 - data length register"]
    pub dlr: crate::Reg<dlr::DLR_SPEC>,
    #[doc = "0x14 - communication configuration register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x18 - address register"]
    pub ar: crate::Reg<ar::AR_SPEC>,
    #[doc = "0x1c - ABR"]
    pub abr: crate::Reg<abr::ABR_SPEC>,
    #[doc = "0x20 - data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x24 - polling status mask register"]
    pub psmkr: crate::Reg<psmkr::PSMKR_SPEC>,
    #[doc = "0x28 - polling status match register"]
    pub psmar: crate::Reg<psmar::PSMAR_SPEC>,
    #[doc = "0x2c - polling interval register"]
    pub pir: crate::Reg<pir::PIR_SPEC>,
    #[doc = "0x30 - low-power timeout register"]
    pub lptr: crate::Reg<lptr::LPTR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "DCR register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "device configuration register"]
pub mod dcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "flag clear register"]
pub mod fcr;
#[doc = "DLR register accessor: an alias for `Reg<DLR_SPEC>`"]
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
#[doc = "data length register"]
pub mod dlr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "AR register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "address register"]
pub mod ar;
#[doc = "ABR register accessor: an alias for `Reg<ABR_SPEC>`"]
pub type ABR = crate::Reg<abr::ABR_SPEC>;
#[doc = "ABR"]
pub mod abr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
#[doc = "PSMKR register accessor: an alias for `Reg<PSMKR_SPEC>`"]
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
#[doc = "polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR register accessor: an alias for `Reg<PSMAR_SPEC>`"]
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
#[doc = "polling status match register"]
pub mod psmar;
#[doc = "PIR register accessor: an alias for `Reg<PIR_SPEC>`"]
pub type PIR = crate::Reg<pir::PIR_SPEC>;
#[doc = "polling interval register"]
pub mod pir;
#[doc = "LPTR register accessor: an alias for `Reg<LPTR_SPEC>`"]
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
#[doc = "low-power timeout register"]
pub mod lptr;
