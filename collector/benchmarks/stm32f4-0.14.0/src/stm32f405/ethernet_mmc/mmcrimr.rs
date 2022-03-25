#[doc = "Register `MMCRIMR` reader"]
pub struct R(crate::R<MMCRIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCRIMR` writer"]
pub struct W(crate::W<MMCRIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRIMR_SPEC>;
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
impl From<crate::W<MMCRIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFCEM` reader - RFCEM"]
pub struct RFCEM_R(crate::FieldReader<bool, bool>);
impl RFCEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCEM` writer - RFCEM"]
pub struct RFCEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RFAEM` reader - RFAEM"]
pub struct RFAEM_R(crate::FieldReader<bool, bool>);
impl RFAEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFAEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFAEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFAEM` writer - RFAEM"]
pub struct RFAEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFAEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RGUFM` reader - RGUFM"]
pub struct RGUFM_R(crate::FieldReader<bool, bool>);
impl RGUFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RGUFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGUFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGUFM` writer - RGUFM"]
pub struct RGUFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RGUFM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - RFCEM"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RFAEM"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RGUFM"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RFCEM"]
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W {
        RFCEM_W { w: self }
    }
    #[doc = "Bit 6 - RFAEM"]
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W {
        RFAEM_W { w: self }
    }
    #[doc = "Bit 17 - RGUFM"]
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W {
        RGUFM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC receive interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrimr](index.html) module"]
pub struct MMCRIMR_SPEC;
impl crate::RegisterSpec for MMCRIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrimr::R](R) reader structure"]
impl crate::Readable for MMCRIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcrimr::W](W) writer structure"]
impl crate::Writable for MMCRIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCRIMR to value 0"]
impl crate::Resettable for MMCRIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
