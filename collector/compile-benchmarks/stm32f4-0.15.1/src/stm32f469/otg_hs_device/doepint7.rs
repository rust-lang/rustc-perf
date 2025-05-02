#[doc = "Register `DOEPINT7` reader"]
pub struct R(crate::R<DOEPINT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT7` writer"]
pub struct W(crate::W<DOEPINT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT7_SPEC>;
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
impl From<crate::W<DOEPINT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT7_SPEC, bool, O>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EPDISD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT7_SPEC, bool, O>;
#[doc = "Field `STUP` reader - SETUP phase done"]
pub type STUP_R = crate::BitReader<bool>;
#[doc = "Field `STUP` writer - SETUP phase done"]
pub type STUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT7_SPEC, bool, O>;
#[doc = "Field `OTEPDIS` reader - OUT token received when endpoint disabled"]
pub type OTEPDIS_R = crate::BitReader<bool>;
#[doc = "Field `OTEPDIS` writer - OUT token received when endpoint disabled"]
pub type OTEPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT7_SPEC, bool, O>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2BSTUP_R = crate::BitReader<bool>;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2BSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT7_SPEC, bool, O>;
#[doc = "Field `NYET` reader - NYET interrupt"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - NYET interrupt"]
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT7_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<3> {
        STUP_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<4> {
        OTEPDIS_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<6> {
        B2BSTUP_W::new(self)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<14> {
        NYET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device endpoint-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint7](index.html) module"]
pub struct DOEPINT7_SPEC;
impl crate::RegisterSpec for DOEPINT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint7::R](R) reader structure"]
impl crate::Readable for DOEPINT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint7::W](W) writer structure"]
impl crate::Writable for DOEPINT7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT7 to value 0"]
impl crate::Resettable for DOEPINT7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
