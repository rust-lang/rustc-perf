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
#[doc = "Field `CEIF` reader - Configuration error interrupt flag"]
pub struct CEIF_R(crate::FieldReader<bool, bool>);
impl CEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF` reader - CLUT transfer complete interrupt flag"]
pub struct CTCIF_R(crate::FieldReader<bool, bool>);
impl CTCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAEIF` reader - CLUT access error interrupt flag"]
pub struct CAEIF_R(crate::FieldReader<bool, bool>);
impl CAEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIF` reader - Transfer watermark interrupt flag"]
pub struct TWIF_R(crate::FieldReader<bool, bool>);
impl TWIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF` reader - Transfer complete interrupt flag"]
pub struct TCIF_R(crate::FieldReader<bool, bool>);
impl TCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF` reader - Transfer error interrupt flag"]
pub struct TEIF_R(crate::FieldReader<bool, bool>);
impl TEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - Configuration error interrupt flag"]
    #[inline(always)]
    pub fn ceif(&self) -> CEIF_R {
        CEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caeif(&self) -> CAEIF_R {
        CAEIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn twif(&self) -> TWIF_R {
        TWIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt flag"]
    #[inline(always)]
    pub fn tcif(&self) -> TCIF_R {
        TCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transfer error interrupt flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
