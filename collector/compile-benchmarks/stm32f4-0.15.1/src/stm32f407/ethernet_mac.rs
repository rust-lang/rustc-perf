#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    pub maccr: crate::Reg<maccr::MACCR_SPEC>,
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    pub macffr: crate::Reg<macffr::MACFFR_SPEC>,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machthr: crate::Reg<machthr::MACHTHR_SPEC>,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtlr: crate::Reg<machtlr::MACHTLR_SPEC>,
    #[doc = "0x10 - Ethernet MAC MII address register"]
    pub macmiiar: crate::Reg<macmiiar::MACMIIAR_SPEC>,
    #[doc = "0x14 - Ethernet MAC MII data register"]
    pub macmiidr: crate::Reg<macmiidr::MACMIIDR_SPEC>,
    #[doc = "0x18 - Ethernet MAC flow control register"]
    pub macfcr: crate::Reg<macfcr::MACFCR_SPEC>,
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    pub macvlantr: crate::Reg<macvlantr::MACVLANTR_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x2c - Ethernet MAC PMT control and status register"]
    pub macpmtcsr: crate::Reg<macpmtcsr::MACPMTCSR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - Ethernet MAC debug register"]
    pub macdbgr: crate::Reg<macdbgr::MACDBGR_SPEC>,
    #[doc = "0x38 - Ethernet MAC interrupt status register"]
    pub macsr: crate::Reg<macsr::MACSR_SPEC>,
    #[doc = "0x3c - Ethernet MAC interrupt mask register"]
    pub macimr: crate::Reg<macimr::MACIMR_SPEC>,
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    pub maca0hr: crate::Reg<maca0hr::MACA0HR_SPEC>,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: crate::Reg<maca0lr::MACA0LR_SPEC>,
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    pub maca1hr: crate::Reg<maca1hr::MACA1HR_SPEC>,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: crate::Reg<maca1lr::MACA1LR_SPEC>,
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    pub maca2hr: crate::Reg<maca2hr::MACA2HR_SPEC>,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2lr: crate::Reg<maca2lr::MACA2LR_SPEC>,
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    pub maca3hr: crate::Reg<maca3hr::MACA3HR_SPEC>,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3lr: crate::Reg<maca3lr::MACA3LR_SPEC>,
}
#[doc = "MACCR register accessor: an alias for `Reg<MACCR_SPEC>`"]
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
#[doc = "Ethernet MAC configuration register"]
pub mod maccr;
#[doc = "MACFFR register accessor: an alias for `Reg<MACFFR_SPEC>`"]
pub type MACFFR = crate::Reg<macffr::MACFFR_SPEC>;
#[doc = "Ethernet MAC frame filter register"]
pub mod macffr;
#[doc = "MACHTHR register accessor: an alias for `Reg<MACHTHR_SPEC>`"]
pub type MACHTHR = crate::Reg<machthr::MACHTHR_SPEC>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "MACHTLR register accessor: an alias for `Reg<MACHTLR_SPEC>`"]
pub type MACHTLR = crate::Reg<machtlr::MACHTLR_SPEC>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "MACMIIAR register accessor: an alias for `Reg<MACMIIAR_SPEC>`"]
pub type MACMIIAR = crate::Reg<macmiiar::MACMIIAR_SPEC>;
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiar;
#[doc = "MACMIIDR register accessor: an alias for `Reg<MACMIIDR_SPEC>`"]
pub type MACMIIDR = crate::Reg<macmiidr::MACMIIDR_SPEC>;
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidr;
#[doc = "MACFCR register accessor: an alias for `Reg<MACFCR_SPEC>`"]
pub type MACFCR = crate::Reg<macfcr::MACFCR_SPEC>;
#[doc = "Ethernet MAC flow control register"]
pub mod macfcr;
#[doc = "MACVLANTR register accessor: an alias for `Reg<MACVLANTR_SPEC>`"]
pub type MACVLANTR = crate::Reg<macvlantr::MACVLANTR_SPEC>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlantr;
#[doc = "MACPMTCSR register accessor: an alias for `Reg<MACPMTCSR_SPEC>`"]
pub type MACPMTCSR = crate::Reg<macpmtcsr::MACPMTCSR_SPEC>;
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtcsr;
#[doc = "MACDBGR register accessor: an alias for `Reg<MACDBGR_SPEC>`"]
pub type MACDBGR = crate::Reg<macdbgr::MACDBGR_SPEC>;
#[doc = "Ethernet MAC debug register"]
pub mod macdbgr;
#[doc = "MACSR register accessor: an alias for `Reg<MACSR_SPEC>`"]
pub type MACSR = crate::Reg<macsr::MACSR_SPEC>;
#[doc = "Ethernet MAC interrupt status register"]
pub mod macsr;
#[doc = "MACIMR register accessor: an alias for `Reg<MACIMR_SPEC>`"]
pub type MACIMR = crate::Reg<macimr::MACIMR_SPEC>;
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "MACA0HR register accessor: an alias for `Reg<MACA0HR_SPEC>`"]
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "MACA0LR register accessor: an alias for `Reg<MACA0LR_SPEC>`"]
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "MACA1HR register accessor: an alias for `Reg<MACA1HR_SPEC>`"]
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "MACA1LR register accessor: an alias for `Reg<MACA1LR_SPEC>`"]
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "MACA2HR register accessor: an alias for `Reg<MACA2HR_SPEC>`"]
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "MACA2LR register accessor: an alias for `Reg<MACA2LR_SPEC>`"]
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "MACA3HR register accessor: an alias for `Reg<MACA3HR_SPEC>`"]
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "MACA3LR register accessor: an alias for `Reg<MACA3LR_SPEC>`"]
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;
