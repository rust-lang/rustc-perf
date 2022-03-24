#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x14 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch0: CH,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20..0x34 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch1: CH,
    _reserved2: [u8; 0x0c],
    #[doc = "0x40..0x54 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch2: CH,
    _reserved3: [u8; 0x0c],
    #[doc = "0x60..0x74 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch3: CH,
    _reserved4: [u8; 0x0c],
    #[doc = "0x80..0x94 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch4: CH,
    _reserved5: [u8; 0x0c],
    #[doc = "0xa0..0xb4 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch5: CH,
    _reserved6: [u8; 0x0c],
    #[doc = "0xc0..0xd4 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch6: CH,
    _reserved7: [u8; 0x0c],
    #[doc = "0xe0..0xf4 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
    pub ch7: CH,
    _reserved8: [u8; 0x0c],
    #[doc = "0x100..0x13c - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt0: FLT,
    _reserved9: [u8; 0x44],
    #[doc = "0x180..0x1bc - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt1: FLT,
    _reserved10: [u8; 0x44],
    #[doc = "0x200..0x23c - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt2: FLT,
    _reserved11: [u8; 0x44],
    #[doc = "0x280..0x2bc - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
    pub flt3: FLT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - channel configuration y register"]
    pub cfgr1: crate::Reg<self::ch::cfgr1::CFGR1_SPEC>,
    #[doc = "0x04 - channel configuration y register"]
    pub cfgr2: crate::Reg<self::ch::cfgr2::CFGR2_SPEC>,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub awscdr: crate::Reg<self::ch::awscdr::AWSCDR_SPEC>,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub wdatr: crate::Reg<self::ch::wdatr::WDATR_SPEC>,
    #[doc = "0x10 - channel data input register"]
    pub datinr: crate::Reg<self::ch::datinr::DATINR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLT {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<self::flt::cr1::CR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub cr2: crate::Reg<self::flt::cr2::CR2_SPEC>,
    #[doc = "0x08 - interrupt and status register"]
    pub isr: crate::Reg<self::flt::isr::ISR_SPEC>,
    #[doc = "0x0c - interrupt flag clear register"]
    pub icr: crate::Reg<self::flt::icr::ICR_SPEC>,
    #[doc = "0x10 - injected channel group selection register"]
    pub jchgr: crate::Reg<self::flt::jchgr::JCHGR_SPEC>,
    #[doc = "0x14 - filter control register"]
    pub fcr: crate::Reg<self::flt::fcr::FCR_SPEC>,
    #[doc = "0x18 - data register for injected group"]
    pub jdatar: crate::Reg<self::flt::jdatar::JDATAR_SPEC>,
    #[doc = "0x1c - data register for the regular channel"]
    pub rdatar: crate::Reg<self::flt::rdatar::RDATAR_SPEC>,
    #[doc = "0x20 - analog watchdog high threshold register"]
    pub awhtr: crate::Reg<self::flt::awhtr::AWHTR_SPEC>,
    #[doc = "0x24 - analog watchdog low threshold register"]
    pub awltr: crate::Reg<self::flt::awltr::AWLTR_SPEC>,
    #[doc = "0x28 - analog watchdog status register"]
    pub awsr: crate::Reg<self::flt::awsr::AWSR_SPEC>,
    #[doc = "0x2c - analog watchdog clear flag register"]
    pub awcfr: crate::Reg<self::flt::awcfr::AWCFR_SPEC>,
    #[doc = "0x30 - Extremes detector maximum register"]
    pub exmax: crate::Reg<self::flt::exmax::EXMAX_SPEC>,
    #[doc = "0x34 - Extremes detector minimum register"]
    pub exmin: crate::Reg<self::flt::exmin::EXMIN_SPEC>,
    #[doc = "0x38 - conversion timer register"]
    pub cnvtimr: crate::Reg<self::flt::cnvtimr::CNVTIMR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR"]
pub mod flt;
