#[doc = "Register `CWSIZE` reader"]
pub struct R(crate::R<CWSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWSIZE` writer"]
pub struct W(crate::W<CWSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWSIZE_SPEC>;
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
impl From<crate::W<CWSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLINE` reader - Vertical line count"]
pub type VLINE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLINE` writer - Vertical line count"]
pub type VLINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWSIZE_SPEC, u16, u16, 14, O>;
#[doc = "Field `CAPCNT` reader - Capture count"]
pub type CAPCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPCNT` writer - Capture count"]
pub type CAPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWSIZE_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&mut self) -> VLINE_W<16> {
        VLINE_W::new(self)
    }
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&mut self) -> CAPCNT_W<0> {
        CAPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "crop window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwsize](index.html) module"]
pub struct CWSIZE_SPEC;
impl crate::RegisterSpec for CWSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwsize::R](R) reader structure"]
impl crate::Readable for CWSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwsize::W](W) writer structure"]
impl crate::Writable for CWSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWSIZE to value 0"]
impl crate::Resettable for CWSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
