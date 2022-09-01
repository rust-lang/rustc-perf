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
pub struct ALARMOUTTYPE_R(crate::FieldReader<bool, bool>);
impl ALARMOUTTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALARMOUTTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARMOUTTYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSINSEL` reader - TIMESTAMP mapping"]
pub struct TSINSEL_R(crate::FieldReader<bool, bool>);
impl TSINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSINSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1INSEL` reader - TAMPER1 mapping"]
pub struct TAMP1INSEL_R(crate::FieldReader<bool, bool>);
impl TAMP1INSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1INSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub struct TAMPIE_R(crate::FieldReader<bool, bool>);
impl TAMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub struct TAMP1TRG_R(crate::FieldReader<bool, bool>);
impl TAMP1TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub struct TAMP1E_R(crate::FieldReader<bool, bool>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
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
