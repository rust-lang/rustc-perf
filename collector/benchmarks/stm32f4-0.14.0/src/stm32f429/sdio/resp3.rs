#[doc = "Register `RESP3` reader"]
pub struct R(crate::R<RESP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS3` reader - see Table 132."]
pub struct CARDSTATUS3_R(crate::FieldReader<u32, u32>);
impl CARDSTATUS3_R {
    pub(crate) fn new(bits: u32) -> Self {
        CARDSTATUS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDSTATUS3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - see Table 132."]
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "response 1..4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp3](index.html) module"]
pub struct RESP3_SPEC;
impl crate::RegisterSpec for RESP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp3::R](R) reader structure"]
impl crate::Readable for RESP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP3 to value 0"]
impl crate::Resettable for RESP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
