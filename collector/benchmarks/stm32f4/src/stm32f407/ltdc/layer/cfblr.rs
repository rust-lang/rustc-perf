#[doc = "Reader of register CFBLR"]
pub type R = crate::R<u32, super::CFBLR>;
#[doc = "Writer for register CFBLR"]
pub type W = crate::W<u32, super::CFBLR>;
#[doc = "Register CFBLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFBLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFBP`"]
pub type CFBP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFBP`"]
pub struct CFBP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | (((value as u32) & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CFBLL`"]
pub type CFBLL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFBLL`"]
pub struct CFBLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:12 - Color Frame Buffer Line Length"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:28 - Color Frame Buffer Pitch in bytes"]
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W {
        CFBP_W { w: self }
    }
    #[doc = "Bits 0:12 - Color Frame Buffer Line Length"]
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W {
        CFBLL_W { w: self }
    }
}
