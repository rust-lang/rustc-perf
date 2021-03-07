#[doc = "Reader of register PMC"]
pub type R = crate::R<u32, super::PMC>;
#[doc = "Writer for register PMC"]
pub type W = crate::W<u32, super::PMC>;
#[doc = "Register PMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1DC2`"]
pub type ADC1DC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1DC2`"]
pub struct ADC1DC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1DC2_W<'a> {
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
impl R {
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADC1DC2"]
    #[inline(always)]
    pub fn adc1dc2(&mut self) -> ADC1DC2_W {
        ADC1DC2_W { w: self }
    }
}
