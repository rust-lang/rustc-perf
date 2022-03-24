#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
    pub pcgcctl: crate::Reg<pcgcctl::PCGCCTL_SPEC>,
}
#[doc = "PCGCCTL register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)"]
pub mod pcgcctl;
