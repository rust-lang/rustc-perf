#[doc = "Reader of register AHB2LPENR"]
pub type R = crate::R<u32, super::AHB2LPENR>;
#[doc = "Writer for register AHB2LPENR"]
pub type W = crate::W<u32, super::AHB2LPENR>;
#[doc = "Register AHB2LPENR `reset()`'s with value 0xf1"]
impl crate::ResetValue for super::AHB2LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf1
    }
}
#[doc = "USB OTG FS clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP = 1,
}
impl From<OTGFSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OTGFSLPEN`"]
pub type OTGFSLPEN_R = crate::R<bool, OTGFSLPEN_A>;
impl OTGFSLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGFSLPEN_A {
        match self.bits {
            false => OTGFSLPEN_A::DISABLEDINSLEEP,
            true => OTGFSLPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == OTGFSLPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == OTGFSLPEN_A::ENABLEDINSLEEP
    }
}
#[doc = "Write proxy for field `OTGFSLPEN`"]
pub struct OTGFSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTGFSLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(OTGFSLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W {
        OTGFSLPEN_W { w: self }
    }
}
