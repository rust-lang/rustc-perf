#[doc = "Register `DCKCFGR2` reader"]
pub struct R(crate::R<DCKCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCKCFGR2` writer"]
pub struct W(crate::W<DCKCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR2_SPEC>;
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
impl From<crate::W<DCKCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C4 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FMPI2C1SEL_A {
    #[doc = "0: APB clock selected as I2C clock"]
    Apb = 0,
    #[doc = "1: System clock selected as I2C clock"]
    Sysclk = 1,
    #[doc = "2: HSI clock selected as I2C clock"]
    Hsi = 2,
}
impl From<FMPI2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FMPI2C1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FMPI2C1SEL` reader - I2C4 kernel clock source selection"]
pub type FMPI2C1SEL_R = crate::FieldReader<u8, FMPI2C1SEL_A>;
impl FMPI2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FMPI2C1SEL_A> {
        match self.bits {
            0 => Some(FMPI2C1SEL_A::Apb),
            1 => Some(FMPI2C1SEL_A::Sysclk),
            2 => Some(FMPI2C1SEL_A::Hsi),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Apb`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == FMPI2C1SEL_A::Apb
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == FMPI2C1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == FMPI2C1SEL_A::Hsi
    }
}
#[doc = "Field `FMPI2C1SEL` writer - I2C4 kernel clock source selection"]
pub type FMPI2C1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCKCFGR2_SPEC, u8, FMPI2C1SEL_A, 2, O>;
impl<'a, const O: u8> FMPI2C1SEL_W<'a, O> {
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::Apb)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::Sysclk)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::Hsi)
    }
}
#[doc = "HDMI CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSEL_A {
    #[doc = "0: LSE clock is selected as HDMI-CEC clock"]
    Lse = 0,
    #[doc = "1: HSI divided by 488 clock is selected as HDMI-CEC clock"]
    HsiDiv488 = 1,
}
impl From<CECSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECSEL` reader - HDMI CEC clock source selection"]
pub type CECSEL_R = crate::BitReader<CECSEL_A>;
impl CECSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSEL_A {
        match self.bits {
            false => CECSEL_A::Lse,
            true => CECSEL_A::HsiDiv488,
        }
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL_A::Lse
    }
    #[doc = "Checks if the value of the field is `HsiDiv488`"]
    #[inline(always)]
    pub fn is_hsi_div488(&self) -> bool {
        *self == CECSEL_A::HsiDiv488
    }
}
#[doc = "Field `CECSEL` writer - HDMI CEC clock source selection"]
pub type CECSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, CECSEL_A, O>;
impl<'a, const O: u8> CECSEL_W<'a, O> {
    #[doc = "LSE clock is selected as HDMI-CEC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSEL_A::Lse)
    }
    #[doc = "HSI divided by 488 clock is selected as HDMI-CEC clock"]
    #[inline(always)]
    pub fn hsi_div488(self) -> &'a mut W {
        self.variant(CECSEL_A::HsiDiv488)
    }
}
#[doc = "SDIO/USBFS/HS clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK48MSEL_A {
    #[doc = "0: 48MHz clock from PLL is selected"]
    Pll = 0,
    #[doc = "1: 48MHz clock from PLLSAI is selected"]
    Pllsai = 1,
}
impl From<CK48MSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK48MSEL` reader - SDIO/USBFS/HS clock selection"]
pub type CK48MSEL_R = crate::BitReader<CK48MSEL_A>;
impl CK48MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK48MSEL_A {
        match self.bits {
            false => CK48MSEL_A::Pll,
            true => CK48MSEL_A::Pllsai,
        }
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL_A::Pll
    }
    #[doc = "Checks if the value of the field is `Pllsai`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL_A::Pllsai
    }
}
#[doc = "Field `CK48MSEL` writer - SDIO/USBFS/HS clock selection"]
pub type CK48MSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, CK48MSEL_A, O>;
impl<'a, const O: u8> CK48MSEL_W<'a, O> {
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CK48MSEL_A::Pll)
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(CK48MSEL_A::Pllsai)
    }
}
#[doc = "SDIO clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOSEL_A {
    #[doc = "0: 48 MHz clock is selected as SD clock"]
    Ck48m = 0,
    #[doc = "1: System clock is selected as SD clock"]
    Sysclk = 1,
}
impl From<SDIOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOSEL` reader - SDIO clock selection"]
pub type SDIOSEL_R = crate::BitReader<SDIOSEL_A>;
impl SDIOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOSEL_A {
        match self.bits {
            false => SDIOSEL_A::Ck48m,
            true => SDIOSEL_A::Sysclk,
        }
    }
    #[doc = "Checks if the value of the field is `Ck48m`"]
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDIOSEL_A::Ck48m
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDIOSEL_A::Sysclk
    }
}
#[doc = "Field `SDIOSEL` writer - SDIO clock selection"]
pub type SDIOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, SDIOSEL_A, O>;
impl<'a, const O: u8> SDIOSEL_W<'a, O> {
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut W {
        self.variant(SDIOSEL_A::Ck48m)
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SDIOSEL_A::Sysclk)
    }
}
#[doc = "SPDIF clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIFRXSEL_A {
    #[doc = "0: SPDIF-Rx clock from PLL is selected"]
    Pll = 0,
    #[doc = "1: SPDIF-Rx clock from PLLI2S is selected"]
    Plli2s = 1,
}
impl From<SPDIFRXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SPDIFRXSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDIFRXSEL` reader - SPDIF clock selection"]
pub type SPDIFRXSEL_R = crate::BitReader<SPDIFRXSEL_A>;
impl SPDIFRXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIFRXSEL_A {
        match self.bits {
            false => SPDIFRXSEL_A::Pll,
            true => SPDIFRXSEL_A::Plli2s,
        }
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SPDIFRXSEL_A::Pll
    }
    #[doc = "Checks if the value of the field is `Plli2s`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SPDIFRXSEL_A::Plli2s
    }
}
#[doc = "Field `SPDIFRXSEL` writer - SPDIF clock selection"]
pub type SPDIFRXSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, SPDIFRXSEL_A, O>;
impl<'a, const O: u8> SPDIFRXSEL_W<'a, O> {
    #[doc = "SPDIF-Rx clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::Pll)
    }
    #[doc = "SPDIF-Rx clock from PLLI2S is selected"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::Plli2s)
    }
}
impl R {
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 26 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SDIO/USBFS/HS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SPDIF clock selection"]
    #[inline(always)]
    pub fn spdifrxsel(&self) -> SPDIFRXSEL_R {
        SPDIFRXSEL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W<22> {
        FMPI2C1SEL_W::new(self)
    }
    #[doc = "Bit 26 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W<26> {
        CECSEL_W::new(self)
    }
    #[doc = "Bit 27 - SDIO/USBFS/HS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<27> {
        CK48MSEL_W::new(self)
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&mut self) -> SDIOSEL_W<28> {
        SDIOSEL_W::new(self)
    }
    #[doc = "Bit 29 - SPDIF clock selection"]
    #[inline(always)]
    pub fn spdifrxsel(&mut self) -> SPDIFRXSEL_W<29> {
        SPDIFRXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dedicated clocks configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dckcfgr2](index.html) module"]
pub struct DCKCFGR2_SPEC;
impl crate::RegisterSpec for DCKCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dckcfgr2::R](R) reader structure"]
impl crate::Readable for DCKCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dckcfgr2::W](W) writer structure"]
impl crate::Writable for DCKCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCKCFGR2 to value 0"]
impl crate::Resettable for DCKCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
