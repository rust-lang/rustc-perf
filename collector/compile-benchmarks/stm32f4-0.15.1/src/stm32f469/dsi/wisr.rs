#[doc = "Register `WISR` reader"]
pub struct R(crate::R<WISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RRIF` reader - Regulator Ready Interrupt Flag"]
pub type RRIF_R = crate::BitReader<bool>;
#[doc = "Field `RRS` reader - Regulator Ready Status"]
pub type RRS_R = crate::BitReader<bool>;
#[doc = "Field `PLLUIF` reader - PLL Unlock Interrupt Flag"]
pub type PLLUIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLLIF` reader - PLL Lock Interrupt Flag"]
pub type PLLLIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLLS` reader - PLL Lock Status"]
pub type PLLLS_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - Busy Flag"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `ERIF` reader - End of Refresh Interrupt Flag"]
pub type ERIF_R = crate::BitReader<bool>;
#[doc = "Field `TEIF` reader - Tearing Effect Interrupt Flag"]
pub type TEIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 13 - Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Regulator Ready Status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL Lock Status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 1) != 0)
    }
}
#[doc = "DSI Wrapper Interrupt & Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wisr](index.html) module"]
pub struct WISR_SPEC;
impl crate::RegisterSpec for WISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wisr::R](R) reader structure"]
impl crate::Readable for WISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WISR to value 0"]
impl crate::Resettable for WISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
