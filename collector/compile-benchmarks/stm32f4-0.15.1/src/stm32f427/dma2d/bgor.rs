#[doc = "Register `BGOR` reader"]
pub struct R(crate::R<BGOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGOR` writer"]
pub struct W(crate::W<BGOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGOR_SPEC>;
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
impl From<crate::W<BGOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub type LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LO` writer - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGOR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<0> {
        LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D background offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgor](index.html) module"]
pub struct BGOR_SPEC;
impl crate::RegisterSpec for BGOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgor::R](R) reader structure"]
impl crate::Readable for BGOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgor::W](W) writer structure"]
impl crate::Writable for BGOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGOR to value 0"]
impl crate::Resettable for BGOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
