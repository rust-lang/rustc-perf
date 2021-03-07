#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Microcontroller clock output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO2_A {
    #[doc = "0: System clock (SYSCLK) selected"]
    SYSCLK = 0,
    #[doc = "1: PLLI2S clock selected"]
    PLLI2S = 1,
    #[doc = "2: HSE oscillator clock selected"]
    HSE = 2,
    #[doc = "3: PLL clock selected"]
    PLL = 3,
}
impl From<MCO2_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO2`"]
pub type MCO2_R = crate::R<u8, MCO2_A>;
impl MCO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCO2_A {
        match self.bits {
            0 => MCO2_A::SYSCLK,
            1 => MCO2_A::PLLI2S,
            2 => MCO2_A::HSE,
            3 => MCO2_A::PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == MCO2_A::PLLI2S
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO2_A::PLL
    }
}
#[doc = "Write proxy for field `MCO2`"]
pub struct MCO2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "System clock (SYSCLK) selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO2_A::SYSCLK)
    }
    #[doc = "PLLI2S clock selected"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(MCO2_A::PLLI2S)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO2_A::HSE)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO2_A::PLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "MCO2 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO2PRE_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "4: Division by 2"]
    DIV2 = 4,
    #[doc = "5: Division by 3"]
    DIV3 = 5,
    #[doc = "6: Division by 4"]
    DIV4 = 6,
    #[doc = "7: Division by 5"]
    DIV5 = 7,
}
impl From<MCO2PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2PRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO2PRE`"]
pub type MCO2PRE_R = crate::R<u8, MCO2PRE_A>;
impl MCO2PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO2PRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO2PRE_A::DIV1),
            4 => Val(MCO2PRE_A::DIV2),
            5 => Val(MCO2PRE_A::DIV3),
            6 => Val(MCO2PRE_A::DIV4),
            7 => Val(MCO2PRE_A::DIV5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCO2PRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCO2PRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == MCO2PRE_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCO2PRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == MCO2PRE_A::DIV5
    }
}
#[doc = "Write proxy for field `MCO2PRE`"]
pub struct MCO2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO2PRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV2)
    }
    #[doc = "Division by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV3)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV4)
    }
    #[doc = "Division by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "MCO1 prescaler"]
pub type MCO1PRE_A = MCO2PRE_A;
#[doc = "Reader of field `MCO1PRE`"]
pub type MCO1PRE_R = crate::R<u8, MCO2PRE_A>;
#[doc = "Write proxy for field `MCO1PRE`"]
pub struct MCO1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO1PRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV1)
    }
    #[doc = "Division by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV2)
    }
    #[doc = "Division by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV3)
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV4)
    }
    #[doc = "Division by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(MCO2PRE_A::DIV5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "I2S clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SSRC_A {
    #[doc = "0: PLLI2S clock used as I2S clock source"]
    PLLI2S = 0,
    #[doc = "1: External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    CKIN = 1,
}
impl From<I2SSRC_A> for bool {
    #[inline(always)]
    fn from(variant: I2SSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2SSRC`"]
pub type I2SSRC_R = crate::R<bool, I2SSRC_A>;
impl I2SSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSRC_A {
        match self.bits {
            false => I2SSRC_A::PLLI2S,
            true => I2SSRC_A::CKIN,
        }
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == I2SSRC_A::PLLI2S
    }
    #[doc = "Checks if the value of the field is `CKIN`"]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == I2SSRC_A::CKIN
    }
}
#[doc = "Write proxy for field `I2SSRC`"]
pub struct I2SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLLI2S clock used as I2S clock source"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(I2SSRC_A::PLLI2S)
    }
    #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut W {
        self.variant(I2SSRC_A::CKIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Microcontroller clock output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO1_A {
    #[doc = "0: HSI clock selected"]
    HSI = 0,
    #[doc = "1: LSE oscillator selected"]
    LSE = 1,
    #[doc = "2: HSE oscillator clock selected"]
    HSE = 2,
    #[doc = "3: PLL clock selected"]
    PLL = 3,
}
impl From<MCO1_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO1`"]
pub type MCO1_R = crate::R<u8, MCO1_A>;
impl MCO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCO1_A {
        match self.bits {
            0 => MCO1_A::HSI,
            1 => MCO1_A::LSE,
            2 => MCO1_A::HSE,
            3 => MCO1_A::PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO1_A::PLL
    }
}
#[doc = "Write proxy for field `MCO1`"]
pub struct MCO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HSI clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO1_A::HSI)
    }
    #[doc = "LSE oscillator selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO1_A::LSE)
    }
    #[doc = "HSE oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO1_A::HSE)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO1_A::PLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `RTCPRE`"]
pub type RTCPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCPRE`"]
pub struct RTCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "APB high-speed prescaler (APB2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPRE2_A {
    #[doc = "0: HCLK not divided"]
    DIV1 = 0,
    #[doc = "4: HCLK divided by 2"]
    DIV2 = 4,
    #[doc = "5: HCLK divided by 4"]
    DIV4 = 5,
    #[doc = "6: HCLK divided by 8"]
    DIV8 = 6,
    #[doc = "7: HCLK divided by 16"]
    DIV16 = 7,
}
impl From<PPRE2_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PPRE2`"]
pub type PPRE2_R = crate::R<u8, PPRE2_A>;
impl PPRE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PPRE2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PPRE2_A::DIV1),
            4 => Val(PPRE2_A::DIV2),
            5 => Val(PPRE2_A::DIV4),
            6 => Val(PPRE2_A::DIV8),
            7 => Val(PPRE2_A::DIV16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE2_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE2_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE2_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE2_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE2_A::DIV16
    }
}
#[doc = "Write proxy for field `PPRE2`"]
pub struct PPRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPRE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "APB Low speed prescaler (APB1)"]
pub type PPRE1_A = PPRE2_A;
#[doc = "Reader of field `PPRE1`"]
pub type PPRE1_R = crate::R<u8, PPRE2_A>;
#[doc = "Write proxy for field `PPRE1`"]
pub struct PPRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPRE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: SYSCLK not divided"]
    DIV1 = 0,
    #[doc = "8: SYSCLK divided by 2"]
    DIV2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    DIV4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    DIV8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    DIV16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    DIV64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    DIV128 = 13,
    #[doc = "14: SYSCLK divided by 256"]
    DIV256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    DIV512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HPRE`"]
pub type HPRE_R = crate::R<u8, HPRE_A>;
impl HPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HPRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HPRE_A::DIV1),
            8 => Val(HPRE_A::DIV2),
            9 => Val(HPRE_A::DIV4),
            10 => Val(HPRE_A::DIV8),
            11 => Val(HPRE_A::DIV16),
            12 => Val(HPRE_A::DIV64),
            13 => Val(HPRE_A::DIV128),
            14 => Val(HPRE_A::DIV256),
            15 => Val(HPRE_A::DIV512),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::DIV512
    }
}
#[doc = "Write proxy for field `HPRE`"]
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::DIV1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::DIV2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::DIV4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::DIV8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::DIV16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::DIV64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::DIV128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::DIV256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::DIV512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "0: HSE oscillator used as system clock"]
    HSI = 0,
    #[doc = "1: HSI oscillator used as system clock"]
    HSE = 1,
    #[doc = "2: PLL used as system clock"]
    PLL = 2,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWS`"]
pub type SWS_R = crate::R<u8, SWS_A>;
impl SWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWS_A::HSI),
            1 => Val(SWS_A::HSE),
            2 => Val(SWS_A::PLL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWS_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWS_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWS_A::PLL
    }
}
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: HSI selected as system clock"]
    HSI = 0,
    #[doc = "1: HSE selected as system clock"]
    HSE = 1,
    #[doc = "2: PLL selected as system clock"]
    PLL = 2,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<u8, SW_A>;
impl SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW_A::HSI),
            1 => Val(SW_A::HSE),
            2 => Val(SW_A::PLL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW_A::PLL
    }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::HSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::HSE)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::PLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&mut self) -> MCO2_W {
        MCO2_W { w: self }
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W {
        MCO2PRE_W { w: self }
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W {
        MCO1PRE_W { w: self }
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W {
        I2SSRC_W { w: self }
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&mut self) -> MCO1_W {
        MCO1_W { w: self }
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W {
        RTCPRE_W { w: self }
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W {
        PPRE2_W { w: self }
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W {
        PPRE1_W { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
}
