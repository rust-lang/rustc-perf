#[doc = "Register `TDTR` reader"]
pub struct R(crate::R<TDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDTR` writer"]
pub struct W(crate::W<TDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDTR_SPEC>;
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
impl From<crate::W<TDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - TIME"]
pub struct TIME_R(crate::FieldReader<u16, u16>);
impl TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME` writer - TIME"]
pub struct TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `TGT` reader - TGT"]
pub struct TGT_R(crate::FieldReader<bool, bool>);
impl TGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGT` writer - TGT"]
pub struct TGT_W<'a> {
    w: &'a mut W,
}
impl<'a> TGT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DLC` reader - DLC"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` writer - DLC"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W {
        TIME_W { w: self }
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&mut self) -> TGT_W {
        TGT_W { w: self }
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mailbox data length control and time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdtr](index.html) module"]
pub struct TDTR_SPEC;
impl crate::RegisterSpec for TDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdtr::R](R) reader structure"]
impl crate::Readable for TDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdtr::W](W) writer structure"]
impl crate::Writable for TDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TDTR to value 0"]
impl crate::Resettable for TDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
