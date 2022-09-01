#[doc = "Register `MACSR` reader"]
pub struct R(crate::R<MACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSR` writer"]
pub struct W(crate::W<MACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMTS` reader - PMTS"]
pub struct PMTS_R(crate::FieldReader<bool, bool>);
impl PMTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCS` reader - MMCS"]
pub struct MMCS_R(crate::FieldReader<bool, bool>);
impl MMCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCRS` reader - MMCRS"]
pub struct MMCRS_R(crate::FieldReader<bool, bool>);
impl MMCRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCTS` reader - MMCTS"]
pub struct MMCTS_R(crate::FieldReader<bool, bool>);
impl MMCTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTS` reader - TSTS"]
pub struct TSTS_R(crate::FieldReader<bool, bool>);
impl TSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTS` writer - TSTS"]
pub struct TSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - PMTS"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMCS"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMCRS"]
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMCTS"]
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TSTS"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - TSTS"]
    #[inline(always)]
    pub fn tsts(&mut self) -> TSTS_W {
        TSTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macsr](index.html) module"]
pub struct MACSR_SPEC;
impl crate::RegisterSpec for MACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macsr::R](R) reader structure"]
impl crate::Readable for MACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macsr::W](W) writer structure"]
impl crate::Writable for MACSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACSR to value 0"]
impl crate::Resettable for MACSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
