#[doc = "Reader of register DMAMFBOCR"]
pub type R = crate::R<u32, super::DMAMFBOCR>;
#[doc = "Writer for register DMAMFBOCR"]
pub type W = crate::W<u32, super::DMAMFBOCR>;
#[doc = "Register DMAMFBOCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAMFBOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFC`"]
pub type MFC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MFC`"]
pub struct MFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `OMFC`"]
pub type OMFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OMFC`"]
pub struct OMFC_W<'a> {
    w: &'a mut W,
}
impl<'a> OMFC_W<'a> {
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
#[doc = "Reader of field `MFA`"]
pub type MFA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MFA`"]
pub struct MFA_W<'a> {
    w: &'a mut W,
}
impl<'a> MFA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 17)) | (((value as u32) & 0x07ff) << 17);
        self.w
    }
}
#[doc = "Reader of field `OFOC`"]
pub type OFOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFOC`"]
pub struct OFOC_W<'a> {
    w: &'a mut W,
}
impl<'a> OFOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MFC"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - OMFC"]
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:27 - MFA"]
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - OFOC"]
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MFC"]
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W {
        MFC_W { w: self }
    }
    #[doc = "Bit 16 - OMFC"]
    #[inline(always)]
    pub fn omfc(&mut self) -> OMFC_W {
        OMFC_W { w: self }
    }
    #[doc = "Bits 17:27 - MFA"]
    #[inline(always)]
    pub fn mfa(&mut self) -> MFA_W {
        MFA_W { w: self }
    }
    #[doc = "Bit 28 - OFOC"]
    #[inline(always)]
    pub fn ofoc(&mut self) -> OFOC_W {
        OFOC_W { w: self }
    }
}
