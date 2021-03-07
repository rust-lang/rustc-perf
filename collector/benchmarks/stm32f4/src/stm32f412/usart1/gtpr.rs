#[doc = "Reader of register GTPR"]
pub type R = crate::R<u32, super::GTPR>;
#[doc = "Writer for register GTPR"]
pub type W = crate::W<u32, super::GTPR>;
#[doc = "Register GTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::GTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GT`"]
pub type GT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GT`"]
pub struct GT_W<'a> {
    w: &'a mut W,
}
impl<'a> GT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W {
        GT_W { w: self }
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
