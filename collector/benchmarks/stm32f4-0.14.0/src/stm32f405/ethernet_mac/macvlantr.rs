#[doc = "Register `MACVLANTR` reader"]
pub struct R(crate::R<MACVLANTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVLANTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVLANTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVLANTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACVLANTR` writer"]
pub struct W(crate::W<MACVLANTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVLANTR_SPEC>;
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
impl From<crate::W<MACVLANTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVLANTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLANTI` reader - VLANTI"]
pub struct VLANTI_R(crate::FieldReader<u16, u16>);
impl VLANTI_R {
    pub(crate) fn new(bits: u16) -> Self {
        VLANTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLANTI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLANTI` writer - VLANTI"]
pub struct VLANTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `VLANTC` reader - VLANTC"]
pub struct VLANTC_R(crate::FieldReader<bool, bool>);
impl VLANTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLANTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLANTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLANTC` writer - VLANTC"]
pub struct VLANTC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLANTI"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - VLANTC"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLANTI"]
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W {
        VLANTI_W { w: self }
    }
    #[doc = "Bit 16 - VLANTC"]
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W {
        VLANTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvlantr](index.html) module"]
pub struct MACVLANTR_SPEC;
impl crate::RegisterSpec for MACVLANTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macvlantr::R](R) reader structure"]
impl crate::Readable for MACVLANTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macvlantr::W](W) writer structure"]
impl crate::Writable for MACVLANTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MACVLANTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
