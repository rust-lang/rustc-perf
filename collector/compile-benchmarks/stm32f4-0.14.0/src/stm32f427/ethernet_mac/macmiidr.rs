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
pub struct MD_R(crate::FieldReader<u16, u16>);
impl MD_R {
    pub(crate) fn new(bits: u16) -> Self {
        MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MD` writer - MII data read from/written to the PHY"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
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
