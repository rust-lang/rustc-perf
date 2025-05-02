#[doc = "Register `PTPTSSR` reader"]
pub struct R(crate::R<PTPTSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSSO` reader - TSSO"]
pub type TSSO_R = crate::BitReader<bool>;
#[doc = "Field `TSTTR` reader - TSTTR"]
pub type TSTTR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TSSO"]
    #[inline(always)]
    pub fn tsso(&self) -> TSSO_R {
        TSSO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSTTR"]
    #[inline(always)]
    pub fn tsttr(&self) -> TSTTR_R {
        TSTTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Ethernet PTP time stamp status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptssr](index.html) module"]
pub struct PTPTSSR_SPEC;
impl crate::RegisterSpec for PTPTSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptssr::R](R) reader structure"]
impl crate::Readable for PTPTSSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPTSSR to value 0"]
impl crate::Resettable for PTPTSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
