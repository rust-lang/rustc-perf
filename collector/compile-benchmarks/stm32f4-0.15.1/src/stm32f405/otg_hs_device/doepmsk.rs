#[doc = "Register `DOEPMSK` reader"]
pub struct R(crate::R<DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPMSK` writer"]
pub struct W(crate::W<DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPMSK_SPEC>;
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
impl From<crate::W<DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader<bool>;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `STUPM` reader - SETUP phase done mask"]
pub type STUPM_R = crate::BitReader<bool>;
#[doc = "Field `STUPM` writer - SETUP phase done mask"]
pub type STUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OTEPDM` reader - OUT token received when endpoint disabled mask"]
pub type OTEPDM_R = crate::BitReader<bool>;
#[doc = "Field `OTEPDM` writer - OUT token received when endpoint disabled mask"]
pub type OTEPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received mask"]
pub type B2BSTUP_R = crate::BitReader<bool>;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received mask"]
pub type B2BSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OPEM` reader - OUT packet error mask"]
pub type OPEM_R = crate::BitReader<bool>;
#[doc = "Field `OPEM` writer - OUT packet error mask"]
pub type OPEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `BOIM` reader - BNA interrupt mask"]
pub type BOIM_R = crate::BitReader<bool>;
#[doc = "Field `BOIM` writer - BNA interrupt mask"]
pub type BOIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn opem(&self) -> OPEM_R {
        OPEM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn boim(&self) -> BOIM_R {
        BOIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&mut self) -> STUPM_W<3> {
        STUPM_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&mut self) -> OTEPDM_W<4> {
        OTEPDM_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received mask"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<6> {
        B2BSTUP_W::new(self)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn opem(&mut self) -> OPEM_W<8> {
        OPEM_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn boim(&mut self) -> BOIM_W<9> {
        BOIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device OUT endpoint common interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](index.html) module"]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepmsk::R](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
