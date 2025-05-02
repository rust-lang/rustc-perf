#[doc = "Register `LCOLCR` reader"]
pub struct R(crate::R<LCOLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCOLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCOLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCOLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCOLCR` writer"]
pub struct W(crate::W<LCOLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCOLCR_SPEC>;
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
impl From<crate::W<LCOLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCOLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPE` reader - Loosely Packet Enable"]
pub type LPE_R = crate::BitReader<bool>;
#[doc = "Field `LPE` writer - Loosely Packet Enable"]
pub type LPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCOLCR_SPEC, bool, O>;
#[doc = "Field `COLC` reader - Color Coding"]
pub type COLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COLC` writer - Color Coding"]
pub type COLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCOLCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 8 - Loosely Packet Enable"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Color Coding"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Loosely Packet Enable"]
    #[inline(always)]
    pub fn lpe(&mut self) -> LPE_W<8> {
        LPE_W::new(self)
    }
    #[doc = "Bits 0:3 - Color Coding"]
    #[inline(always)]
    pub fn colc(&mut self) -> COLC_W<0> {
        COLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC Color Coding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcolcr](index.html) module"]
pub struct LCOLCR_SPEC;
impl crate::RegisterSpec for LCOLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcolcr::R](R) reader structure"]
impl crate::Readable for LCOLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcolcr::W](W) writer structure"]
impl crate::Writable for LCOLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCOLCR to value 0"]
impl crate::Resettable for LCOLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
