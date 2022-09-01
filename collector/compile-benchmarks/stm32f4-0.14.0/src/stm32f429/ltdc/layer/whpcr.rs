#[doc = "Register `WHPCR` reader"]
pub struct R(crate::R<WHPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WHPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WHPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WHPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WHPCR` writer"]
pub struct W(crate::W<WHPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WHPCR_SPEC>;
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
impl From<crate::W<WHPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WHPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WHSPPOS` reader - Window Horizontal Stop Position"]
pub struct WHSPPOS_R(crate::FieldReader<u16, u16>);
impl WHSPPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        WHSPPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WHSPPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WHSPPOS` writer - Window Horizontal Stop Position"]
pub struct WHSPPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WHSPPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `WHSTPOS` reader - Window Horizontal Start Position"]
pub struct WHSTPOS_R(crate::FieldReader<u16, u16>);
impl WHSTPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        WHSTPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WHSTPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WHSTPOS` writer - Window Horizontal Start Position"]
pub struct WHSTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WHSTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&mut self) -> WHSPPOS_W {
        WHSPPOS_W { w: self }
    }
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&mut self) -> WHSTPOS_W {
        WHSTPOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Window Horizontal Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [whpcr](index.html) module"]
pub struct WHPCR_SPEC;
impl crate::RegisterSpec for WHPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [whpcr::R](R) reader structure"]
impl crate::Readable for WHPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [whpcr::W](W) writer structure"]
impl crate::Writable for WHPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WHPCR to value 0"]
impl crate::Resettable for WHPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
