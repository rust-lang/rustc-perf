#[doc = "Register `MACA0HR` reader"]
pub struct R(crate::R<MACA0HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA0HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA0HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA0HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA0HR` writer"]
pub struct W(crate::W<MACA0HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA0HR_SPEC>;
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
impl From<crate::W<MACA0HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA0HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA0H` reader - MAC address0 high"]
pub type MACA0H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MACA0H` writer - MAC address0 high"]
pub type MACA0H_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA0HR_SPEC, u16, u16, 16, O>;
#[doc = "Field `MO` reader - Always 1"]
pub type MO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn maca0h(&self) -> MACA0H_R {
        MACA0H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn maca0h(&mut self) -> MACA0H_W<0> {
        MACA0H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 0 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0hr](index.html) module"]
pub struct MACA0HR_SPEC;
impl crate::RegisterSpec for MACA0HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca0hr::R](R) reader structure"]
impl crate::Readable for MACA0HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca0hr::W](W) writer structure"]
impl crate::Writable for MACA0HR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA0HR to value 0x0010_ffff"]
impl crate::Resettable for MACA0HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_ffff
    }
}
