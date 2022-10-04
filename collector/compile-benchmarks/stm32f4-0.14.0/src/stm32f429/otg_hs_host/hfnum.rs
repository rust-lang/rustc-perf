#[doc = "Register `HFNUM` reader"]
pub struct R(crate::R<HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRNUM` reader - Frame number"]
pub struct FRNUM_R(crate::FieldReader<u16, u16>);
impl FRNUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FRNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub struct FTREM_R(crate::FieldReader<u16, u16>);
impl FTREM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FTREM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTREM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_HS host frame number/frame time remaining register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfnum](index.html) module"]
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfnum::R](R) reader structure"]
impl crate::Readable for HFNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFNUM to value 0x3fff"]
impl crate::Resettable for HFNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff
    }
}
