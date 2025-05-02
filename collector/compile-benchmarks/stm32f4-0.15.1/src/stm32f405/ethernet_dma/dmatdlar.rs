#[doc = "Register `DMATDLAR` reader"]
pub struct R(crate::R<DMATDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATDLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATDLAR` writer"]
pub struct W(crate::W<DMATDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATDLAR_SPEC>;
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
impl From<crate::W<DMATDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATDLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STL` reader - STL"]
pub type STL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STL` writer - STL"]
pub type STL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMATDLAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - STL"]
    #[inline(always)]
    pub fn stl(&self) -> STL_R {
        STL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - STL"]
    #[inline(always)]
    pub fn stl(&mut self) -> STL_W<0> {
        STL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA transmit descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatdlar](index.html) module"]
pub struct DMATDLAR_SPEC;
impl crate::RegisterSpec for DMATDLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatdlar::R](R) reader structure"]
impl crate::Readable for DMATDLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatdlar::W](W) writer structure"]
impl crate::Writable for DMATDLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATDLAR to value 0"]
impl crate::Resettable for DMATDLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
