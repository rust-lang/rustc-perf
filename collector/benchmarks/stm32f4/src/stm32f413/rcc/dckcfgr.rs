#[doc = "Reader of register DCKCFGR"]
pub type R = crate::R<u32, super::DCKCFGR>;
#[doc = "Writer for register DCKCFGR"]
pub type W = crate::W<u32, super::DCKCFGR>;
#[doc = "Register DCKCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCKCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DFSDM1 audio clock selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDFSDM1ASEL_A {
    #[doc = "0: CK_I2S_APB1 selected as audio clock"]
    I2S1 = 0,
    #[doc = "1: CK_I2S_APB2 selected as audio clock"]
    I2S2 = 1,
}
impl From<CKDFSDM1ASEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM1ASEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKDFSDM1ASEL`"]
pub type CKDFSDM1ASEL_R = crate::R<bool, CKDFSDM1ASEL_A>;
impl CKDFSDM1ASEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKDFSDM1ASEL_A {
        match self.bits {
            false => CKDFSDM1ASEL_A::I2S1,
            true => CKDFSDM1ASEL_A::I2S2,
        }
    }
    #[doc = "Checks if the value of the field is `I2S1`"]
    #[inline(always)]
    pub fn is_i2s1(&self) -> bool {
        *self == CKDFSDM1ASEL_A::I2S1
    }
    #[doc = "Checks if the value of the field is `I2S2`"]
    #[inline(always)]
    pub fn is_i2s2(&self) -> bool {
        *self == CKDFSDM1ASEL_A::I2S2
    }
}
#[doc = "Write proxy for field `CKDFSDM1ASEL`"]
pub struct CKDFSDM1ASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDFSDM1ASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDFSDM1ASEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CK_I2S_APB1 selected as audio clock"]
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2S1)
    }
    #[doc = "CK_I2S_APB2 selected as audio clock"]
    #[inline(always)]
    pub fn i2s2(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2S2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Timers clocks prescalers selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPRE_A {
    #[doc = "0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    MUL2 = 0,
    #[doc = "1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    MUL4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMPRE`"]
pub type TIMPRE_R = crate::R<bool, TIMPRE_A>;
impl TIMPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::MUL2,
            true => TIMPRE_A::MUL4,
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == TIMPRE_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == TIMPRE_A::MUL4
    }
}
#[doc = "Write proxy for field `TIMPRE`"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL2)
    }
    #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::MUL4)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "I2S APB1 clocks source selection (I2S2/3)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S1SRC_A {
    #[doc = "0: I2Sx clock frequency = f(PLLI2S_R)"]
    PLLI2SR = 0,
    #[doc = "1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    I2S_CKIN = 1,
    #[doc = "2: I2Sx clock frequency = f(PLL_R)"]
    PLLR = 2,
    #[doc = "3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    HSI_HSE = 3,
}
impl From<I2S1SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2S1SRC`"]
pub type I2S1SRC_R = crate::R<u8, I2S1SRC_A>;
impl I2S1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S1SRC_A {
        match self.bits {
            0 => I2S1SRC_A::PLLI2SR,
            1 => I2S1SRC_A::I2S_CKIN,
            2 => I2S1SRC_A::PLLR,
            3 => I2S1SRC_A::HSI_HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLLI2SR`"]
    #[inline(always)]
    pub fn is_plli2sr(&self) -> bool {
        *self == I2S1SRC_A::PLLI2SR
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2S1SRC_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `PLLR`"]
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == I2S1SRC_A::PLLR
    }
    #[doc = "Checks if the value of the field is `HSI_HSE`"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2S1SRC_A::HSI_HSE
    }
}
#[doc = "Write proxy for field `I2S1SRC`"]
pub struct I2S1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S1SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "I2Sx clock frequency = f(PLLI2S_R)"]
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::PLLI2SR)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S1SRC_A::I2S_CKIN)
    }
    #[doc = "I2Sx clock frequency = f(PLL_R)"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::PLLR)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2S1SRC_A::HSI_HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "I2S APB2 clocks source selection (I2S1/4/5)"]
pub type I2S2SRC_A = I2S1SRC_A;
#[doc = "Reader of field `I2S2SRC`"]
pub type I2S2SRC_R = crate::R<u8, I2S1SRC_A>;
#[doc = "Write proxy for field `I2S2SRC`"]
pub struct I2S2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S2SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "I2Sx clock frequency = f(PLLI2S_R)"]
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::PLLI2SR)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S1SRC_A::I2S_CKIN)
    }
    #[doc = "I2Sx clock frequency = f(PLL_R)"]
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::PLLR)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2S1SRC_A::HSI_HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "PLLI2S division factor for SAI1 A/B clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLI2SDIVR_A {
    #[doc = "0: PLLI2SDIVQ = /1"]
    DIV1 = 0,
    #[doc = "1: PLLI2SDIVQ = /2"]
    DIV2 = 1,
    #[doc = "2: PLLI2SDIVQ = /3"]
    DIV3 = 2,
    #[doc = "3: PLLI2SDIVQ = /4"]
    DIV4 = 3,
    #[doc = "4: PLLI2SDIVQ = /5"]
    DIV5 = 4,
    #[doc = "5: PLLI2SDIVQ = /6"]
    DIV6 = 5,
    #[doc = "6: PLLI2SDIVQ = /7"]
    DIV7 = 6,
    #[doc = "7: PLLI2SDIVQ = /8"]
    DIV8 = 7,
    #[doc = "8: PLLI2SDIVQ = /9"]
    DIV9 = 8,
    #[doc = "9: PLLI2SDIVQ = /10"]
    DIV10 = 9,
    #[doc = "10: PLLI2SDIVQ = /11"]
    DIV11 = 10,
    #[doc = "11: PLLI2SDIVQ = /12"]
    DIV12 = 11,
    #[doc = "12: PLLI2SDIVQ = /13"]
    DIV13 = 12,
    #[doc = "13: PLLI2SDIVQ = /14"]
    DIV14 = 13,
    #[doc = "14: PLLI2SDIVQ = /15"]
    DIV15 = 14,
    #[doc = "15: PLLI2SDIVQ = /16"]
    DIV16 = 15,
    #[doc = "16: PLLI2SDIVQ = /17"]
    DIV17 = 16,
    #[doc = "17: PLLI2SDIVQ = /18"]
    DIV18 = 17,
    #[doc = "18: PLLI2SDIVQ = /19"]
    DIV19 = 18,
    #[doc = "19: PLLI2SDIVQ = /20"]
    DIV20 = 19,
    #[doc = "20: PLLI2SDIVQ = /21"]
    DIV21 = 20,
    #[doc = "21: PLLI2SDIVQ = /22"]
    DIV22 = 21,
    #[doc = "22: PLLI2SDIVQ = /23"]
    DIV23 = 22,
    #[doc = "23: PLLI2SDIVQ = /24"]
    DIV24 = 23,
    #[doc = "24: PLLI2SDIVQ = /25"]
    DIV25 = 24,
    #[doc = "25: PLLI2SDIVQ = /26"]
    DIV26 = 25,
    #[doc = "26: PLLI2SDIVQ = /27"]
    DIV27 = 26,
    #[doc = "27: PLLI2SDIVQ = /28"]
    DIV28 = 27,
    #[doc = "28: PLLI2SDIVQ = /29"]
    DIV29 = 28,
    #[doc = "29: PLLI2SDIVQ = /30"]
    DIV30 = 29,
    #[doc = "30: PLLI2SDIVQ = /31"]
    DIV31 = 30,
    #[doc = "31: PLLI2SDIVQ = /32"]
    DIV32 = 31,
}
impl From<PLLI2SDIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLI2SDIVR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLI2SDIVR`"]
pub type PLLI2SDIVR_R = crate::R<u8, PLLI2SDIVR_A>;
impl PLLI2SDIVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SDIVR_A {
        match self.bits {
            0 => PLLI2SDIVR_A::DIV1,
            1 => PLLI2SDIVR_A::DIV2,
            2 => PLLI2SDIVR_A::DIV3,
            3 => PLLI2SDIVR_A::DIV4,
            4 => PLLI2SDIVR_A::DIV5,
            5 => PLLI2SDIVR_A::DIV6,
            6 => PLLI2SDIVR_A::DIV7,
            7 => PLLI2SDIVR_A::DIV8,
            8 => PLLI2SDIVR_A::DIV9,
            9 => PLLI2SDIVR_A::DIV10,
            10 => PLLI2SDIVR_A::DIV11,
            11 => PLLI2SDIVR_A::DIV12,
            12 => PLLI2SDIVR_A::DIV13,
            13 => PLLI2SDIVR_A::DIV14,
            14 => PLLI2SDIVR_A::DIV15,
            15 => PLLI2SDIVR_A::DIV16,
            16 => PLLI2SDIVR_A::DIV17,
            17 => PLLI2SDIVR_A::DIV18,
            18 => PLLI2SDIVR_A::DIV19,
            19 => PLLI2SDIVR_A::DIV20,
            20 => PLLI2SDIVR_A::DIV21,
            21 => PLLI2SDIVR_A::DIV22,
            22 => PLLI2SDIVR_A::DIV23,
            23 => PLLI2SDIVR_A::DIV24,
            24 => PLLI2SDIVR_A::DIV25,
            25 => PLLI2SDIVR_A::DIV26,
            26 => PLLI2SDIVR_A::DIV27,
            27 => PLLI2SDIVR_A::DIV28,
            28 => PLLI2SDIVR_A::DIV29,
            29 => PLLI2SDIVR_A::DIV30,
            30 => PLLI2SDIVR_A::DIV31,
            31 => PLLI2SDIVR_A::DIV32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV31
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLI2SDIVR_A::DIV32
    }
}
#[doc = "Write proxy for field `PLLI2SDIVR`"]
pub struct PLLI2SDIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SDIVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SDIVR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLI2SDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV1)
    }
    #[doc = "PLLI2SDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV2)
    }
    #[doc = "PLLI2SDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV3)
    }
    #[doc = "PLLI2SDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV4)
    }
    #[doc = "PLLI2SDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV5)
    }
    #[doc = "PLLI2SDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV6)
    }
    #[doc = "PLLI2SDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV7)
    }
    #[doc = "PLLI2SDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV8)
    }
    #[doc = "PLLI2SDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV9)
    }
    #[doc = "PLLI2SDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV10)
    }
    #[doc = "PLLI2SDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV11)
    }
    #[doc = "PLLI2SDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV12)
    }
    #[doc = "PLLI2SDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV13)
    }
    #[doc = "PLLI2SDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV14)
    }
    #[doc = "PLLI2SDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV15)
    }
    #[doc = "PLLI2SDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV16)
    }
    #[doc = "PLLI2SDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV17)
    }
    #[doc = "PLLI2SDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV18)
    }
    #[doc = "PLLI2SDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV19)
    }
    #[doc = "PLLI2SDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV20)
    }
    #[doc = "PLLI2SDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV21)
    }
    #[doc = "PLLI2SDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV22)
    }
    #[doc = "PLLI2SDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV23)
    }
    #[doc = "PLLI2SDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV24)
    }
    #[doc = "PLLI2SDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV25)
    }
    #[doc = "PLLI2SDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV26)
    }
    #[doc = "PLLI2SDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV27)
    }
    #[doc = "PLLI2SDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV28)
    }
    #[doc = "PLLI2SDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV29)
    }
    #[doc = "PLLI2SDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV30)
    }
    #[doc = "PLLI2SDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV31)
    }
    #[doc = "PLLI2SDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLI2SDIVR_A::DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "PLL division factor for SAI1 A/B clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLDIVR_A {
    #[doc = "0: PLLSAIDIVQ = /1"]
    DIV1 = 0,
    #[doc = "1: PLLSAIDIVQ = /2"]
    DIV2 = 1,
    #[doc = "2: PLLSAIDIVQ = /3"]
    DIV3 = 2,
    #[doc = "3: PLLSAIDIVQ = /4"]
    DIV4 = 3,
    #[doc = "4: PLLSAIDIVQ = /5"]
    DIV5 = 4,
    #[doc = "5: PLLSAIDIVQ = /6"]
    DIV6 = 5,
    #[doc = "6: PLLSAIDIVQ = /7"]
    DIV7 = 6,
    #[doc = "7: PLLSAIDIVQ = /8"]
    DIV8 = 7,
    #[doc = "8: PLLSAIDIVQ = /9"]
    DIV9 = 8,
    #[doc = "9: PLLSAIDIVQ = /10"]
    DIV10 = 9,
    #[doc = "10: PLLSAIDIVQ = /11"]
    DIV11 = 10,
    #[doc = "11: PLLSAIDIVQ = /12"]
    DIV12 = 11,
    #[doc = "12: PLLSAIDIVQ = /13"]
    DIV13 = 12,
    #[doc = "13: PLLSAIDIVQ = /14"]
    DIV14 = 13,
    #[doc = "14: PLLSAIDIVQ = /15"]
    DIV15 = 14,
    #[doc = "15: PLLSAIDIVQ = /16"]
    DIV16 = 15,
    #[doc = "16: PLLSAIDIVQ = /17"]
    DIV17 = 16,
    #[doc = "17: PLLSAIDIVQ = /18"]
    DIV18 = 17,
    #[doc = "18: PLLSAIDIVQ = /19"]
    DIV19 = 18,
    #[doc = "19: PLLSAIDIVQ = /20"]
    DIV20 = 19,
    #[doc = "20: PLLSAIDIVQ = /21"]
    DIV21 = 20,
    #[doc = "21: PLLSAIDIVQ = /22"]
    DIV22 = 21,
    #[doc = "22: PLLSAIDIVQ = /23"]
    DIV23 = 22,
    #[doc = "23: PLLSAIDIVQ = /24"]
    DIV24 = 23,
    #[doc = "24: PLLSAIDIVQ = /25"]
    DIV25 = 24,
    #[doc = "25: PLLSAIDIVQ = /26"]
    DIV26 = 25,
    #[doc = "26: PLLSAIDIVQ = /27"]
    DIV27 = 26,
    #[doc = "27: PLLSAIDIVQ = /28"]
    DIV28 = 27,
    #[doc = "28: PLLSAIDIVQ = /29"]
    DIV29 = 28,
    #[doc = "29: PLLSAIDIVQ = /30"]
    DIV30 = 29,
    #[doc = "30: PLLSAIDIVQ = /31"]
    DIV31 = 30,
    #[doc = "31: PLLSAIDIVQ = /32"]
    DIV32 = 31,
}
impl From<PLLDIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLDIVR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLDIVR`"]
pub type PLLDIVR_R = crate::R<u8, PLLDIVR_A>;
impl PLLDIVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLDIVR_A {
        match self.bits {
            0 => PLLDIVR_A::DIV1,
            1 => PLLDIVR_A::DIV2,
            2 => PLLDIVR_A::DIV3,
            3 => PLLDIVR_A::DIV4,
            4 => PLLDIVR_A::DIV5,
            5 => PLLDIVR_A::DIV6,
            6 => PLLDIVR_A::DIV7,
            7 => PLLDIVR_A::DIV8,
            8 => PLLDIVR_A::DIV9,
            9 => PLLDIVR_A::DIV10,
            10 => PLLDIVR_A::DIV11,
            11 => PLLDIVR_A::DIV12,
            12 => PLLDIVR_A::DIV13,
            13 => PLLDIVR_A::DIV14,
            14 => PLLDIVR_A::DIV15,
            15 => PLLDIVR_A::DIV16,
            16 => PLLDIVR_A::DIV17,
            17 => PLLDIVR_A::DIV18,
            18 => PLLDIVR_A::DIV19,
            19 => PLLDIVR_A::DIV20,
            20 => PLLDIVR_A::DIV21,
            21 => PLLDIVR_A::DIV22,
            22 => PLLDIVR_A::DIV23,
            23 => PLLDIVR_A::DIV24,
            24 => PLLDIVR_A::DIV25,
            25 => PLLDIVR_A::DIV26,
            26 => PLLDIVR_A::DIV27,
            27 => PLLDIVR_A::DIV28,
            28 => PLLDIVR_A::DIV29,
            29 => PLLDIVR_A::DIV30,
            30 => PLLDIVR_A::DIV31,
            31 => PLLDIVR_A::DIV32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLDIVR_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLDIVR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLDIVR_A::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLDIVR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLDIVR_A::DIV5
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLDIVR_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLDIVR_A::DIV7
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLDIVR_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLDIVR_A::DIV9
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLDIVR_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLDIVR_A::DIV11
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLDIVR_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLDIVR_A::DIV13
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLDIVR_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLDIVR_A::DIV15
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLDIVR_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV17`"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLDIVR_A::DIV17
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLDIVR_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV19`"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLDIVR_A::DIV19
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLDIVR_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV21`"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLDIVR_A::DIV21
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLDIVR_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV23`"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLDIVR_A::DIV23
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLDIVR_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV25`"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLDIVR_A::DIV25
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLDIVR_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV27`"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLDIVR_A::DIV27
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLDIVR_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV29`"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLDIVR_A::DIV29
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLDIVR_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV31`"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLDIVR_A::DIV31
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLDIVR_A::DIV32
    }
}
#[doc = "Write proxy for field `PLLDIVR`"]
pub struct PLLDIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDIVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLDIVR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PLLSAIDIVQ = /1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV1)
    }
    #[doc = "PLLSAIDIVQ = /2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV2)
    }
    #[doc = "PLLSAIDIVQ = /3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV3)
    }
    #[doc = "PLLSAIDIVQ = /4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV4)
    }
    #[doc = "PLLSAIDIVQ = /5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV5)
    }
    #[doc = "PLLSAIDIVQ = /6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV6)
    }
    #[doc = "PLLSAIDIVQ = /7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV7)
    }
    #[doc = "PLLSAIDIVQ = /8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV8)
    }
    #[doc = "PLLSAIDIVQ = /9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV9)
    }
    #[doc = "PLLSAIDIVQ = /10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV10)
    }
    #[doc = "PLLSAIDIVQ = /11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV11)
    }
    #[doc = "PLLSAIDIVQ = /12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV12)
    }
    #[doc = "PLLSAIDIVQ = /13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV13)
    }
    #[doc = "PLLSAIDIVQ = /14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV14)
    }
    #[doc = "PLLSAIDIVQ = /15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV15)
    }
    #[doc = "PLLSAIDIVQ = /16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV16)
    }
    #[doc = "PLLSAIDIVQ = /17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV17)
    }
    #[doc = "PLLSAIDIVQ = /18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV18)
    }
    #[doc = "PLLSAIDIVQ = /19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV19)
    }
    #[doc = "PLLSAIDIVQ = /20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV20)
    }
    #[doc = "PLLSAIDIVQ = /21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV21)
    }
    #[doc = "PLLSAIDIVQ = /22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV22)
    }
    #[doc = "PLLSAIDIVQ = /23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV23)
    }
    #[doc = "PLLSAIDIVQ = /24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV24)
    }
    #[doc = "PLLSAIDIVQ = /25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV25)
    }
    #[doc = "PLLSAIDIVQ = /26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV26)
    }
    #[doc = "PLLSAIDIVQ = /27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV27)
    }
    #[doc = "PLLSAIDIVQ = /28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV28)
    }
    #[doc = "PLLSAIDIVQ = /29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV29)
    }
    #[doc = "PLLSAIDIVQ = /30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV30)
    }
    #[doc = "PLLSAIDIVQ = /31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV31)
    }
    #[doc = "PLLSAIDIVQ = /32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PLLDIVR_A::DIV32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "DFSDM2 audio clock selection"]
pub type CKDFSDM2ASEL_A = CKDFSDM1ASEL_A;
#[doc = "Reader of field `CKDFSDM2ASEL`"]
pub type CKDFSDM2ASEL_R = crate::R<bool, CKDFSDM1ASEL_A>;
#[doc = "Write proxy for field `CKDFSDM2ASEL`"]
pub struct CKDFSDM2ASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDFSDM2ASEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDFSDM2ASEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CK_I2S_APB1 selected as audio clock"]
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2S1)
    }
    #[doc = "CK_I2S_APB2 selected as audio clock"]
    #[inline(always)]
    pub fn i2s2(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2S2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "SAI1-A clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1ASRC_A {
    #[doc = "0: SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    PLLSAI = 0,
    #[doc = "1: SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    PLLI2S = 1,
    #[doc = "2: SAI1-A clock frequency = Alternate function input frequency"]
    I2S_CKIN = 2,
}
impl From<SAI1ASRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1ASRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1ASRC`"]
pub type SAI1ASRC_R = crate::R<u8, SAI1ASRC_A>;
impl SAI1ASRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1ASRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1ASRC_A::PLLSAI),
            1 => Val(SAI1ASRC_A::PLLI2S),
            2 => Val(SAI1ASRC_A::I2S_CKIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1ASRC_A::PLLSAI
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1ASRC_A::PLLI2S
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1ASRC_A::I2S_CKIN
    }
}
#[doc = "Write proxy for field `SAI1ASRC`"]
pub struct SAI1ASRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1ASRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1ASRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SAI1-A clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::PLLSAI)
    }
    #[doc = "SAI1-A clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::PLLI2S)
    }
    #[doc = "SAI1-A clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1ASRC_A::I2S_CKIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "SAI1-B clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1BSRC_A {
    #[doc = "0: SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    PLLSAI = 0,
    #[doc = "1: SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    PLLI2S = 1,
    #[doc = "2: SAI1-B clock frequency = Alternate function input frequency"]
    I2S_CKIN = 2,
}
impl From<SAI1BSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1BSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1BSRC`"]
pub type SAI1BSRC_R = crate::R<u8, SAI1BSRC_A>;
impl SAI1BSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1BSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1BSRC_A::PLLSAI),
            1 => Val(SAI1BSRC_A::PLLI2S),
            2 => Val(SAI1BSRC_A::I2S_CKIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLLSAI`"]
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == SAI1BSRC_A::PLLSAI
    }
    #[doc = "Checks if the value of the field is `PLLI2S`"]
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == SAI1BSRC_A::PLLI2S
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1BSRC_A::I2S_CKIN
    }
}
#[doc = "Write proxy for field `SAI1BSRC`"]
pub struct SAI1BSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1BSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1BSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SAI1-B clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::PLLSAI)
    }
    #[doc = "SAI1-B clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::PLLI2S)
    }
    #[doc = "SAI1-B clock frequency = Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1BSRC_A::I2S_CKIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "DFSDM1 Kernel clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDFSDM1SEL_A {
    #[doc = "0: APB2 clock used as Kernel clock"]
    APB2 = 0,
    #[doc = "1: System clock used as Kernel clock"]
    SYSCLK = 1,
}
impl From<CKDFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKDFSDM1SEL`"]
pub type CKDFSDM1SEL_R = crate::R<bool, CKDFSDM1SEL_A>;
impl CKDFSDM1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKDFSDM1SEL_A {
        match self.bits {
            false => CKDFSDM1SEL_A::APB2,
            true => CKDFSDM1SEL_A::SYSCLK,
        }
    }
    #[doc = "Checks if the value of the field is `APB2`"]
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == CKDFSDM1SEL_A::APB2
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKDFSDM1SEL_A::SYSCLK
    }
}
#[doc = "Write proxy for field `CKDFSDM1SEL`"]
pub struct CKDFSDM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDFSDM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKDFSDM1SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "APB2 clock used as Kernel clock"]
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(CKDFSDM1SEL_A::APB2)
    }
    #[doc = "System clock used as Kernel clock"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKDFSDM1SEL_A::SYSCLK)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - DFSDM1 audio clock selection."]
    #[inline(always)]
    pub fn ckdfsdm1asel(&self) -> CKDFSDM1ASEL_R {
        CKDFSDM1ASEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)"]
    #[inline(always)]
    pub fn i2s1src(&self) -> I2S1SRC_R {
        I2S1SRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)"]
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 A/B clock"]
    #[inline(always)]
    pub fn plli2sdivr(&self) -> PLLI2SDIVR_R {
        PLLI2SDIVR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLL division factor for SAI1 A/B clock"]
    #[inline(always)]
    pub fn plldivr(&self) -> PLLDIVR_R {
        PLLDIVR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - DFSDM2 audio clock selection"]
    #[inline(always)]
    pub fn ckdfsdm2asel(&self) -> CKDFSDM2ASEL_R {
        CKDFSDM2ASEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - SAI1-A clock source selection"]
    #[inline(always)]
    pub fn sai1asrc(&self) -> SAI1ASRC_R {
        SAI1ASRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - SAI1-B clock source selection"]
    #[inline(always)]
    pub fn sai1bsrc(&self) -> SAI1BSRC_R {
        SAI1BSRC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 31 - DFSDM1 Kernel clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1sel(&self) -> CKDFSDM1SEL_R {
        CKDFSDM1SEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - DFSDM1 audio clock selection."]
    #[inline(always)]
    pub fn ckdfsdm1asel(&mut self) -> CKDFSDM1ASEL_W {
        CKDFSDM1ASEL_W { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)"]
    #[inline(always)]
    pub fn i2s1src(&mut self) -> I2S1SRC_W {
        I2S1SRC_W { w: self }
    }
    #[doc = "Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)"]
    #[inline(always)]
    pub fn i2s2src(&mut self) -> I2S2SRC_W {
        I2S2SRC_W { w: self }
    }
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 A/B clock"]
    #[inline(always)]
    pub fn plli2sdivr(&mut self) -> PLLI2SDIVR_W {
        PLLI2SDIVR_W { w: self }
    }
    #[doc = "Bits 8:12 - PLL division factor for SAI1 A/B clock"]
    #[inline(always)]
    pub fn plldivr(&mut self) -> PLLDIVR_W {
        PLLDIVR_W { w: self }
    }
    #[doc = "Bit 14 - DFSDM2 audio clock selection"]
    #[inline(always)]
    pub fn ckdfsdm2asel(&mut self) -> CKDFSDM2ASEL_W {
        CKDFSDM2ASEL_W { w: self }
    }
    #[doc = "Bits 20:21 - SAI1-A clock source selection"]
    #[inline(always)]
    pub fn sai1asrc(&mut self) -> SAI1ASRC_W {
        SAI1ASRC_W { w: self }
    }
    #[doc = "Bits 22:23 - SAI1-B clock source selection"]
    #[inline(always)]
    pub fn sai1bsrc(&mut self) -> SAI1BSRC_W {
        SAI1BSRC_W { w: self }
    }
    #[doc = "Bit 31 - DFSDM1 Kernel clock selection"]
    #[inline(always)]
    pub fn ckdfsdm1sel(&mut self) -> CKDFSDM1SEL_W {
        CKDFSDM1SEL_W { w: self }
    }
}
