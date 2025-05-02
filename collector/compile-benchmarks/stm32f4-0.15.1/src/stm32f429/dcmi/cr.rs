#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - DCMI enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - DCMI enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FCRC` reader - Frame capture rate control"]
pub type FCRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCRC` writer - Frame capture rate control"]
pub type FCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VSPOL` reader - Vertical synchronization polarity"]
pub type VSPOL_R = crate::BitReader<bool>;
#[doc = "Field `VSPOL` writer - Vertical synchronization polarity"]
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSPOL` reader - Horizontal synchronization polarity"]
pub type HSPOL_R = crate::BitReader<bool>;
#[doc = "Field `HSPOL` writer - Horizontal synchronization polarity"]
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PCKPOL` reader - Pixel clock polarity"]
pub type PCKPOL_R = crate::BitReader<bool>;
#[doc = "Field `PCKPOL` writer - Pixel clock polarity"]
pub type PCKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ESS` reader - Embedded synchronization select"]
pub type ESS_R = crate::BitReader<bool>;
#[doc = "Field `ESS` writer - Embedded synchronization select"]
pub type ESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `JPEG` reader - JPEG format"]
pub type JPEG_R = crate::BitReader<bool>;
#[doc = "Field `JPEG` writer - JPEG format"]
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CROP` reader - Crop feature"]
pub type CROP_R = crate::BitReader<bool>;
#[doc = "Field `CROP` writer - Crop feature"]
pub type CROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CM` reader - Capture mode"]
pub type CM_R = crate::BitReader<bool>;
#[doc = "Field `CM` writer - Capture mode"]
pub type CM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CAPTURE` reader - Capture enable"]
pub type CAPTURE_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE` writer - Capture enable"]
pub type CAPTURE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 14 - DCMI enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Frame capture rate control"]
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Embedded synchronization select"]
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Capture enable"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - DCMI enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    #[doc = "Bits 8:9 - Frame capture rate control"]
    #[inline(always)]
    pub fn fcrc(&mut self) -> FCRC_W<8> {
        FCRC_W::new(self)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<7> {
        VSPOL_W::new(self)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<6> {
        HSPOL_W::new(self)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    pub fn pckpol(&mut self) -> PCKPOL_W<5> {
        PCKPOL_W::new(self)
    }
    #[doc = "Bit 4 - Embedded synchronization select"]
    #[inline(always)]
    pub fn ess(&mut self) -> ESS_W<4> {
        ESS_W::new(self)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&mut self) -> JPEG_W<3> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 2 - Crop feature"]
    #[inline(always)]
    pub fn crop(&mut self) -> CROP_W<2> {
        CROP_W::new(self)
    }
    #[doc = "Bit 1 - Capture mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W<1> {
        CM_W::new(self)
    }
    #[doc = "Bit 0 - Capture enable"]
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W<0> {
        CAPTURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
