#[doc = "Register `CLUTWR` writer"]
pub struct W(crate::W<CLUTWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLUTWR_SPEC>;
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
impl From<crate::W<CLUTWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLUTWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLUTADD` writer - CLUT Address"]
pub type CLUTADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLUTWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED` writer - Red value"]
pub type RED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLUTWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `GREEN` writer - Green value"]
pub type GREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLUTWR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BLUE` writer - Blue value"]
pub type BLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLUTWR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W<24> {
        CLUTADD_W::new(self)
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 0:7 - Blue value"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx CLUT Write Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clutwr](index.html) module"]
pub struct CLUTWR_SPEC;
impl crate::RegisterSpec for CLUTWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clutwr::W](W) writer structure"]
impl crate::Writable for CLUTWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLUTWR to value 0"]
impl crate::Resettable for CLUTWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
