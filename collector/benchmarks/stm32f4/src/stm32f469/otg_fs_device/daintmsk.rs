#[doc = "Reader of register DAINTMSK"]
pub type R = crate::R<u32, super::DAINTMSK>;
#[doc = "Writer for register DAINTMSK"]
pub type W = crate::W<u32, super::DAINTMSK>;
#[doc = "Register DAINTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DAINTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEPM`"]
pub type IEPM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IEPM`"]
pub struct IEPM_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `OEPINT`"]
pub type OEPINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OEPINT`"]
pub struct OEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W {
        IEPM_W { w: self }
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W {
        OEPINT_W { w: self }
    }
}
