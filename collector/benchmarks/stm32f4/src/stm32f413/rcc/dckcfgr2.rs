#[doc = "Reader of register DCKCFGR2"]
pub type R = crate::R<u32, super::DCKCFGR2>;
#[doc = "Writer for register DCKCFGR2"]
pub type W = crate::W<u32, super::DCKCFGR2>;
#[doc = "Register DCKCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCKCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDIO clock selection.\n\nValue on reset: 0"]
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
#[doc = "Reader of field `SDIOSEL`"]
pub type SDIOSEL_R = crate::R<bool, SDIOSEL_A>;
impl SDIOSEL_R {
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
        *self == SDIOSEL_A::CK48M
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDIOSEL_A::SYSCLK
    }
}
#[doc = "Write proxy for field `SDIOSEL`"]
pub struct SDIOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "SDIO/USBFS clock selection.\n\nValue on reset: 0"]
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
#[doc = "Reader of field `CK48MSEL`"]
pub type CK48MSEL_R = crate::R<bool, CK48MSEL_A>;
impl CK48MSEL_R {
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
        *self == CK48MSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL_A::PLLSAI
    }
}
#[doc = "Write proxy for field `CK48MSEL`"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CK48MSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "I2CFMP1 kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Reader of field `FMPI2C1SEL`"]
pub type FMPI2C1SEL_R = crate::R<u8, FMPI2C1SEL_A>;
impl FMPI2C1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FMPI2C1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FMPI2C1SEL_A::APB),
            1 => Val(FMPI2C1SEL_A::SYSCLK),
            2 => Val(FMPI2C1SEL_A::HSI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == FMPI2C1SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == FMPI2C1SEL_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == FMPI2C1SEL_A::HSI
    }
}
#[doc = "Write proxy for field `FMPI2C1SEL`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "LPTIM1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: APB1 clock (PCLK1) selected as LPTILM1 clock"]
    APB1 = 0,
    #[doc = "1: LSI clock is selected as LPTILM1 clock"]
    LSI = 1,
    #[doc = "2: HSI clock is selected as LPTILM1 clock"]
    HSI = 2,
    #[doc = "3: LSE clock is selected as LPTILM1 clock"]
    LSE = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTIM1SEL`"]
pub type LPTIM1SEL_R = crate::R<u8, LPTIM1SEL_A>;
impl LPTIM1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::APB1,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `APB1`"]
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL_A::APB1
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::LSE
    }
}
#[doc = "Write proxy for field `LPTIM1SEL`"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::APB1)
    }
    #[doc = "LSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    #[doc = "HSI clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI)
    }
    #[doc = "LSE clock is selected as LPTILM1 clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - SDIO clock selection."]
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SDIO/USBFS clock selection."]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - I2CFMP1 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 28 - SDIO clock selection."]
    #[inline(always)]
    pub fn sdiosel(&mut self) -> SDIOSEL_W {
        SDIOSEL_W { w: self }
    }
    #[doc = "Bit 27 - SDIO/USBFS clock selection."]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - I2CFMP1 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W {
        FMPI2C1SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
}
