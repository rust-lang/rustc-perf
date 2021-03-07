#[doc = "Reader of register BRR"]
pub type R = crate::R<u32, super::BRR>;
#[doc = "Writer for register BRR"]
pub type W = crate::W<u32, super::BRR>;
#[doc = "Register BRR `reset()`'s with value 0"]
impl crate::ResetValue for super::BRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIV_Mantissa`"]
pub type DIV_MANTISSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIV_Mantissa`"]
pub struct DIV_MANTISSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_MANTISSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIV_Fraction`"]
pub type DIV_FRACTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIV_Fraction`"]
pub struct DIV_FRACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_FRACTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn div_mantissa(&self) -> DIV_MANTISSA_R {
        DIV_MANTISSA_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DIV_FRACTION_R {
        DIV_FRACTION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn div_mantissa(&mut self) -> DIV_MANTISSA_W {
        DIV_MANTISSA_W { w: self }
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W {
        DIV_FRACTION_W { w: self }
    }
}
