#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host Version Register"]
    pub vr: crate::Reg<vr::VR_SPEC>,
    #[doc = "0x04 - DSI Host Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - DSI HOST Clock Control Register"]
    pub dsihsot_ccr: crate::Reg<dsihsot_ccr::DSIHSOT_CCR_SPEC>,
    #[doc = "0x0c - DSI Host LTDC VCID Register"]
    pub lvcidr: crate::Reg<lvcidr::LVCIDR_SPEC>,
    #[doc = "0x10 - DSI Host LTDC Color Coding Register"]
    pub lcolcr: crate::Reg<lcolcr::LCOLCR_SPEC>,
    #[doc = "0x14 - DSI Host LTDC Polarity Configuration Register"]
    pub lpcr: crate::Reg<lpcr::LPCR_SPEC>,
    #[doc = "0x18 - DSI Host Low-Power Mode Configuration Register"]
    pub lpmcr: crate::Reg<lpmcr::LPMCR_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - DSI Host Protocol Configuration Register"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x30 - DSI Host Generic VCID Register"]
    pub gvcidr: crate::Reg<gvcidr::GVCIDR_SPEC>,
    #[doc = "0x34 - DSI Host Mode Configuration Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x38 - DSI Host Video mode Configuration Register"]
    pub vmcr: crate::Reg<vmcr::VMCR_SPEC>,
    #[doc = "0x3c - DSI Host Video Packet Configuration Register"]
    pub vpcr: crate::Reg<vpcr::VPCR_SPEC>,
    #[doc = "0x40 - DSI Host Video Chunks Configuration Register"]
    pub vccr: crate::Reg<vccr::VCCR_SPEC>,
    #[doc = "0x44 - DSI Host Video Null Packet Configuration Register"]
    pub vnpcr: crate::Reg<vnpcr::VNPCR_SPEC>,
    #[doc = "0x48 - DSI Host Video HSA Configuration Register"]
    pub vhsacr: crate::Reg<vhsacr::VHSACR_SPEC>,
    #[doc = "0x4c - DSI Host Video HBP Configuration Register"]
    pub vhbpcr: crate::Reg<vhbpcr::VHBPCR_SPEC>,
    #[doc = "0x50 - DSI Host Video Line Configuration Register"]
    pub vlcr: crate::Reg<vlcr::VLCR_SPEC>,
    #[doc = "0x54 - DSI Host Video VSA Configuration Register"]
    pub vvsacr: crate::Reg<vvsacr::VVSACR_SPEC>,
    #[doc = "0x58 - DSI Host Video VBP Configuration Register"]
    pub vvbpcr: crate::Reg<vvbpcr::VVBPCR_SPEC>,
    #[doc = "0x5c - DSI Host Video VFP Configuration Register"]
    pub vvfpcr: crate::Reg<vvfpcr::VVFPCR_SPEC>,
    #[doc = "0x60 - DSI Host Video VA Configuration Register"]
    pub vvacr: crate::Reg<vvacr::VVACR_SPEC>,
    #[doc = "0x64 - DSI Host LTDC Command Configuration Register"]
    pub lccr: crate::Reg<lccr::LCCR_SPEC>,
    #[doc = "0x68 - DSI Host Command mode Configuration Register"]
    pub cmcr: crate::Reg<cmcr::CMCR_SPEC>,
    #[doc = "0x6c - DSI Host Generic Header Configuration Register"]
    pub ghcr: crate::Reg<ghcr::GHCR_SPEC>,
    #[doc = "0x70 - DSI Host Generic Payload Data Register"]
    pub gpdr: crate::Reg<gpdr::GPDR_SPEC>,
    #[doc = "0x74 - DSI Host Generic Packet Status Register"]
    pub gpsr: crate::Reg<gpsr::GPSR_SPEC>,
    #[doc = "0x78 - DSI Host Timeout Counter Configuration Register1"]
    pub tccr1: crate::Reg<tccr1::TCCR1_SPEC>,
    #[doc = "0x7c - DSI Host Timeout Counter Configuration Register2"]
    pub tccr2: crate::Reg<tccr2::TCCR2_SPEC>,
    #[doc = "0x80 - DSI Host Timeout Counter Configuration Register3"]
    pub tccr3: crate::Reg<tccr3::TCCR3_SPEC>,
    #[doc = "0x84 - DSI Host Timeout Counter Configuration Register4"]
    pub tccr4: crate::Reg<tccr4::TCCR4_SPEC>,
    #[doc = "0x88 - DSI Host Timeout Counter Configuration Register5"]
    pub tccr5: crate::Reg<tccr5::TCCR5_SPEC>,
    #[doc = "0x8c - DSI Host Timeout Counter Configuration Register6"]
    pub tccr6: crate::Reg<tccr6::TCCR6_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x94 - DSI Host Clock Lane Configuration Register"]
    pub clcr: crate::Reg<clcr::CLCR_SPEC>,
    #[doc = "0x98 - DSI Host Clock Lane Timer Configuration Register"]
    pub cltcr: crate::Reg<cltcr::CLTCR_SPEC>,
    #[doc = "0x9c - DSI Host Data Lane Timer Configuration Register"]
    pub dltcr: crate::Reg<dltcr::DLTCR_SPEC>,
    #[doc = "0xa0 - DSI Host PHY Control Register"]
    pub pctlr: crate::Reg<pctlr::PCTLR_SPEC>,
    #[doc = "0xa4 - DSI Host PHY Configuration Register"]
    pub pcconfr: crate::Reg<pcconfr::PCCONFR_SPEC>,
    #[doc = "0xa8 - DSI Host PHY ULPS Control Register"]
    pub pucr: crate::Reg<pucr::PUCR_SPEC>,
    #[doc = "0xac - DSI Host PHY TX Triggers Configuration Register"]
    pub pttcr: crate::Reg<pttcr::PTTCR_SPEC>,
    #[doc = "0xb0 - DSI Host PHY Status Register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0xbc - DSI Host Interrupt & Status Register 0"]
    pub isr0: crate::Reg<isr0::ISR0_SPEC>,
    #[doc = "0xc0 - DSI Host Interrupt & Status Register 1"]
    pub isr1: crate::Reg<isr1::ISR1_SPEC>,
    #[doc = "0xc4 - DSI Host Interrupt Enable Register 0"]
    pub ier0: crate::Reg<ier0::IER0_SPEC>,
    #[doc = "0xc8 - DSI Host Interrupt Enable Register 1"]
    pub ier1: crate::Reg<ier1::IER1_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0xd8 - DSI Host Force Interrupt Register 0"]
    pub fir0: crate::Reg<fir0::FIR0_SPEC>,
    #[doc = "0xdc - DSI Host Force Interrupt Register 1"]
    pub fir1: crate::Reg<fir1::FIR1_SPEC>,
    _reserved46: [u8; 0x20],
    #[doc = "0x100 - DSI Host Video Shadow Control Register"]
    pub vscr: crate::Reg<vscr::VSCR_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0x10c - DSI Host LTDC Current VCID Register"]
    pub lcvcidr: crate::Reg<lcvcidr::LCVCIDR_SPEC>,
    #[doc = "0x110 - DSI Host LTDC Current Color Coding Register"]
    pub lcccr: crate::Reg<lcccr::LCCCR_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x118 - DSI Host Low-power Mode Current Configuration Register"]
    pub lpmccr: crate::Reg<lpmccr::LPMCCR_SPEC>,
    _reserved50: [u8; 0x1c],
    #[doc = "0x138 - DSI Host Video mode Current Configuration Register"]
    pub vmccr: crate::Reg<vmccr::VMCCR_SPEC>,
    #[doc = "0x13c - DSI Host Video Packet Current Configuration Register"]
    pub vpccr: crate::Reg<vpccr::VPCCR_SPEC>,
    #[doc = "0x140 - DSI Host Video Chunks Current Configuration Register"]
    pub vcccr: crate::Reg<vcccr::VCCCR_SPEC>,
    #[doc = "0x144 - DSI Host Video Null Packet Current Configuration Register"]
    pub vnpccr: crate::Reg<vnpccr::VNPCCR_SPEC>,
    #[doc = "0x148 - DSI Host Video HSA Current Configuration Register"]
    pub vhsaccr: crate::Reg<vhsaccr::VHSACCR_SPEC>,
    #[doc = "0x14c - DSI Host Video HBP Current Configuration Register"]
    pub vhbpccr: crate::Reg<vhbpccr::VHBPCCR_SPEC>,
    #[doc = "0x150 - DSI Host Video Line Current Configuration Register"]
    pub vlccr: crate::Reg<vlccr::VLCCR_SPEC>,
    #[doc = "0x154 - DSI Host Video VSA Current Configuration Register"]
    pub vvsaccr: crate::Reg<vvsaccr::VVSACCR_SPEC>,
    #[doc = "0x158 - DSI Host Video VBP Current Configuration Register"]
    pub vvbpccr: crate::Reg<vvbpccr::VVBPCCR_SPEC>,
    #[doc = "0x15c - DSI Host Video VFP Current Configuration Register"]
    pub vvfpccr: crate::Reg<vvfpccr::VVFPCCR_SPEC>,
    #[doc = "0x160 - DSI Host Video VA Current Configuration Register"]
    pub vvaccr: crate::Reg<vvaccr::VVACCR_SPEC>,
    _reserved61: [u8; 0x029c],
    #[doc = "0x400 - DSI Wrapper Configuration Register"]
    pub wcfgr: crate::Reg<wcfgr::WCFGR_SPEC>,
    #[doc = "0x404 - DSI Wrapper Control Register"]
    pub wcr: crate::Reg<wcr::WCR_SPEC>,
    #[doc = "0x408 - DSI Wrapper Interrupt Enable Register"]
    pub wier: crate::Reg<wier::WIER_SPEC>,
    #[doc = "0x40c - DSI Wrapper Interrupt & Status Register"]
    pub wisr: crate::Reg<wisr::WISR_SPEC>,
    #[doc = "0x410 - DSI Wrapper Interrupt Flag Clear Register"]
    pub wifcr: crate::Reg<wifcr::WIFCR_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x418 - DSI Wrapper PHY Configuration Register 1"]
    pub wpcr1: crate::Reg<wpcr1::WPCR1_SPEC>,
    #[doc = "0x41c - DSI Wrapper PHY Configuration Register 2"]
    pub wpcr2: crate::Reg<wpcr2::WPCR2_SPEC>,
    #[doc = "0x420 - DSI Wrapper PHY Configuration Register 3"]
    pub wpcr3: crate::Reg<wpcr3::WPCR3_SPEC>,
    #[doc = "0x424 - DSI_WPCR4"]
    pub wpcr4: crate::Reg<wpcr4::WPCR4_SPEC>,
    #[doc = "0x428 - DSI Wrapper PHY Configuration Register 5"]
    pub wpcr5: crate::Reg<wpcr5::WPCR5_SPEC>,
    _reserved71: [u8; 0x04],
    #[doc = "0x430 - DSI Wrapper Regulator and PLL Control Register"]
    pub wrpcr: crate::Reg<wrpcr::WRPCR_SPEC>,
}
#[doc = "VR register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "DSI Host Version Register"]
pub mod vr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DSI Host Control Register"]
pub mod cr;
#[doc = "DSIHSOT_CCR register accessor: an alias for `Reg<DSIHSOT_CCR_SPEC>`"]
pub type DSIHSOT_CCR = crate::Reg<dsihsot_ccr::DSIHSOT_CCR_SPEC>;
#[doc = "DSI HOST Clock Control Register"]
pub mod dsihsot_ccr;
#[doc = "LVCIDR register accessor: an alias for `Reg<LVCIDR_SPEC>`"]
pub type LVCIDR = crate::Reg<lvcidr::LVCIDR_SPEC>;
#[doc = "DSI Host LTDC VCID Register"]
pub mod lvcidr;
#[doc = "LCOLCR register accessor: an alias for `Reg<LCOLCR_SPEC>`"]
pub type LCOLCR = crate::Reg<lcolcr::LCOLCR_SPEC>;
#[doc = "DSI Host LTDC Color Coding Register"]
pub mod lcolcr;
#[doc = "LPCR register accessor: an alias for `Reg<LPCR_SPEC>`"]
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
#[doc = "DSI Host LTDC Polarity Configuration Register"]
pub mod lpcr;
#[doc = "LPMCR register accessor: an alias for `Reg<LPMCR_SPEC>`"]
pub type LPMCR = crate::Reg<lpmcr::LPMCR_SPEC>;
#[doc = "DSI Host Low-Power Mode Configuration Register"]
pub mod lpmcr;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "DSI Host Protocol Configuration Register"]
pub mod pcr;
#[doc = "GVCIDR register accessor: an alias for `Reg<GVCIDR_SPEC>`"]
pub type GVCIDR = crate::Reg<gvcidr::GVCIDR_SPEC>;
#[doc = "DSI Host Generic VCID Register"]
pub mod gvcidr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "DSI Host Mode Configuration Register"]
pub mod mcr;
#[doc = "VMCR register accessor: an alias for `Reg<VMCR_SPEC>`"]
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
#[doc = "DSI Host Video mode Configuration Register"]
pub mod vmcr;
#[doc = "VPCR register accessor: an alias for `Reg<VPCR_SPEC>`"]
pub type VPCR = crate::Reg<vpcr::VPCR_SPEC>;
#[doc = "DSI Host Video Packet Configuration Register"]
pub mod vpcr;
#[doc = "VCCR register accessor: an alias for `Reg<VCCR_SPEC>`"]
pub type VCCR = crate::Reg<vccr::VCCR_SPEC>;
#[doc = "DSI Host Video Chunks Configuration Register"]
pub mod vccr;
#[doc = "VNPCR register accessor: an alias for `Reg<VNPCR_SPEC>`"]
pub type VNPCR = crate::Reg<vnpcr::VNPCR_SPEC>;
#[doc = "DSI Host Video Null Packet Configuration Register"]
pub mod vnpcr;
#[doc = "VHSACR register accessor: an alias for `Reg<VHSACR_SPEC>`"]
pub type VHSACR = crate::Reg<vhsacr::VHSACR_SPEC>;
#[doc = "DSI Host Video HSA Configuration Register"]
pub mod vhsacr;
#[doc = "VHBPCR register accessor: an alias for `Reg<VHBPCR_SPEC>`"]
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCR_SPEC>;
#[doc = "DSI Host Video HBP Configuration Register"]
pub mod vhbpcr;
#[doc = "VLCR register accessor: an alias for `Reg<VLCR_SPEC>`"]
pub type VLCR = crate::Reg<vlcr::VLCR_SPEC>;
#[doc = "DSI Host Video Line Configuration Register"]
pub mod vlcr;
#[doc = "VVSACR register accessor: an alias for `Reg<VVSACR_SPEC>`"]
pub type VVSACR = crate::Reg<vvsacr::VVSACR_SPEC>;
#[doc = "DSI Host Video VSA Configuration Register"]
pub mod vvsacr;
#[doc = "VVBPCR register accessor: an alias for `Reg<VVBPCR_SPEC>`"]
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCR_SPEC>;
#[doc = "DSI Host Video VBP Configuration Register"]
pub mod vvbpcr;
#[doc = "VVFPCR register accessor: an alias for `Reg<VVFPCR_SPEC>`"]
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCR_SPEC>;
#[doc = "DSI Host Video VFP Configuration Register"]
pub mod vvfpcr;
#[doc = "VVACR register accessor: an alias for `Reg<VVACR_SPEC>`"]
pub type VVACR = crate::Reg<vvacr::VVACR_SPEC>;
#[doc = "DSI Host Video VA Configuration Register"]
pub mod vvacr;
#[doc = "LCCR register accessor: an alias for `Reg<LCCR_SPEC>`"]
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
#[doc = "DSI Host LTDC Command Configuration Register"]
pub mod lccr;
#[doc = "CMCR register accessor: an alias for `Reg<CMCR_SPEC>`"]
pub type CMCR = crate::Reg<cmcr::CMCR_SPEC>;
#[doc = "DSI Host Command mode Configuration Register"]
pub mod cmcr;
#[doc = "GHCR register accessor: an alias for `Reg<GHCR_SPEC>`"]
pub type GHCR = crate::Reg<ghcr::GHCR_SPEC>;
#[doc = "DSI Host Generic Header Configuration Register"]
pub mod ghcr;
#[doc = "GPDR register accessor: an alias for `Reg<GPDR_SPEC>`"]
pub type GPDR = crate::Reg<gpdr::GPDR_SPEC>;
#[doc = "DSI Host Generic Payload Data Register"]
pub mod gpdr;
#[doc = "GPSR register accessor: an alias for `Reg<GPSR_SPEC>`"]
pub type GPSR = crate::Reg<gpsr::GPSR_SPEC>;
#[doc = "DSI Host Generic Packet Status Register"]
pub mod gpsr;
#[doc = "TCCR1 register accessor: an alias for `Reg<TCCR1_SPEC>`"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register1"]
pub mod tccr1;
#[doc = "TCCR2 register accessor: an alias for `Reg<TCCR2_SPEC>`"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register2"]
pub mod tccr2;
#[doc = "TCCR3 register accessor: an alias for `Reg<TCCR3_SPEC>`"]
pub type TCCR3 = crate::Reg<tccr3::TCCR3_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register3"]
pub mod tccr3;
#[doc = "TCCR4 register accessor: an alias for `Reg<TCCR4_SPEC>`"]
pub type TCCR4 = crate::Reg<tccr4::TCCR4_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register4"]
pub mod tccr4;
#[doc = "TCCR5 register accessor: an alias for `Reg<TCCR5_SPEC>`"]
pub type TCCR5 = crate::Reg<tccr5::TCCR5_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register5"]
pub mod tccr5;
#[doc = "TCCR6 register accessor: an alias for `Reg<TCCR6_SPEC>`"]
pub type TCCR6 = crate::Reg<tccr6::TCCR6_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register6"]
pub mod tccr6;
#[doc = "CLCR register accessor: an alias for `Reg<CLCR_SPEC>`"]
pub type CLCR = crate::Reg<clcr::CLCR_SPEC>;
#[doc = "DSI Host Clock Lane Configuration Register"]
pub mod clcr;
#[doc = "CLTCR register accessor: an alias for `Reg<CLTCR_SPEC>`"]
pub type CLTCR = crate::Reg<cltcr::CLTCR_SPEC>;
#[doc = "DSI Host Clock Lane Timer Configuration Register"]
pub mod cltcr;
#[doc = "DLTCR register accessor: an alias for `Reg<DLTCR_SPEC>`"]
pub type DLTCR = crate::Reg<dltcr::DLTCR_SPEC>;
#[doc = "DSI Host Data Lane Timer Configuration Register"]
pub mod dltcr;
#[doc = "PCTLR register accessor: an alias for `Reg<PCTLR_SPEC>`"]
pub type PCTLR = crate::Reg<pctlr::PCTLR_SPEC>;
#[doc = "DSI Host PHY Control Register"]
pub mod pctlr;
#[doc = "PCCONFR register accessor: an alias for `Reg<PCCONFR_SPEC>`"]
pub type PCCONFR = crate::Reg<pcconfr::PCCONFR_SPEC>;
#[doc = "DSI Host PHY Configuration Register"]
pub mod pcconfr;
#[doc = "PUCR register accessor: an alias for `Reg<PUCR_SPEC>`"]
pub type PUCR = crate::Reg<pucr::PUCR_SPEC>;
#[doc = "DSI Host PHY ULPS Control Register"]
pub mod pucr;
#[doc = "PTTCR register accessor: an alias for `Reg<PTTCR_SPEC>`"]
pub type PTTCR = crate::Reg<pttcr::PTTCR_SPEC>;
#[doc = "DSI Host PHY TX Triggers Configuration Register"]
pub mod pttcr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "DSI Host PHY Status Register"]
pub mod psr;
#[doc = "ISR0 register accessor: an alias for `Reg<ISR0_SPEC>`"]
pub type ISR0 = crate::Reg<isr0::ISR0_SPEC>;
#[doc = "DSI Host Interrupt & Status Register 0"]
pub mod isr0;
#[doc = "ISR1 register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "DSI Host Interrupt & Status Register 1"]
pub mod isr1;
#[doc = "IER0 register accessor: an alias for `Reg<IER0_SPEC>`"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "DSI Host Interrupt Enable Register 0"]
pub mod ier0;
#[doc = "IER1 register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "DSI Host Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "FIR0 register accessor: an alias for `Reg<FIR0_SPEC>`"]
pub type FIR0 = crate::Reg<fir0::FIR0_SPEC>;
#[doc = "DSI Host Force Interrupt Register 0"]
pub mod fir0;
#[doc = "FIR1 register accessor: an alias for `Reg<FIR1_SPEC>`"]
pub type FIR1 = crate::Reg<fir1::FIR1_SPEC>;
#[doc = "DSI Host Force Interrupt Register 1"]
pub mod fir1;
#[doc = "VSCR register accessor: an alias for `Reg<VSCR_SPEC>`"]
pub type VSCR = crate::Reg<vscr::VSCR_SPEC>;
#[doc = "DSI Host Video Shadow Control Register"]
pub mod vscr;
#[doc = "LCVCIDR register accessor: an alias for `Reg<LCVCIDR_SPEC>`"]
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDR_SPEC>;
#[doc = "DSI Host LTDC Current VCID Register"]
pub mod lcvcidr;
#[doc = "LCCCR register accessor: an alias for `Reg<LCCCR_SPEC>`"]
pub type LCCCR = crate::Reg<lcccr::LCCCR_SPEC>;
#[doc = "DSI Host LTDC Current Color Coding Register"]
pub mod lcccr;
#[doc = "LPMCCR register accessor: an alias for `Reg<LPMCCR_SPEC>`"]
pub type LPMCCR = crate::Reg<lpmccr::LPMCCR_SPEC>;
#[doc = "DSI Host Low-power Mode Current Configuration Register"]
pub mod lpmccr;
#[doc = "VMCCR register accessor: an alias for `Reg<VMCCR_SPEC>`"]
pub type VMCCR = crate::Reg<vmccr::VMCCR_SPEC>;
#[doc = "DSI Host Video mode Current Configuration Register"]
pub mod vmccr;
#[doc = "VPCCR register accessor: an alias for `Reg<VPCCR_SPEC>`"]
pub type VPCCR = crate::Reg<vpccr::VPCCR_SPEC>;
#[doc = "DSI Host Video Packet Current Configuration Register"]
pub mod vpccr;
#[doc = "VCCCR register accessor: an alias for `Reg<VCCCR_SPEC>`"]
pub type VCCCR = crate::Reg<vcccr::VCCCR_SPEC>;
#[doc = "DSI Host Video Chunks Current Configuration Register"]
pub mod vcccr;
#[doc = "VNPCCR register accessor: an alias for `Reg<VNPCCR_SPEC>`"]
pub type VNPCCR = crate::Reg<vnpccr::VNPCCR_SPEC>;
#[doc = "DSI Host Video Null Packet Current Configuration Register"]
pub mod vnpccr;
#[doc = "VHSACCR register accessor: an alias for `Reg<VHSACCR_SPEC>`"]
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCR_SPEC>;
#[doc = "DSI Host Video HSA Current Configuration Register"]
pub mod vhsaccr;
#[doc = "VHBPCCR register accessor: an alias for `Reg<VHBPCCR_SPEC>`"]
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCR_SPEC>;
#[doc = "DSI Host Video HBP Current Configuration Register"]
pub mod vhbpccr;
#[doc = "VLCCR register accessor: an alias for `Reg<VLCCR_SPEC>`"]
pub type VLCCR = crate::Reg<vlccr::VLCCR_SPEC>;
#[doc = "DSI Host Video Line Current Configuration Register"]
pub mod vlccr;
#[doc = "VVSACCR register accessor: an alias for `Reg<VVSACCR_SPEC>`"]
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCR_SPEC>;
#[doc = "DSI Host Video VSA Current Configuration Register"]
pub mod vvsaccr;
#[doc = "VVBPCCR register accessor: an alias for `Reg<VVBPCCR_SPEC>`"]
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCR_SPEC>;
#[doc = "DSI Host Video VBP Current Configuration Register"]
pub mod vvbpccr;
#[doc = "VVFPCCR register accessor: an alias for `Reg<VVFPCCR_SPEC>`"]
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCR_SPEC>;
#[doc = "DSI Host Video VFP Current Configuration Register"]
pub mod vvfpccr;
#[doc = "VVACCR register accessor: an alias for `Reg<VVACCR_SPEC>`"]
pub type VVACCR = crate::Reg<vvaccr::VVACCR_SPEC>;
#[doc = "DSI Host Video VA Current Configuration Register"]
pub mod vvaccr;
#[doc = "WCFGR register accessor: an alias for `Reg<WCFGR_SPEC>`"]
pub type WCFGR = crate::Reg<wcfgr::WCFGR_SPEC>;
#[doc = "DSI Wrapper Configuration Register"]
pub mod wcfgr;
#[doc = "WCR register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "DSI Wrapper Control Register"]
pub mod wcr;
#[doc = "WIER register accessor: an alias for `Reg<WIER_SPEC>`"]
pub type WIER = crate::Reg<wier::WIER_SPEC>;
#[doc = "DSI Wrapper Interrupt Enable Register"]
pub mod wier;
#[doc = "WISR register accessor: an alias for `Reg<WISR_SPEC>`"]
pub type WISR = crate::Reg<wisr::WISR_SPEC>;
#[doc = "DSI Wrapper Interrupt & Status Register"]
pub mod wisr;
#[doc = "WIFCR register accessor: an alias for `Reg<WIFCR_SPEC>`"]
pub type WIFCR = crate::Reg<wifcr::WIFCR_SPEC>;
#[doc = "DSI Wrapper Interrupt Flag Clear Register"]
pub mod wifcr;
#[doc = "WPCR1 register accessor: an alias for `Reg<WPCR1_SPEC>`"]
pub type WPCR1 = crate::Reg<wpcr1::WPCR1_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 1"]
pub mod wpcr1;
#[doc = "WPCR2 register accessor: an alias for `Reg<WPCR2_SPEC>`"]
pub type WPCR2 = crate::Reg<wpcr2::WPCR2_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 2"]
pub mod wpcr2;
#[doc = "WPCR3 register accessor: an alias for `Reg<WPCR3_SPEC>`"]
pub type WPCR3 = crate::Reg<wpcr3::WPCR3_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 3"]
pub mod wpcr3;
#[doc = "WPCR4 register accessor: an alias for `Reg<WPCR4_SPEC>`"]
pub type WPCR4 = crate::Reg<wpcr4::WPCR4_SPEC>;
#[doc = "DSI_WPCR4"]
pub mod wpcr4;
#[doc = "WPCR5 register accessor: an alias for `Reg<WPCR5_SPEC>`"]
pub type WPCR5 = crate::Reg<wpcr5::WPCR5_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 5"]
pub mod wpcr5;
#[doc = "WRPCR register accessor: an alias for `Reg<WRPCR_SPEC>`"]
pub type WRPCR = crate::Reg<wrpcr::WRPCR_SPEC>;
#[doc = "DSI Wrapper Regulator and PLL Control Register"]
pub mod wrpcr;
