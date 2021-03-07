#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OFFSET`"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DTRBS`"]
pub type DTRBS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTRBS`"]
pub struct DTRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W {
        DTRBS_W { w: self }
    }
}
