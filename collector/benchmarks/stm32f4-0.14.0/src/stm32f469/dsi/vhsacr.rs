#[doc = "Register `VHSACR` reader"]
pub struct R(crate::R<VHSACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHSACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHSACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHSACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VHSACR` writer"]
pub struct W(crate::W<VHSACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VHSACR_SPEC>;
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
impl From<crate::W<VHSACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VHSACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSA` reader - Horizontal Synchronism Active duration"]
pub struct HSA_R(crate::FieldReader<u16, u16>);
impl HSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSA` writer - Horizontal Synchronism Active duration"]
pub struct HSA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&mut self) -> HSA_W {
        HSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video HSA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhsacr](index.html) module"]
pub struct VHSACR_SPEC;
impl crate::RegisterSpec for VHSACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vhsacr::R](R) reader structure"]
impl crate::Readable for VHSACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vhsacr::W](W) writer structure"]
impl crate::Writable for VHSACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VHSACR to value 0"]
impl crate::Resettable for VHSACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
