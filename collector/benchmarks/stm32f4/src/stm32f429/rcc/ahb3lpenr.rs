#[doc = "Reader of register AHB3LPENR"]
pub type R = crate::R<u32, super::AHB3LPENR>;
#[doc = "Writer for register AHB3LPENR"]
pub type W = crate::W<u32, super::AHB3LPENR>;
#[doc = "Register AHB3LPENR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AHB3LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Flexible memory controller module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP = 1,
}
impl From<FMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMCLPEN`"]
pub type FMCLPEN_R = crate::R<bool, FMCLPEN_A>;
impl FMCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCLPEN_A {
        match self.bits {
            false => FMCLPEN_A::DISABLEDINSLEEP,
            true => FMCLPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FMCLPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FMCLPEN_A::ENABLEDINSLEEP
    }
}
#[doc = "Write proxy for field `FMCLPEN`"]
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
}
