#[doc = "Register `MACHTLR` reader"]
pub struct R(crate::R<MACHTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACHTLR` writer"]
pub struct W(crate::W<MACHTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACHTLR_SPEC>;
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
impl From<crate::W<MACHTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACHTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTL` reader - Lower 32 bits of hash table"]
pub type HTL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTL` writer - Lower 32 bits of hash table"]
pub type HTL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACHTLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Lower 32 bits of hash table"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32 bits of hash table"]
    #[inline(always)]
    pub fn htl(&mut self) -> HTL_W<0> {
        HTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Ethernet MAC hash table low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machtlr](index.html) module"]
pub struct MACHTLR_SPEC;
impl crate::RegisterSpec for MACHTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [machtlr::R](R) reader structure"]
impl crate::Readable for MACHTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [machtlr::W](W) writer structure"]
impl crate::Writable for MACHTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACHTLR to value 0"]
impl crate::Resettable for MACHTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
