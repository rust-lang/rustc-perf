#[doc = "Register `IMSCR` reader"]
pub struct R(crate::R<IMSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMSCR` writer"]
pub struct W(crate::W<IMSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMSCR_SPEC>;
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
impl From<crate::W<IMSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTIM` reader - Output FIFO service interrupt mask"]
pub struct OUTIM_R(crate::FieldReader<bool, bool>);
impl OUTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTIM` writer - Output FIFO service interrupt mask"]
pub struct OUTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `INIM` reader - Input FIFO service interrupt mask"]
pub struct INIM_R(crate::FieldReader<bool, bool>);
impl INIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIM` writer - Input FIFO service interrupt mask"]
pub struct INIM_W<'a> {
    w: &'a mut W,
}
impl<'a> INIM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Output FIFO service interrupt mask"]
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Input FIFO service interrupt mask"]
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Output FIFO service interrupt mask"]
    #[inline(always)]
    pub fn outim(&mut self) -> OUTIM_W {
        OUTIM_W { w: self }
    }
    #[doc = "Bit 0 - Input FIFO service interrupt mask"]
    #[inline(always)]
    pub fn inim(&mut self) -> INIM_W {
        INIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt mask set/clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imscr](index.html) module"]
pub struct IMSCR_SPEC;
impl crate::RegisterSpec for IMSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imscr::R](R) reader structure"]
impl crate::Readable for IMSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imscr::W](W) writer structure"]
impl crate::Writable for IMSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMSCR to value 0"]
impl crate::Resettable for IMSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
