#[doc = "Register `PLLSAICFGR` reader"]
pub struct R(crate::R<PLLSAICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSAICFGR` writer"]
pub struct W(crate::W<PLLSAICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAICFGR_SPEC>;
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
impl From<crate::W<PLLSAICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSAIN` reader - PLLSAI division factor for VCO"]
pub type PLLSAIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLLSAIN` writer - PLLSAI division factor for VCO"]
pub type PLLSAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAICFGR_SPEC, u16, u16, 9, O>;
#[doc = "PLLSAI division factor for 48 MHz clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSAIP_A {
    #[doc = "0: PLL*P=2"]
    Div2 = 0,
    #[doc = "1: PLL*P=4"]
    Div4 = 1,
    #[doc = "2: PLL*P=6"]
    Div6 = 2,
    #[doc = "3: PLL*P=8"]
    Div8 = 3,
}
impl From<PLLSAIP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSAIP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSAIP` reader - PLLSAI division factor for 48 MHz clock"]
pub type PLLSAIP_R = crate::FieldReader<u8, PLLSAIP_A>;
impl PLLSAIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAIP_A {
        match self.bits {
            0 => PLLSAIP_A::Div2,
            1 => PLLSAIP_A::Div4,
            2 => PLLSAIP_A::Div6,
            3 => PLLSAIP_A::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLSAIP_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLSAIP_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLSAIP_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLSAIP_A::Div8
    }
}
#[doc = "Field `PLLSAIP` writer - PLLSAI division factor for 48 MHz clock"]
pub type PLLSAIP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLLSAICFGR_SPEC, u8, PLLSAIP_A, 2, O>;
impl<'a, const O: u8> PLLSAIP_W<'a, O> {
    #[doc = "PLL*P=2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div2)
    }
    #[doc = "PLL*P=4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div4)
    }
    #[doc = "PLL*P=6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div6)
    }
    #[doc = "PLL*P=8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLSAIP_A::Div8)
    }
}
#[doc = "Field `PLLSAIQ` reader - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAIQ` writer - PLLSAI division factor for SAI1 clock"]
pub type PLLSAIQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAICFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PLLSAIR` reader - PLLSAI division factor for LCD clock"]
pub type PLLSAIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAIR` writer - PLLSAI division factor for LCD clock"]
pub type PLLSAIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAICFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - PLLSAI division factor for 48 MHz clock"]
    #[inline(always)]
    pub fn pllsaip(&self) -> PLLSAIP_R {
        PLLSAIP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - PLLSAI division factor for LCD clock"]
    #[inline(always)]
    pub fn pllsair(&self) -> PLLSAIR_R {
        PLLSAIR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&mut self) -> PLLSAIN_W<6> {
        PLLSAIN_W::new(self)
    }
    #[doc = "Bits 16:17 - PLLSAI division factor for 48 MHz clock"]
    #[inline(always)]
    pub fn pllsaip(&mut self) -> PLLSAIP_W<16> {
        PLLSAIP_W::new(self)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W<24> {
        PLLSAIQ_W::new(self)
    }
    #[doc = "Bits 28:30 - PLLSAI division factor for LCD clock"]
    #[inline(always)]
    pub fn pllsair(&mut self) -> PLLSAIR_W<28> {
        PLLSAIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsaicfgr](index.html) module"]
pub struct PLLSAICFGR_SPEC;
impl crate::RegisterSpec for PLLSAICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllsaicfgr::R](R) reader structure"]
impl crate::Readable for PLLSAICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllsaicfgr::W](W) writer structure"]
impl crate::Writable for PLLSAICFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSAICFGR to value 0x2400_3000"]
impl crate::Resettable for PLLSAICFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2400_3000
    }
}
