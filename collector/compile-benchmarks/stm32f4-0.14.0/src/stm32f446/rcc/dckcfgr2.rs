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
    APB = 0,
    #[doc = "1: System clock selected as I2C clock"]
    SYSCLK = 1,
    #[doc = "2: HSI clock selected as I2C clock"]
    HSI = 2,
}
impl From<FMPI2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FMPI2C1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FMPI2C1SEL` reader - I2C4 kernel clock source selection"]
pub struct FMPI2C1SEL_R(crate::FieldReader<u8, FMPI2C1SEL_A>);
impl FMPI2C1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMPI2C1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FMPI2C1SEL_A> {
        match self.bits {
            0 => Some(FMPI2C1SEL_A::APB),
            1 => Some(FMPI2C1SEL_A::SYSCLK),
            2 => Some(FMPI2C1SEL_A::HSI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == FMPI2C1SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == FMPI2C1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == FMPI2C1SEL_A::HSI
    }
}
impl core::ops::Deref for FMPI2C1SEL_R {
    type Target = crate::FieldReader<u8, FMPI2C1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMPI2C1SEL` writer - I2C4 kernel clock source selection"]
pub struct FMPI2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMPI2C1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as I2C clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::APB)
    }
    #[doc = "System clock selected as I2C clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::SYSCLK)
    }
    #[doc = "HSI clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(FMPI2C1SEL_A::HSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "HDMI CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSEL_A {
    #[doc = "0: LSE clock is selected as HDMI-CEC clock"]
    LSE = 0,
    #[doc = "1: HSI divided by 488 clock is selected as HDMI-CEC clock"]
    HSI_DIV488 = 1,
}
impl From<CECSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CECSEL` reader - HDMI CEC clock source selection"]
pub struct CECSEL_R(crate::FieldReader<bool, CECSEL_A>);
impl CECSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSEL_A {
        match self.bits {
            false => CECSEL_A::LSE,
            true => CECSEL_A::HSI_DIV488,
        }
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == CECSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSI_DIV488`"]
    #[inline(always)]
    pub fn is_hsi_div488(&self) -> bool {
        **self == CECSEL_A::HSI_DIV488
    }
}
impl core::ops::Deref for CECSEL_R {
    type Target = crate::FieldReader<bool, CECSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECSEL` writer - HDMI CEC clock source selection"]
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSE clock is selected as HDMI-CEC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSEL_A::LSE)
    }
    #[doc = "HSI divided by 488 clock is selected as HDMI-CEC clock"]
    #[inline(always)]
    pub fn hsi_div488(self) -> &'a mut W {
        self.variant(CECSEL_A::HSI_DIV488)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "SDIO/USBFS/HS clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CK48MSEL_A {
    #[doc = "0: 48MHz clock from PLL is selected"]
    PLL = 0,
    #[doc = "1: 48MHz clock from PLLSAI is selected"]
    PLLSAI = 1,
}
impl From<CK48MSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK48MSEL` reader - SDIO/USBFS/HS clock selection"]
pub struct CK48MSEL_R(crate::FieldReader<bool, CK48MSEL_A>);
impl CK48MSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CK48MSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CK48MSEL_A {
        match self.bits {
            false => CK48MSEL_A::PLL,
            true => CK48MSEL_A::PLLSAI,
        }
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == CK48MSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        **self == CK48MSEL_A::PLLSAI
    }
}
impl core::ops::Deref for CK48MSEL_R {
    type Target = crate::FieldReader<bool, CK48MSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK48MSEL` writer - SDIO/USBFS/HS clock selection"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK48MSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "48MHz clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CK48MSEL_A::PLL)
    }
    #[doc = "48MHz clock from PLLSAI is selected"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(CK48MSEL_A::PLLSAI)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "SDIO clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOSEL_A {
    #[doc = "0: 48 MHz clock is selected as SD clock"]
    CK48M = 0,
    #[doc = "1: System clock is selected as SD clock"]
    SYSCLK = 1,
}
impl From<SDIOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOSEL` reader - SDIO clock selection"]
pub struct SDIOSEL_R(crate::FieldReader<bool, SDIOSEL_A>);
impl SDIOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOSEL_A {
        match self.bits {
            false => SDIOSEL_A::CK48M,
            true => SDIOSEL_A::SYSCLK,
        }
    }
    #[doc = "Checks if the value of the field is `CK48M`"]
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        **self == SDIOSEL_A::CK48M
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == SDIOSEL_A::SYSCLK
    }
}
impl core::ops::Deref for SDIOSEL_R {
    type Target = crate::FieldReader<bool, SDIOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOSEL` writer - SDIO clock selection"]
pub struct SDIOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "48 MHz clock is selected as SD clock"]
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut W {
        self.variant(SDIOSEL_A::CK48M)
    }
    #[doc = "System clock is selected as SD clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SDIOSEL_A::SYSCLK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "SPDIF clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDIFRXSEL_A {
    #[doc = "0: SPDIF-Rx clock from PLL is selected"]
    PLL = 0,
    #[doc = "1: SPDIF-Rx clock from PLLI2S is selected"]
    PLLI2S = 1,
}
impl From<SPDIFRXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SPDIFRXSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPDIFRXSEL` reader - SPDIF clock selection"]
pub struct SPDIFRXSEL_R(crate::FieldReader<bool, SPDIFRXSEL_A>);
impl SPDIFRXSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDIFRXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIFRXSEL_A {
        match self.bits {
            false => SPDIFRXSEL_A::PLL,
            true => SPDIFRXSEL_A::PLLI2S,
        }
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == SPDIFRXSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        **self == SPDIFRXSEL_A::PLLI2S
    }
}
impl core::ops::Deref for SPDIFRXSEL_R {
    type Target = crate::FieldReader<bool, SPDIFRXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIFRXSEL` writer - SPDIF clock selection"]
pub struct SPDIFRXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIFRXSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPDIF-Rx clock from PLL is selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::PLL)
    }
    #[doc = "SPDIF-Rx clock from PLLI2S is selected"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SPDIFRXSEL_A::PLLI2S)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 26 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SDIO/USBFS/HS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SPDIF clock selection"]
    #[inline(always)]
    pub fn spdifrxsel(&self) -> SPDIFRXSEL_R {
        SPDIFRXSEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W {
        FMPI2C1SEL_W { w: self }
    }
    #[doc = "Bit 26 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W { w: self }
    }
    #[doc = "Bit 27 - SDIO/USBFS/HS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W { w: self }
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&mut self) -> SDIOSEL_W {
        SDIOSEL_W { w: self }
    }
    #[doc = "Bit 29 - SPDIF clock selection"]
    #[inline(always)]
    pub fn spdifrxsel(&mut self) -> SPDIFRXSEL_W {
        SPDIFRXSEL_W { w: self }
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
