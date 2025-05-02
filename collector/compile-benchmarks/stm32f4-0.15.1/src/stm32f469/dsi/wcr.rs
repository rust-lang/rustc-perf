#[doc = "Register `WCR` reader"]
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIEN` reader - DSI Enable"]
pub type DSIEN_R = crate::BitReader<bool>;
#[doc = "Field `DSIEN` writer - DSI Enable"]
pub type DSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
#[doc = "Field `LTDCEN` reader - LTDC Enable"]
pub type LTDCEN_R = crate::BitReader<bool>;
#[doc = "Field `LTDCEN` writer - LTDC Enable"]
pub type LTDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
#[doc = "Field `SHTDN` reader - Shutdown"]
pub type SHTDN_R = crate::BitReader<bool>;
#[doc = "Field `SHTDN` writer - Shutdown"]
pub type SHTDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
#[doc = "Field `COLM` reader - Color Mode"]
pub type COLM_R = crate::BitReader<bool>;
#[doc = "Field `COLM` writer - Color Mode"]
pub type COLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - DSI Enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - LTDC Enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Color Mode"]
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DSI Enable"]
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W<3> {
        DSIEN_W::new(self)
    }
    #[doc = "Bit 2 - LTDC Enable"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<2> {
        LTDCEN_W::new(self)
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&mut self) -> SHTDN_W<1> {
        SHTDN_W::new(self)
    }
    #[doc = "Bit 0 - Color Mode"]
    #[inline(always)]
    pub fn colm(&mut self) -> COLM_W<0> {
        COLM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wcr::R](R) reader structure"]
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
