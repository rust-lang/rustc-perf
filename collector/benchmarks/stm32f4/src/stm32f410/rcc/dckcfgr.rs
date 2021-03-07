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
#[doc = "TIMPRE\n\nValue on reset: 0"]
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
#[doc = "I2SSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SSRC_A {
    #[doc = "0: I2Sx clock frequency = f(PLLCLK_R)"]
    PLLCLKR = 0,
    #[doc = "1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    I2S_CKIN = 1,
    #[doc = "3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    HSI_HSE = 3,
}
impl From<I2SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2SSRC`"]
pub type I2SSRC_R = crate::R<u8, I2SSRC_A>;
impl I2SSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2SSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2SSRC_A::PLLCLKR),
            1 => Val(I2SSRC_A::I2S_CKIN),
            3 => Val(I2SSRC_A::HSI_HSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLLCLKR`"]
    #[inline(always)]
    pub fn is_pllclkr(&self) -> bool {
        *self == I2SSRC_A::PLLCLKR
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2SSRC_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `HSI_HSE`"]
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2SSRC_A::HSI_HSE
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "I2Sx clock frequency = f(PLLCLK_R)"]
    #[inline(always)]
    pub fn pllclkr(self) -> &'a mut W {
        self.variant(I2SSRC_A::PLLCLKR)
    }
    #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2SSRC_A::I2S_CKIN)
    }
    #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2SSRC_A::HSI_HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - I2SSRC"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - TIMPRE"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Bits 25:26 - I2SSRC"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W {
        I2SSRC_W { w: self }
    }
}
