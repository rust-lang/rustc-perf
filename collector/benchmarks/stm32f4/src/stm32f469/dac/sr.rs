#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC channel2 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR2_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel X"]
    NOUNDERRUN = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel X"]
    UNDERRUN = 1,
}
impl From<DMAUDR2_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAUDR2`"]
pub type DMAUDR2_R = crate::R<bool, DMAUDR2_A>;
impl DMAUDR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR2_A {
        match self.bits {
            false => DMAUDR2_A::NOUNDERRUN,
            true => DMAUDR2_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == DMAUDR2_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == DMAUDR2_A::UNDERRUN
    }
}
#[doc = "Write proxy for field `DMAUDR2`"]
pub struct DMAUDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR2_A::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR2_A::UNDERRUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "DAC channel1 DMA underrun flag"]
pub type DMAUDR1_A = DMAUDR2_A;
#[doc = "Reader of field `DMAUDR1`"]
pub type DMAUDR1_R = crate::R<bool, DMAUDR2_A>;
#[doc = "Write proxy for field `DMAUDR1`"]
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAUDR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn no_underrun(self) -> &'a mut W {
        self.variant(DMAUDR2_A::NOUNDERRUN)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel X"]
    #[inline(always)]
    pub fn underrun(self) -> &'a mut W {
        self.variant(DMAUDR2_A::UNDERRUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W {
        DMAUDR2_W { w: self }
    }
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
}
