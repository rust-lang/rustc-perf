#[doc = "Register `HPRT` reader"]
pub struct R(crate::R<HPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPRT` writer"]
pub struct W(crate::W<HPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPRT_SPEC>;
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
impl From<crate::W<HPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSTS` reader - Port connect status"]
pub type PCSTS_R = crate::BitReader<bool>;
#[doc = "Field `PCDET` reader - Port connect detected"]
pub type PCDET_R = crate::BitReader<bool>;
#[doc = "Field `PCDET` writer - Port connect detected"]
pub type PCDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PENA` reader - Port enable"]
pub type PENA_R = crate::BitReader<bool>;
#[doc = "Field `PENA` writer - Port enable"]
pub type PENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PENCHNG` reader - Port enable/disable change"]
pub type PENCHNG_R = crate::BitReader<bool>;
#[doc = "Field `PENCHNG` writer - Port enable/disable change"]
pub type PENCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `POCA` reader - Port overcurrent active"]
pub type POCA_R = crate::BitReader<bool>;
#[doc = "Field `POCCHNG` reader - Port overcurrent change"]
pub type POCCHNG_R = crate::BitReader<bool>;
#[doc = "Field `POCCHNG` writer - Port overcurrent change"]
pub type POCCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PRES` reader - Port resume"]
pub type PRES_R = crate::BitReader<bool>;
#[doc = "Field `PRES` writer - Port resume"]
pub type PRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PSUSP` reader - Port suspend"]
pub type PSUSP_R = crate::BitReader<bool>;
#[doc = "Field `PSUSP` writer - Port suspend"]
pub type PSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PRST` reader - Port reset"]
pub type PRST_R = crate::BitReader<bool>;
#[doc = "Field `PRST` writer - Port reset"]
pub type PRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PLSTS` reader - Port line status"]
pub type PLSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPWR` reader - Port power"]
pub type PPWR_R = crate::BitReader<bool>;
#[doc = "Field `PPWR` writer - Port power"]
pub type PPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HPRT_SPEC, bool, O>;
#[doc = "Field `PTCTL` reader - Port test control"]
pub type PTCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTCTL` writer - Port test control"]
pub type PTCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPRT_SPEC, u8, u8, 4, O>;
#[doc = "Field `PSPD` reader - Port speed"]
pub type PSPD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port overcurrent active"]
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcdet(&mut self) -> PCDET_W<1> {
        PCDET_W::new(self)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pena(&mut self) -> PENA_W<2> {
        PENA_W::new(self)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(&mut self) -> PENCHNG_W<3> {
        PENCHNG_W::new(self)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(&mut self) -> POCCHNG_W<5> {
        POCCHNG_W::new(self)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<6> {
        PRES_W::new(self)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W<7> {
        PSUSP_W::new(self)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W<8> {
        PRST_W::new(self)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn ppwr(&mut self) -> PPWR_W<12> {
        PPWR_W::new(self)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn ptctl(&mut self) -> PTCTL_W<13> {
        PTCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host port control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprt](index.html) module"]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hprt::R](R) reader structure"]
impl crate::Readable for HPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hprt::W](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
