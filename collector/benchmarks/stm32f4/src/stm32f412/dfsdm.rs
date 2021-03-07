#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch0: CH,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch1: CH,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch2: CH,
    _reserved3: [u8; 12usize],
    #[doc = "0x60 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch3: CH,
    _reserved4: [u8; 12usize],
    #[doc = "0x80 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch4: CH,
    _reserved5: [u8; 12usize],
    #[doc = "0xa0 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch5: CH,
    _reserved6: [u8; 12usize],
    #[doc = "0xc0 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch6: CH,
    _reserved7: [u8; 12usize],
    #[doc = "0xe0 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch7: CH,
    _reserved8: [u8; 12usize],
    _reserved_8_flt0: [u8; 60usize],
    _reserved9: [u8; 196usize],
    _reserved_9_flt1: [u8; 60usize],
    _reserved10: [u8; 196usize],
    _reserved_10_flt2: [u8; 60usize],
    _reserved11: [u8; 196usize],
    _reserved_11_flt3: [u8; 60usize],
}
impl RegisterBlock {
    #[doc = "0x100 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt0(&self) -> &FLT {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const FLT) }
    }
    #[doc = "0x100 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt0_mut(&self) -> &mut FLT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut FLT) }
    }
    #[doc = "0x104 - control register 2"]
    #[inline(always)]
    pub fn dfsdm0_cr2(&self) -> &DFSDM0_CR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const DFSDM0_CR2) }
    }
    #[doc = "0x104 - control register 2"]
    #[inline(always)]
    pub fn dfsdm0_cr2_mut(&self) -> &mut DFSDM0_CR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut DFSDM0_CR2) }
    }
    #[doc = "0x200 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt1(&self) -> &FLT {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const FLT) }
    }
    #[doc = "0x200 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt1_mut(&self) -> &mut FLT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut FLT) }
    }
    #[doc = "0x204 - control register 2"]
    #[inline(always)]
    pub fn dfsdm1_cr2(&self) -> &DFSDM1_CR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const DFSDM1_CR2) }
    }
    #[doc = "0x204 - control register 2"]
    #[inline(always)]
    pub fn dfsdm1_cr2_mut(&self) -> &mut DFSDM1_CR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut DFSDM1_CR2) }
    }
    #[doc = "0x300 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt2(&self) -> &FLT {
        unsafe { &*(((self as *const Self) as *const u8).add(768usize) as *const FLT) }
    }
    #[doc = "0x300 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt2_mut(&self) -> &mut FLT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(768usize) as *mut FLT) }
    }
    #[doc = "0x304 - control register 2"]
    #[inline(always)]
    pub fn dfsdm2_cr2(&self) -> &DFSDM2_CR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(772usize) as *const DFSDM2_CR2) }
    }
    #[doc = "0x304 - control register 2"]
    #[inline(always)]
    pub fn dfsdm2_cr2_mut(&self) -> &mut DFSDM2_CR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(772usize) as *mut DFSDM2_CR2) }
    }
    #[doc = "0x400 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt3(&self) -> &FLT {
        unsafe { &*(((self as *const Self) as *const u8).add(1024usize) as *const FLT) }
    }
    #[doc = "0x400 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    #[inline(always)]
    pub fn flt3_mut(&self) -> &mut FLT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1024usize) as *mut FLT) }
    }
    #[doc = "0x404 - control register 2"]
    #[inline(always)]
    pub fn dfsdm3_cr2(&self) -> &DFSDM3_CR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1028usize) as *const DFSDM3_CR2) }
    }
    #[doc = "0x404 - control register 2"]
    #[inline(always)]
    pub fn dfsdm3_cr2_mut(&self) -> &mut DFSDM3_CR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1028usize) as *mut DFSDM3_CR2) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - channel configuration y register"]
    pub cfgr1: self::ch::CFGR1,
    #[doc = "0x04 - channel configuration y register"]
    pub cfgr2: self::ch::CFGR2,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub awscdr: self::ch::AWSCDR,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub wdatr: self::ch::WDATR,
    #[doc = "0x10 - channel data input register"]
    pub datinr: self::ch::DATINR,
}
#[doc = r"Register block"]
#[doc = "DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLT {
    #[doc = "0x00 - control register 1"]
    pub cr2: self::flt::CR2,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - interrupt and status register"]
    pub isr: self::flt::ISR,
    #[doc = "0x0c - interrupt flag clear register"]
    pub icr: self::flt::ICR,
    #[doc = "0x10 - injected channel group selection register"]
    pub jchgr: self::flt::JCHGR,
    #[doc = "0x14 - filter control register"]
    pub fcr: self::flt::FCR,
    #[doc = "0x18 - data register for injected group"]
    pub jdatar: self::flt::JDATAR,
    #[doc = "0x1c - data register for the regular channel"]
    pub rdatar: self::flt::RDATAR,
    #[doc = "0x20 - analog watchdog high threshold register"]
    pub awhtr: self::flt::AWHTR,
    #[doc = "0x24 - analog watchdog low threshold register"]
    pub awltr: self::flt::AWLTR,
    #[doc = "0x28 - analog watchdog status register"]
    pub awsr: self::flt::AWSR,
    #[doc = "0x2c - analog watchdog clear flag register"]
    pub awcfr: self::flt::AWCFR,
    #[doc = "0x30 - Extremes detector maximum register"]
    pub exmax: self::flt::EXMAX,
    #[doc = "0x34 - Extremes detector minimum register"]
    pub exmin: self::flt::EXMIN,
    #[doc = "0x38 - conversion timer register"]
    pub cnvtimr: self::flt::CNVTIMR,
}
#[doc = r"Register block"]
#[doc = "Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
pub mod flt;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_cr2](dfsdm0_cr2) module"]
pub type DFSDM0_CR2 = crate::Reg<u32, _DFSDM0_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM0_CR2;
#[doc = "`read()` method returns [dfsdm0_cr2::R](dfsdm0_cr2::R) reader structure"]
impl crate::Readable for DFSDM0_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm0_cr2::W](dfsdm0_cr2::W) writer structure"]
impl crate::Writable for DFSDM0_CR2 {}
#[doc = "control register 2"]
pub mod dfsdm0_cr2;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm1_cr2](dfsdm1_cr2) module"]
pub type DFSDM1_CR2 = crate::Reg<u32, _DFSDM1_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM1_CR2;
#[doc = "`read()` method returns [dfsdm1_cr2::R](dfsdm1_cr2::R) reader structure"]
impl crate::Readable for DFSDM1_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm1_cr2::W](dfsdm1_cr2::W) writer structure"]
impl crate::Writable for DFSDM1_CR2 {}
#[doc = "control register 2"]
pub mod dfsdm1_cr2;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_cr2](dfsdm2_cr2) module"]
pub type DFSDM2_CR2 = crate::Reg<u32, _DFSDM2_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM2_CR2;
#[doc = "`read()` method returns [dfsdm2_cr2::R](dfsdm2_cr2::R) reader structure"]
impl crate::Readable for DFSDM2_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm2_cr2::W](dfsdm2_cr2::W) writer structure"]
impl crate::Writable for DFSDM2_CR2 {}
#[doc = "control register 2"]
pub mod dfsdm2_cr2;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_cr2](dfsdm3_cr2) module"]
pub type DFSDM3_CR2 = crate::Reg<u32, _DFSDM3_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM3_CR2;
#[doc = "`read()` method returns [dfsdm3_cr2::R](dfsdm3_cr2::R) reader structure"]
impl crate::Readable for DFSDM3_CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm3_cr2::W](dfsdm3_cr2::W) writer structure"]
impl crate::Writable for DFSDM3_CR2 {}
#[doc = "control register 2"]
pub mod dfsdm3_cr2;
