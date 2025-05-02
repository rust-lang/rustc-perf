#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR` writer"]
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UAN1` reader - ULPS Active Not lane 1"]
pub type UAN1_R = crate::BitReader<bool>;
#[doc = "Field `UAN1` writer - ULPS Active Not lane 1"]
pub type UAN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `PSS1` reader - PHY Stop State lane 1"]
pub type PSS1_R = crate::BitReader<bool>;
#[doc = "Field `PSS1` writer - PHY Stop State lane 1"]
pub type PSS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `RUE0` reader - RX ULPS Escape lane 0"]
pub type RUE0_R = crate::BitReader<bool>;
#[doc = "Field `RUE0` writer - RX ULPS Escape lane 0"]
pub type RUE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `UAN0` reader - ULPS Active Not lane 1"]
pub type UAN0_R = crate::BitReader<bool>;
#[doc = "Field `UAN0` writer - ULPS Active Not lane 1"]
pub type UAN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `PSS0` reader - PHY Stop State lane 0"]
pub type PSS0_R = crate::BitReader<bool>;
#[doc = "Field `PSS0` writer - PHY Stop State lane 0"]
pub type PSS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `UANC` reader - ULPS Active Not Clock lane"]
pub type UANC_R = crate::BitReader<bool>;
#[doc = "Field `UANC` writer - ULPS Active Not Clock lane"]
pub type UANC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `PSSC` reader - PHY Stop State Clock lane"]
pub type PSSC_R = crate::BitReader<bool>;
#[doc = "Field `PSSC` writer - PHY Stop State Clock lane"]
pub type PSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `PD` reader - PHY Direction"]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - PHY Direction"]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan1(&mut self) -> UAN1_W<8> {
        UAN1_W::new(self)
    }
    #[doc = "Bit 7 - PHY Stop State lane 1"]
    #[inline(always)]
    pub fn pss1(&mut self) -> PSS1_W<7> {
        PSS1_W::new(self)
    }
    #[doc = "Bit 6 - RX ULPS Escape lane 0"]
    #[inline(always)]
    pub fn rue0(&mut self) -> RUE0_W<6> {
        RUE0_W::new(self)
    }
    #[doc = "Bit 5 - ULPS Active Not lane 1"]
    #[inline(always)]
    pub fn uan0(&mut self) -> UAN0_W<5> {
        UAN0_W::new(self)
    }
    #[doc = "Bit 4 - PHY Stop State lane 0"]
    #[inline(always)]
    pub fn pss0(&mut self) -> PSS0_W<4> {
        PSS0_W::new(self)
    }
    #[doc = "Bit 3 - ULPS Active Not Clock lane"]
    #[inline(always)]
    pub fn uanc(&mut self) -> UANC_W<3> {
        UANC_W::new(self)
    }
    #[doc = "Bit 2 - PHY Stop State Clock lane"]
    #[inline(always)]
    pub fn pssc(&mut self) -> PSSC_W<2> {
        PSSC_W::new(self)
    }
    #[doc = "Bit 1 - PHY Direction"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<1> {
        PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr::W](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSR to value 0x1528"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1528
    }
}
