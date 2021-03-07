#[doc = "Reader of register ESUR"]
pub type R = crate::R<u32, super::ESUR>;
#[doc = "Writer for register ESUR"]
pub type W = crate::W<u32, super::ESUR>;
#[doc = "Register ESUR `reset()`'s with value 0"]
impl crate::ResetValue for super::ESUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEU`"]
pub type FEU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FEU`"]
pub struct FEU_W<'a> {
    w: &'a mut W,
}
impl<'a> FEU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `LEU`"]
pub type LEU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEU`"]
pub struct LEU_W<'a> {
    w: &'a mut W,
}
impl<'a> LEU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LSU`"]
pub type LSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSU`"]
pub struct LSU_W<'a> {
    w: &'a mut W,
}
impl<'a> LSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FSU`"]
pub type FSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSU`"]
pub struct FSU_W<'a> {
    w: &'a mut W,
}
impl<'a> FSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&self) -> FEU_R {
        FEU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&self) -> LEU_R {
        LEU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&self) -> LSU_R {
        LSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&self) -> FSU_R {
        FSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Frame end delimiter unmask"]
    #[inline(always)]
    pub fn feu(&mut self) -> FEU_W {
        FEU_W { w: self }
    }
    #[doc = "Bits 16:23 - Line end delimiter unmask"]
    #[inline(always)]
    pub fn leu(&mut self) -> LEU_W {
        LEU_W { w: self }
    }
    #[doc = "Bits 8:15 - Line start delimiter unmask"]
    #[inline(always)]
    pub fn lsu(&mut self) -> LSU_W {
        LSU_W { w: self }
    }
    #[doc = "Bits 0:7 - Frame start delimiter unmask"]
    #[inline(always)]
    pub fn fsu(&mut self) -> FSU_W {
        FSU_W { w: self }
    }
}
