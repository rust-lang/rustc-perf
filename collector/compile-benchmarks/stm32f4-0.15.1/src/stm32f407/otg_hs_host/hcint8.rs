#[doc = "Register `HCINT8` reader"]
pub struct R(crate::R<HCINT8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINT8` writer"]
pub struct W(crate::W<HCINT8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT8_SPEC>;
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
impl From<crate::W<HCINT8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - Transfer completed"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - Transfer completed"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `CHH` reader - Channel halted"]
pub type CHH_R = crate::BitReader<bool>;
#[doc = "Field `CHH` writer - Channel halted"]
pub type CHH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `AHBERR` reader - AHB error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHB error"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `NYET` reader - Response received interrupt"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - Response received interrupt"]
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `TXERR` reader - Transaction error"]
pub type TXERR_R = crate::BitReader<bool>;
#[doc = "Field `TXERR` writer - Transaction error"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `BBERR` reader - Babble error"]
pub type BBERR_R = crate::BitReader<bool>;
#[doc = "Field `BBERR` writer - Babble error"]
pub type BBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `FRMOR` reader - Frame overrun"]
pub type FRMOR_R = crate::BitReader<bool>;
#[doc = "Field `FRMOR` writer - Frame overrun"]
pub type FRMOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
#[doc = "Field `DTERR` reader - Data toggle error"]
pub type DTERR_R = crate::BitReader<bool>;
#[doc = "Field `DTERR` writer - Data toggle error"]
pub type DTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINT8_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response received interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chh(&mut self) -> CHH_W<1> {
        CHH_W::new(self)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<3> {
        STALL_W::new(self)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<4> {
        NAK_W::new(self)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<5> {
        ACK_W::new(self)
    }
    #[doc = "Bit 6 - Response received interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<6> {
        NYET_W::new(self)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W<7> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bberr(&mut self) -> BBERR_W<8> {
        BBERR_W::new(self)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmor(&mut self) -> FRMOR_W<9> {
        FRMOR_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dterr(&mut self) -> DTERR_W<10> {
        DTERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-8 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint8](index.html) module"]
pub struct HCINT8_SPEC;
impl crate::RegisterSpec for HCINT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcint8::R](R) reader structure"]
impl crate::Readable for HCINT8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcint8::W](W) writer structure"]
impl crate::Writable for HCINT8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINT8 to value 0"]
impl crate::Resettable for HCINT8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
