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
#[doc = "Flexible static memory controller module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMCLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP = 1,
}
impl From<FSMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSMCLPEN`"]
pub type FSMCLPEN_R = crate::R<bool, FSMCLPEN_A>;
impl FSMCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSMCLPEN_A {
        match self.bits {
            false => FSMCLPEN_A::DISABLEDINSLEEP,
            true => FSMCLPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN_A::ENABLEDINSLEEP
    }
}
#[doc = "Write proxy for field `FSMCLPEN`"]
pub struct FSMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSMCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::ENABLEDINSLEEP)
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
    #[doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W {
        FSMCLPEN_W { w: self }
    }
}
