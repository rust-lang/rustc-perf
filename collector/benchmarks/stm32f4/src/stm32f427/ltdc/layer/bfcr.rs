#[doc = "Reader of register BFCR"]
pub type R = crate::R<u32, super::BFCR>;
#[doc = "Writer for register BFCR"]
pub type W = crate::W<u32, super::BFCR>;
#[doc = "Register BFCR `reset()`'s with value 0x0607"]
impl crate::ResetValue for super::BFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0607
    }
}
#[doc = "Reader of field `BF1`"]
pub type BF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BF1`"]
pub struct BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `BF2`"]
pub type BF2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BF2`"]
pub struct BF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W {
        BF1_W { w: self }
    }
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W {
        BF2_W { w: self }
    }
}
