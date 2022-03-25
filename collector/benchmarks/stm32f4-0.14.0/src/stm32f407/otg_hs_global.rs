#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub gotgctl: crate::Reg<gotgctl::GOTGCTL_SPEC>,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub gotgint: crate::Reg<gotgint::GOTGINT_SPEC>,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub gahbcfg: crate::Reg<gahbcfg::GAHBCFG_SPEC>,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub gusbcfg: crate::Reg<gusbcfg::GUSBCFG_SPEC>,
    #[doc = "0x10 - OTG_HS reset register"]
    pub grstctl: crate::Reg<grstctl::GRSTCTL_SPEC>,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub gintsts: crate::Reg<gintsts::GINTSTS_SPEC>,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub gintmsk: crate::Reg<gintmsk::GINTMSK_SPEC>,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub grxfsiz: crate::Reg<grxfsiz::GRXFSIZ_SPEC>,
    _reserved_10_tx0fsiz: [u8; 0x04],
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub gnptxsts: crate::Reg<gnptxsts::GNPTXSTS_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub gccfg: crate::Reg<gccfg::GCCFG_SPEC>,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub cid: crate::Reg<cid::CID_SPEC>,
    _reserved14: [u8; 0xc0],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub hptxfsiz: crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf1: crate::Reg<dieptxf::DIEPTXF_SPEC>,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf2: crate::Reg<dieptxf::DIEPTXF_SPEC>,
    #[doc = "0x10c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf3: crate::Reg<dieptxf::DIEPTXF_SPEC>,
    #[doc = "0x110 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf4: crate::Reg<dieptxf::DIEPTXF_SPEC>,
    #[doc = "0x114 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub dieptxf5: crate::Reg<dieptxf::DIEPTXF_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_HS Receive status debug read register (peripheral mode mode)"]
    #[inline(always)]
    pub fn grxstsr_peripheral(&self) -> &crate::Reg<grxstsr_peripheral::GRXSTSR_PERIPHERAL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<grxstsr_peripheral::GRXSTSR_PERIPHERAL_SPEC>)
        }
    }
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    #[inline(always)]
    pub fn grxstsr_host(&self) -> &crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (peripheral mode)"]
    #[inline(always)]
    pub fn grxstsp_peripheral(&self) -> &crate::Reg<grxstsp_peripheral::GRXSTSP_PERIPHERAL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<grxstsp_peripheral::GRXSTSP_PERIPHERAL_SPEC>)
        }
    }
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    #[inline(always)]
    pub fn grxstsp_host(&self) -> &crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>)
        }
    }
    #[doc = "0x28 - Endpoint 0 transmit FIFO size (peripheral mode)"]
    #[inline(always)]
    pub fn tx0fsiz(&self) -> &crate::Reg<tx0fsiz::TX0FSIZ_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<tx0fsiz::TX0FSIZ_SPEC>)
        }
    }
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub fn gnptxfsiz(&self) -> &crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>)
        }
    }
}
#[doc = "GOTGCTL register accessor: an alias for `Reg<GOTGCTL_SPEC>`"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "OTG_HS control and status register"]
pub mod gotgctl;
#[doc = "GOTGINT register accessor: an alias for `Reg<GOTGINT_SPEC>`"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "OTG_HS interrupt register"]
pub mod gotgint;
#[doc = "GAHBCFG register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "OTG_HS AHB configuration register"]
pub mod gahbcfg;
#[doc = "GUSBCFG register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "OTG_HS USB configuration register"]
pub mod gusbcfg;
#[doc = "GRSTCTL register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "OTG_HS reset register"]
pub mod grstctl;
#[doc = "GINTSTS register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "OTG_HS core interrupt register"]
pub mod gintsts;
#[doc = "GINTMSK register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "OTG_HS interrupt mask register"]
pub mod gintmsk;
#[doc = "GRXSTSR_Host register accessor: an alias for `Reg<GRXSTSR_HOST_SPEC>`"]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod grxstsr_host;
#[doc = "GRXSTSP_Host register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`"]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod grxstsp_host;
#[doc = "GRXFSIZ register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "OTG_HS Receive FIFO size register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ register accessor: an alias for `Reg<GNPTXFSIZ_SPEC>`"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod gnptxfsiz;
#[doc = "TX0FSIZ register accessor: an alias for `Reg<TX0FSIZ_SPEC>`"]
pub type TX0FSIZ = crate::Reg<tx0fsiz::TX0FSIZ_SPEC>;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod tx0fsiz;
#[doc = "GNPTXSTS register accessor: an alias for `Reg<GNPTXSTS_SPEC>`"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod gnptxsts;
#[doc = "GCCFG register accessor: an alias for `Reg<GCCFG_SPEC>`"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "OTG_HS general core configuration register"]
pub mod gccfg;
#[doc = "CID register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "OTG_HS core ID register"]
pub mod cid;
#[doc = "HPTXFSIZ register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF register accessor: an alias for `Reg<DIEPTXF_SPEC>`"]
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXF_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod dieptxf;
#[doc = "GRXSTSR_Peripheral register accessor: an alias for `Reg<GRXSTSR_PERIPHERAL_SPEC>`"]
pub type GRXSTSR_PERIPHERAL = crate::Reg<grxstsr_peripheral::GRXSTSR_PERIPHERAL_SPEC>;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod grxstsr_peripheral;
#[doc = "GRXSTSP_Peripheral register accessor: an alias for `Reg<GRXSTSP_PERIPHERAL_SPEC>`"]
pub type GRXSTSP_PERIPHERAL = crate::Reg<grxstsp_peripheral::GRXSTSP_PERIPHERAL_SPEC>;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod grxstsp_peripheral;
