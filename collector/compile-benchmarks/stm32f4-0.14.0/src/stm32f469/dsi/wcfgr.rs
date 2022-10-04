#[doc = "Register `WCFGR` reader"]
pub struct R(crate::R<WCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCFGR` writer"]
pub struct W(crate::W<WCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCFGR_SPEC>;
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
impl From<crate::W<WCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSPOL` reader - VSync Polarity"]
pub struct VSPOL_R(crate::FieldReader<bool, bool>);
impl VSPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSPOL` writer - VSync Polarity"]
pub struct VSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `AR` reader - Automatic Refresh"]
pub struct AR_R(crate::FieldReader<bool, bool>);
impl AR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR` writer - Automatic Refresh"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
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
#[doc = "Field `TEPOL` reader - TE Polarity"]
pub struct TEPOL_R(crate::FieldReader<bool, bool>);
impl TEPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEPOL` writer - TE Polarity"]
pub struct TEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEPOL_W<'a> {
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
#[doc = "Field `TESRC` reader - TE Source"]
pub struct TESRC_R(crate::FieldReader<bool, bool>);
impl TESRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESRC` writer - TE Source"]
pub struct TESRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TESRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `COLMUX` reader - Color Multiplexing"]
pub struct COLMUX_R(crate::FieldReader<u8, u8>);
impl COLMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        COLMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLMUX` writer - Color Multiplexing"]
pub struct COLMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> COLMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `DSIM` reader - DSI Mode"]
pub struct DSIM_R(crate::FieldReader<bool, bool>);
impl DSIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIM` writer - DSI Mode"]
pub struct DSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIM_W<'a> {
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
    #[doc = "Bit 7 - VSync Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Automatic Refresh"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TE Polarity"]
    #[inline(always)]
    pub fn tepol(&self) -> TEPOL_R {
        TEPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TE Source"]
    #[inline(always)]
    pub fn tesrc(&self) -> TESRC_R {
        TESRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Color Multiplexing"]
    #[inline(always)]
    pub fn colmux(&self) -> COLMUX_R {
        COLMUX_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - DSI Mode"]
    #[inline(always)]
    pub fn dsim(&self) -> DSIM_R {
        DSIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - VSync Polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W {
        VSPOL_W { w: self }
    }
    #[doc = "Bit 6 - Automatic Refresh"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
    #[doc = "Bit 5 - TE Polarity"]
    #[inline(always)]
    pub fn tepol(&mut self) -> TEPOL_W {
        TEPOL_W { w: self }
    }
    #[doc = "Bit 4 - TE Source"]
    #[inline(always)]
    pub fn tesrc(&mut self) -> TESRC_W {
        TESRC_W { w: self }
    }
    #[doc = "Bits 1:3 - Color Multiplexing"]
    #[inline(always)]
    pub fn colmux(&mut self) -> COLMUX_W {
        COLMUX_W { w: self }
    }
    #[doc = "Bit 0 - DSI Mode"]
    #[inline(always)]
    pub fn dsim(&mut self) -> DSIM_W {
        DSIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcfgr](index.html) module"]
pub struct WCFGR_SPEC;
impl crate::RegisterSpec for WCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wcfgr::R](R) reader structure"]
impl crate::Readable for WCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcfgr::W](W) writer structure"]
impl crate::Writable for WCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WCFGR to value 0"]
impl crate::Resettable for WCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
