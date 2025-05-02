#[doc = "Register `MACMIIDR` reader"]
pub struct R(crate::R<MACMIIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMIIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMIIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMIIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMIIDR` writer"]
pub struct W(crate::W<MACMIIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMIIDR_SPEC>;
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
impl From<crate::W<MACMIIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMIIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - MII data read from/written to the PHY"]
pub type MD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MD` writer - MII data read from/written to the PHY"]
pub type MD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACMIIDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MII data read from/written to the PHY"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data read from/written to the PHY"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC MII data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmiidr](index.html) module"]
pub struct MACMIIDR_SPEC;
impl crate::RegisterSpec for MACMIIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmiidr::R](R) reader structure"]
impl crate::Readable for MACMIIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmiidr::W](W) writer structure"]
impl crate::Writable for MACMIIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACMIIDR to value 0"]
impl crate::Resettable for MACMIIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
