#[doc = "Register `TSTR` reader"]
pub struct R(crate::R<TSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALARMOUTTYPE` reader - AFO_ALARM output type"]
pub type ALARMOUTTYPE_R = crate::BitReader<bool>;
#[doc = "Field `TSINSEL` reader - TIMESTAMP mapping"]
pub type TSINSEL_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1INSEL` reader - TAMPER1 mapping"]
pub type TAMP1INSEL_R = crate::BitReader<bool>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub type TAMP1TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub type TAMP1E_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
}
#[doc = "time stamp time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstr](index.html) module"]
pub struct TSTR_SPEC;
impl crate::RegisterSpec for TSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstr::R](R) reader structure"]
impl crate::Readable for TSTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSTR to value 0"]
impl crate::Resettable for TSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
