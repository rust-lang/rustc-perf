#[doc = "Register `DOEPCTL2` reader"]
pub struct R(crate::R<DOEPCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL2` writer"]
pub struct W(crate::W<DOEPCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL2_SPEC>;
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
impl From<crate::W<DOEPCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPENA` reader - EPENA"]
pub type EPENA_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - EPENA"]
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - EPDIS"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - EPDIS"]
pub type EPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `SODDFRM` writer - SODDFRM"]
pub type SODDFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `SD0PID_SEVNFRM` writer - SD0PID/SEVNFRM"]
pub type SD0PID_SEVNFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - SNAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `CNAK` writer - CNAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `Stall` reader - Stall"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `Stall` writer - Stall"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `SNPM` reader - SNPM"]
pub type SNPM_R = crate::BitReader<bool>;
#[doc = "Field `SNPM` writer - SNPM"]
pub type SNPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `EPTYP` reader - EPTYP"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYP` writer - EPTYP"]
pub type EPTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPCTL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `NAKSTS` reader - NAKSTS"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EONUM_DPID` reader - EONUM/DPID"]
pub type EONUM_DPID_R = crate::BitReader<bool>;
#[doc = "Field `USBAEP` reader - USBAEP"]
pub type USBAEP_R = crate::BitReader<bool>;
#[doc = "Field `USBAEP` writer - USBAEP"]
pub type USBAEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL2_SPEC, bool, O>;
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPSIZ` writer - MPSIZ"]
pub type MPSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPCTL2_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 17 - NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - EONUM/DPID"]
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EONUM_DPID_R {
        EONUM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<30> {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 29 - SODDFRM"]
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W<29> {
        SODDFRM_W::new(self)
    }
    #[doc = "Bit 28 - SD0PID/SEVNFRM"]
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<28> {
        SD0PID_SEVNFRM_W::new(self)
    }
    #[doc = "Bit 27 - SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 26 - CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 21 - Stall"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<20> {
        SNPM_W::new(self)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<18> {
        EPTYP_W::new(self)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<15> {
        USBAEP_W::new(self)
    }
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<0> {
        MPSIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl2](index.html) module"]
pub struct DOEPCTL2_SPEC;
impl crate::RegisterSpec for DOEPCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl2::R](R) reader structure"]
impl crate::Readable for DOEPCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl2::W](W) writer structure"]
impl crate::Writable for DOEPCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPCTL2 to value 0"]
impl crate::Resettable for DOEPCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
