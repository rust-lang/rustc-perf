#[doc = "Register `STR` reader"]
pub struct R(crate::R<STR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STR` writer"]
pub struct W(crate::W<STR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STR_SPEC>;
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
impl From<crate::W<STR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCAL` writer - Digest calculation"]
pub type DCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STR_SPEC, bool, O>;
#[doc = "Field `NBLW` reader - Number of valid bits in the last word of the message"]
pub type NBLW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBLW` writer - Number of valid bits in the last word of the message"]
pub type NBLW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Digest calculation"]
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W<8> {
        DCAL_W::new(self)
    }
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W<0> {
        NBLW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](index.html) module"]
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [str::R](R) reader structure"]
impl crate::Readable for STR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [str::W](W) writer structure"]
impl crate::Writable for STR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for STR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
