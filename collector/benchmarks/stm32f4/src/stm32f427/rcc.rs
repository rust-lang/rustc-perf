#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: CR,
    #[doc = "0x04 - PLL configuration register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x08 - clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - clock interrupt register"]
    pub cir: CIR,
    #[doc = "0x10 - AHB1 peripheral reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x14 - AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x18 - AHB3 peripheral reset register"]
    pub ahb3rstr: AHB3RSTR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    _reserved9: [u8; 8usize],
    #[doc = "0x30 - AHB1 peripheral clock register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x34 - AHB2 peripheral clock enable register"]
    pub ahb2enr: AHB2ENR,
    #[doc = "0x38 - AHB3 peripheral clock enable register"]
    pub ahb3enr: AHB3ENR,
    _reserved12: [u8; 4usize],
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    _reserved14: [u8; 8usize],
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    pub ahb2lpenr: AHB2LPENR,
    #[doc = "0x58 - AHB3 peripheral clock enable in low power mode register"]
    pub ahb3lpenr: AHB3LPENR,
    _reserved17: [u8; 4usize],
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: APB1LPENR,
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    pub apb2lpenr: APB2LPENR,
    _reserved19: [u8; 8usize],
    #[doc = "0x70 - Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x74 - clock control & status register"]
    pub csr: CSR,
    _reserved21: [u8; 8usize],
    #[doc = "0x80 - spread spectrum clock generation register"]
    pub sscgr: SSCGR,
    #[doc = "0x84 - PLLI2S configuration register"]
    pub plli2scfgr: PLLI2SCFGR,
    #[doc = "0x88 - RCC PLL configuration register"]
    pub pllsaicfgr: PLLSAICFGR,
    #[doc = "0x8c - RCC Dedicated Clock Configuration Register"]
    pub dckcfgr: DCKCFGR,
}
#[doc = "clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](pllcfgr) module"]
pub type PLLCFGR = crate::Reg<u32, _PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFGR;
#[doc = "`read()` method returns [pllcfgr::R](pllcfgr::R) reader structure"]
impl crate::Readable for PLLCFGR {}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](pllcfgr::W) writer structure"]
impl crate::Writable for PLLCFGR {}
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](cir) module"]
pub type CIR = crate::Reg<u32, _CIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIR;
#[doc = "`read()` method returns [cir::R](cir::R) reader structure"]
impl crate::Readable for CIR {}
#[doc = "`write(|w| ..)` method takes [cir::W](cir::W) writer structure"]
impl crate::Writable for CIR {}
#[doc = "clock interrupt register"]
pub mod cir;
#[doc = "AHB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](ahb1rstr) module"]
pub type AHB1RSTR = crate::Reg<u32, _AHB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1RSTR;
#[doc = "`read()` method returns [ahb1rstr::R](ahb1rstr::R) reader structure"]
impl crate::Readable for AHB1RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](ahb1rstr::W) writer structure"]
impl crate::Writable for AHB1RSTR {}
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](ahb2rstr) module"]
pub type AHB2RSTR = crate::Reg<u32, _AHB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2RSTR;
#[doc = "`read()` method returns [ahb2rstr::R](ahb2rstr::R) reader structure"]
impl crate::Readable for AHB2RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](ahb2rstr::W) writer structure"]
impl crate::Writable for AHB2RSTR {}
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "AHB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](ahb3rstr) module"]
pub type AHB3RSTR = crate::Reg<u32, _AHB3RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3RSTR;
#[doc = "`read()` method returns [ahb3rstr::R](ahb3rstr::R) reader structure"]
impl crate::Readable for AHB3RSTR {}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](ahb3rstr::W) writer structure"]
impl crate::Writable for AHB3RSTR {}
#[doc = "AHB3 peripheral reset register"]
pub mod ahb3rstr;
#[doc = "APB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](apb1rstr) module"]
pub type APB1RSTR = crate::Reg<u32, _APB1RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RSTR;
#[doc = "`read()` method returns [apb1rstr::R](apb1rstr::R) reader structure"]
impl crate::Readable for APB1RSTR {}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](apb1rstr::W) writer structure"]
impl crate::Writable for APB1RSTR {}
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](apb2rstr) module"]
pub type APB2RSTR = crate::Reg<u32, _APB2RSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RSTR;
#[doc = "`read()` method returns [apb2rstr::R](apb2rstr::R) reader structure"]
impl crate::Readable for APB2RSTR {}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](apb2rstr::W) writer structure"]
impl crate::Writable for APB2RSTR {}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1 peripheral clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](ahb1enr) module"]
pub type AHB1ENR = crate::Reg<u32, _AHB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1ENR;
#[doc = "`read()` method returns [ahb1enr::R](ahb1enr::R) reader structure"]
impl crate::Readable for AHB1ENR {}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](ahb1enr::W) writer structure"]
impl crate::Writable for AHB1ENR {}
#[doc = "AHB1 peripheral clock register"]
pub mod ahb1enr;
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](ahb2enr) module"]
pub type AHB2ENR = crate::Reg<u32, _AHB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2ENR;
#[doc = "`read()` method returns [ahb2enr::R](ahb2enr::R) reader structure"]
impl crate::Readable for AHB2ENR {}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](ahb2enr::W) writer structure"]
impl crate::Writable for AHB2ENR {}
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](ahb3enr) module"]
pub type AHB3ENR = crate::Reg<u32, _AHB3ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3ENR;
#[doc = "`read()` method returns [ahb3enr::R](ahb3enr::R) reader structure"]
impl crate::Readable for AHB3ENR {}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](ahb3enr::W) writer structure"]
impl crate::Writable for AHB3ENR {}
#[doc = "AHB3 peripheral clock enable register"]
pub mod ahb3enr;
#[doc = "APB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr](apb1enr) module"]
pub type APB1ENR = crate::Reg<u32, _APB1ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1ENR;
#[doc = "`read()` method returns [apb1enr::R](apb1enr::R) reader structure"]
impl crate::Readable for APB1ENR {}
#[doc = "`write(|w| ..)` method takes [apb1enr::W](apb1enr::W) writer structure"]
impl crate::Writable for APB1ENR {}
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](apb2enr) module"]
pub type APB2ENR = crate::Reg<u32, _APB2ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2ENR;
#[doc = "`read()` method returns [apb2enr::R](apb2enr::R) reader structure"]
impl crate::Readable for APB2ENR {}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](apb2enr::W) writer structure"]
impl crate::Writable for APB2ENR {}
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1lpenr](ahb1lpenr) module"]
pub type AHB1LPENR = crate::Reg<u32, _AHB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB1LPENR;
#[doc = "`read()` method returns [ahb1lpenr::R](ahb1lpenr::R) reader structure"]
impl crate::Readable for AHB1LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb1lpenr::W](ahb1lpenr::W) writer structure"]
impl crate::Writable for AHB1LPENR {}
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "AHB2 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](ahb2lpenr) module"]
pub type AHB2LPENR = crate::Reg<u32, _AHB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB2LPENR;
#[doc = "`read()` method returns [ahb2lpenr::R](ahb2lpenr::R) reader structure"]
impl crate::Readable for AHB2LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](ahb2lpenr::W) writer structure"]
impl crate::Writable for AHB2LPENR {}
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "AHB3 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3lpenr](ahb3lpenr) module"]
pub type AHB3LPENR = crate::Reg<u32, _AHB3LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHB3LPENR;
#[doc = "`read()` method returns [ahb3lpenr::R](ahb3lpenr::R) reader structure"]
impl crate::Readable for AHB3LPENR {}
#[doc = "`write(|w| ..)` method takes [ahb3lpenr::W](ahb3lpenr::W) writer structure"]
impl crate::Writable for AHB3LPENR {}
#[doc = "AHB3 peripheral clock enable in low power mode register"]
pub mod ahb3lpenr;
#[doc = "APB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lpenr](apb1lpenr) module"]
pub type APB1LPENR = crate::Reg<u32, _APB1LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LPENR;
#[doc = "`read()` method returns [apb1lpenr::R](apb1lpenr::R) reader structure"]
impl crate::Readable for APB1LPENR {}
#[doc = "`write(|w| ..)` method takes [apb1lpenr::W](apb1lpenr::W) writer structure"]
impl crate::Writable for APB1LPENR {}
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2 peripheral clock enabled in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](apb2lpenr) module"]
pub type APB2LPENR = crate::Reg<u32, _APB2LPENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2LPENR;
#[doc = "`read()` method returns [apb2lpenr::R](apb2lpenr::R) reader structure"]
impl crate::Readable for APB2LPENR {}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](apb2lpenr::W) writer structure"]
impl crate::Writable for APB2LPENR {}
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](bdcr) module"]
pub type BDCR = crate::Reg<u32, _BDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCR;
#[doc = "`read()` method returns [bdcr::R](bdcr::R) reader structure"]
impl crate::Readable for BDCR {}
#[doc = "`write(|w| ..)` method takes [bdcr::W](bdcr::W) writer structure"]
impl crate::Writable for BDCR {}
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "clock control & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "clock control & status register"]
pub mod csr;
#[doc = "spread spectrum clock generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscgr](sscgr) module"]
pub type SSCGR = crate::Reg<u32, _SSCGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCGR;
#[doc = "`read()` method returns [sscgr::R](sscgr::R) reader structure"]
impl crate::Readable for SSCGR {}
#[doc = "`write(|w| ..)` method takes [sscgr::W](sscgr::W) writer structure"]
impl crate::Writable for SSCGR {}
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "PLLI2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plli2scfgr](plli2scfgr) module"]
pub type PLLI2SCFGR = crate::Reg<u32, _PLLI2SCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLI2SCFGR;
#[doc = "`read()` method returns [plli2scfgr::R](plli2scfgr::R) reader structure"]
impl crate::Readable for PLLI2SCFGR {}
#[doc = "`write(|w| ..)` method takes [plli2scfgr::W](plli2scfgr::W) writer structure"]
impl crate::Writable for PLLI2SCFGR {}
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;
#[doc = "RCC PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsaicfgr](pllsaicfgr) module"]
pub type PLLSAICFGR = crate::Reg<u32, _PLLSAICFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLSAICFGR;
#[doc = "`read()` method returns [pllsaicfgr::R](pllsaicfgr::R) reader structure"]
impl crate::Readable for PLLSAICFGR {}
#[doc = "`write(|w| ..)` method takes [pllsaicfgr::W](pllsaicfgr::W) writer structure"]
impl crate::Writable for PLLSAICFGR {}
#[doc = "RCC PLL configuration register"]
pub mod pllsaicfgr;
#[doc = "RCC Dedicated Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dckcfgr](dckcfgr) module"]
pub type DCKCFGR = crate::Reg<u32, _DCKCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCKCFGR;
#[doc = "`read()` method returns [dckcfgr::R](dckcfgr::R) reader structure"]
impl crate::Readable for DCKCFGR {}
#[doc = "`write(|w| ..)` method takes [dckcfgr::W](dckcfgr::W) writer structure"]
impl crate::Writable for DCKCFGR {}
#[doc = "RCC Dedicated Clock Configuration Register"]
pub mod dckcfgr;
