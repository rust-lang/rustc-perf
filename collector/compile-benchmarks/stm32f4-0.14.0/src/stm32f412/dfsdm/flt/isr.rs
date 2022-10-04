#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCDF` reader - short-circuit detector flag"]
pub struct SCDF_R(crate::FieldReader<u8, u8>);
impl SCDF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKABF` reader - Clock absence flag"]
pub struct CKABF_R(crate::FieldReader<u8, u8>);
impl CKABF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKABF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKABF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCIP` reader - Regular conversion in progress status"]
pub struct RCIP_R(crate::FieldReader<bool, bool>);
impl RCIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JCIP` reader - Injected conversion in progress status"]
pub struct JCIP_R(crate::FieldReader<bool, bool>);
impl JCIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        JCIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JCIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDF` reader - Analog watchdog"]
pub struct AWDF_R(crate::FieldReader<bool, bool>);
impl AWDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag"]
pub struct ROVRF_R(crate::FieldReader<bool, bool>);
impl ROVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag"]
pub struct JOVRF_R(crate::FieldReader<bool, bool>);
impl JOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JOVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REOCF` reader - End of regular conversion flag"]
pub struct REOCF_R(crate::FieldReader<bool, bool>);
impl REOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REOCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REOCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOCF` reader - End of injected conversion flag"]
pub struct JEOCF_R(crate::FieldReader<bool, bool>);
impl JEOCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31 - short-circuit detector flag"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock absence flag"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0x00ff_0000"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0000
    }
}
