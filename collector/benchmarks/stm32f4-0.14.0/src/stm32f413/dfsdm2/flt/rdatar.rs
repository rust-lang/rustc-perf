#[doc = "Register `RDATAR` reader"]
pub struct R(crate::R<RDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Regular channel conversion data"]
pub struct RDATA_R(crate::FieldReader<u32, u32>);
impl RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPEND` reader - Regular channel pending data"]
pub struct RPEND_R(crate::FieldReader<bool, bool>);
impl RPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDATACH` reader - Regular channel most recently converted"]
pub struct RDATACH_R(crate::FieldReader<u8, u8>);
impl RDATACH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDATACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATACH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:31 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 4 - Regular channel pending data"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Regular channel most recently converted"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdatar](index.html) module"]
pub struct RDATAR_SPEC;
impl crate::RegisterSpec for RDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdatar::R](R) reader structure"]
impl crate::Readable for RDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDATAR to value 0"]
impl crate::Resettable for RDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
