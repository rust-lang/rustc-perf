#[doc = "Register `CEC_CFGR` reader"]
pub struct R(crate::R<CEC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_CFGR` writer"]
pub struct W(crate::W<CEC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CFGR_SPEC>;
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
impl From<crate::W<CEC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTN` reader - Listen mode"]
pub type LSTN_R = crate::BitReader<bool>;
#[doc = "Field `LSTN` writer - Listen mode"]
pub type LSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `OAR` reader - Own addresses configuration"]
pub type OAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OAR` writer - Own addresses configuration"]
pub type OAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_CFGR_SPEC, u16, u16, 15, O>;
#[doc = "Field `SFTOP` reader - SFT Option Bit"]
pub type SFTOP_R = crate::BitReader<bool>;
#[doc = "Field `SFTOP` writer - SFT Option Bit"]
pub type SFTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BRDNOGEN` reader - Avoid Error-Bit Generation in Broadcast"]
pub type BRDNOGEN_R = crate::BitReader<bool>;
#[doc = "Field `BRDNOGEN` writer - Avoid Error-Bit Generation in Broadcast"]
pub type BRDNOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error"]
pub type LBPEGEN_R = crate::BitReader<bool>;
#[doc = "Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error"]
pub type LBPEGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BREGEN` reader - Generate Error-Bit on Bit Rising Error"]
pub type BREGEN_R = crate::BitReader<bool>;
#[doc = "Field `BREGEN` writer - Generate Error-Bit on Bit Rising Error"]
pub type BREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `BRESTP` reader - Rx-Stop on Bit Rising Error"]
pub type BRESTP_R = crate::BitReader<bool>;
#[doc = "Field `BRESTP` writer - Rx-Stop on Bit Rising Error"]
pub type BRESTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `RXTOL` reader - Rx-Tolerance"]
pub type RXTOL_R = crate::BitReader<bool>;
#[doc = "Field `RXTOL` writer - Rx-Tolerance"]
pub type RXTOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CFGR_SPEC, bool, O>;
#[doc = "Field `SFT` reader - Signal Free Time"]
pub type SFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFT` writer - Signal Free Time"]
pub type SFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_CFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W<31> {
        LSTN_W::new(self)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W<16> {
        OAR_W::new(self)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W<8> {
        SFTOP_W::new(self)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W<7> {
        BRDNOGEN_W::new(self)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W<6> {
        LBPEGEN_W::new(self)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W<5> {
        BREGEN_W::new(self)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W<4> {
        BRESTP_W::new(self)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W<3> {
        RXTOL_W::new(self)
    }
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W<0> {
        SFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cfgr](index.html) module"]
pub struct CEC_CFGR_SPEC;
impl crate::RegisterSpec for CEC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_cfgr::R](R) reader structure"]
impl crate::Readable for CEC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_cfgr::W](W) writer structure"]
impl crate::Writable for CEC_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_CFGR to value 0"]
impl crate::Resettable for CEC_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
