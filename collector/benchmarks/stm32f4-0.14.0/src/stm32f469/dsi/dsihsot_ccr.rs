#[doc = "Register `DSIHSOT_CCR` reader"]
pub struct R(crate::R<DSIHSOT_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSIHSOT_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSIHSOT_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSIHSOT_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSIHSOT_CCR` writer"]
pub struct W(crate::W<DSIHSOT_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSIHSOT_CCR_SPEC>;
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
impl From<crate::W<DSIHSOT_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSIHSOT_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCKDIV` reader - TOCKDIV"]
pub struct TOCKDIV_R(crate::FieldReader<u8, u8>);
impl TOCKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOCKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOCKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOCKDIV` writer - TOCKDIV"]
pub struct TOCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TXECKDIV` reader - TXECKDIV"]
pub struct TXECKDIV_R(crate::FieldReader<u8, u8>);
impl TXECKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXECKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXECKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXECKDIV` writer - TXECKDIV"]
pub struct TXECKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXECKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&self) -> TOCKDIV_R {
        TOCKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&self) -> TXECKDIV_R {
        TXECKDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&mut self) -> TOCKDIV_W {
        TOCKDIV_W { w: self }
    }
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&mut self) -> TXECKDIV_W {
        TXECKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI HOST Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsihsot_ccr](index.html) module"]
pub struct DSIHSOT_CCR_SPEC;
impl crate::RegisterSpec for DSIHSOT_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsihsot_ccr::R](R) reader structure"]
impl crate::Readable for DSIHSOT_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsihsot_ccr::W](W) writer structure"]
impl crate::Writable for DSIHSOT_CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSIHSOT_CCR to value 0x3133_302a"]
impl crate::Resettable for DSIHSOT_CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3133_302a
    }
}
