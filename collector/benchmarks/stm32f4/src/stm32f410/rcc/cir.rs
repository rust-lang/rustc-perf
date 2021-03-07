#[doc = "Reader of register CIR"]
pub type R = crate::R<u32, super::CIR>;
#[doc = "Writer for register CIR"]
pub type W = crate::W<u32, super::CIR>;
#[doc = "Register CIR `reset()`'s with value 0"]
impl crate::ResetValue for super::CIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSC_AW {
    #[doc = "1: Clear CSSF flag"]
    CLEAR = 1,
}
impl From<CSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CSSC`"]
pub struct CSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear CSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSC_AW::CLEAR)
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
#[doc = "PLLI2S ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLI2SRDYC_AW {
    #[doc = "1: Clear interrupt flag"]
    CLEAR = 1,
}
impl From<PLLI2SRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLI2SRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PLLI2SRDYC`"]
pub struct PLLI2SRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SRDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLI2SRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Main PLL(PLL) ready interrupt clear"]
pub type PLLRDYC_AW = PLLI2SRDYC_AW;
#[doc = "Write proxy for field `PLLRDYC`"]
pub struct PLLRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLRDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLI2SRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "HSE ready interrupt clear"]
pub type HSERDYC_AW = PLLI2SRDYC_AW;
#[doc = "Write proxy for field `HSERDYC`"]
pub struct HSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSERDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLI2SRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "HSI ready interrupt clear"]
pub type HSIRDYC_AW = PLLI2SRDYC_AW;
#[doc = "Write proxy for field `HSIRDYC`"]
pub struct HSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIRDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLI2SRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "LSE ready interrupt clear"]
pub type LSERDYC_AW = PLLI2SRDYC_AW;
#[doc = "Write proxy for field `LSERDYC`"]
pub struct LSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSERDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLI2SRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "LSI ready interrupt clear"]
pub type LSIRDYC_AW = PLLI2SRDYC_AW;
#[doc = "Write proxy for field `LSIRDYC`"]
pub struct LSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSIRDYC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLI2SRDYC_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Main PLL (PLL) ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled"]
    ENABLED = 1,
}
impl From<PLLRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLRDYIE`"]
pub type PLLRDYIE_R = crate::R<bool, PLLRDYIE_A>;
impl PLLRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYIE_A {
        match self.bits {
            false => PLLRDYIE_A::DISABLED,
            true => PLLRDYIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLRDYIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLRDYIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `PLLRDYIE`"]
pub struct PLLRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "HSE ready interrupt enable"]
pub type HSERDYIE_A = PLLRDYIE_A;
#[doc = "Reader of field `HSERDYIE`"]
pub type HSERDYIE_R = crate::R<bool, PLLRDYIE_A>;
#[doc = "Write proxy for field `HSERDYIE`"]
pub struct HSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSERDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "HSI ready interrupt enable"]
pub type HSIRDYIE_A = PLLRDYIE_A;
#[doc = "Reader of field `HSIRDYIE`"]
pub type HSIRDYIE_R = crate::R<bool, PLLRDYIE_A>;
#[doc = "Write proxy for field `HSIRDYIE`"]
pub struct HSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "LSE ready interrupt enable"]
pub type LSERDYIE_A = PLLRDYIE_A;
#[doc = "Reader of field `LSERDYIE`"]
pub type LSERDYIE_R = crate::R<bool, PLLRDYIE_A>;
#[doc = "Write proxy for field `LSERDYIE`"]
pub struct LSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSERDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "LSI ready interrupt enable"]
pub type LSIRDYIE_A = PLLRDYIE_A;
#[doc = "Reader of field `LSIRDYIE`"]
pub type LSIRDYIE_R = crate::R<bool, PLLRDYIE_A>;
#[doc = "Write proxy for field `LSIRDYIE`"]
pub struct LSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSIRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::DISABLED)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSF_A {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NOTINTERRUPTED = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    INTERRUPTED = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSSF`"]
pub type CSSF_R = crate::R<bool, CSSF_A>;
impl CSSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::NOTINTERRUPTED,
            true => CSSF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSF_A::INTERRUPTED
    }
}
#[doc = "Main PLL (PLL) ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDYF_A {
    #[doc = "0: No clock ready interrupt"]
    NOTINTERRUPTED = 0,
    #[doc = "1: Clock ready interrupt"]
    INTERRUPTED = 1,
}
impl From<PLLRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLRDYF`"]
pub type PLLRDYF_R = crate::R<bool, PLLRDYF_A>;
impl PLLRDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYF_A {
        match self.bits {
            false => PLLRDYF_A::NOTINTERRUPTED,
            true => PLLRDYF_A::INTERRUPTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINTERRUPTED`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == PLLRDYF_A::NOTINTERRUPTED
    }
    #[doc = "Checks if the value of the field is `INTERRUPTED`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == PLLRDYF_A::INTERRUPTED
    }
}
#[doc = "HSE ready interrupt flag"]
pub type HSERDYF_A = PLLRDYF_A;
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, PLLRDYF_A>;
#[doc = "HSI ready interrupt flag"]
pub type HSIRDYF_A = PLLRDYF_A;
#[doc = "Reader of field `HSIRDYF`"]
pub type HSIRDYF_R = crate::R<bool, PLLRDYF_A>;
#[doc = "LSE ready interrupt flag"]
pub type LSERDYF_A = PLLRDYF_A;
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, PLLRDYF_A>;
#[doc = "LSI ready interrupt flag"]
pub type LSIRDYF_A = PLLRDYF_A;
#[doc = "Reader of field `LSIRDYF`"]
pub type LSIRDYF_R = crate::R<bool, PLLRDYF_A>;
impl R {
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Main PLL (PLL) ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W {
        CSSC_W { w: self }
    }
    #[doc = "Bit 21 - PLLI2S ready interrupt clear"]
    #[inline(always)]
    pub fn plli2srdyc(&mut self) -> PLLI2SRDYC_W {
        PLLI2SRDYC_W { w: self }
    }
    #[doc = "Bit 20 - Main PLL(PLL) ready interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W {
        PLLRDYC_W { w: self }
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W {
        HSERDYC_W { w: self }
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W {
        HSIRDYC_W { w: self }
    }
    #[doc = "Bit 17 - LSE ready interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W {
        LSERDYC_W { w: self }
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W {
        LSIRDYC_W { w: self }
    }
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W {
        PLLRDYIE_W { w: self }
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
}
