#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDRM`"]
pub type DDRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRM`"]
pub struct DDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `DHHC`"]
pub type DHHC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DHHC`"]
pub struct DHHC_W<'a> {
    w: &'a mut W,
}
impl<'a> DHHC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SIOO`"]
pub type SIOO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIOO`"]
pub struct SIOO_W<'a> {
    w: &'a mut W,
}
impl<'a> SIOO_W<'a> {
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
#[doc = "Reader of field `FMODE`"]
pub type FMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FMODE`"]
pub struct FMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DMODE`"]
pub type DMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMODE`"]
pub struct DMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DCYC`"]
pub type DCYC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DCYC`"]
pub struct DCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ABSIZE`"]
pub type ABSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ABSIZE`"]
pub struct ABSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ABMODE`"]
pub type ABMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ABMODE`"]
pub struct ABMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `ADSIZE`"]
pub type ADSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSIZE`"]
pub struct ADSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ADMODE`"]
pub type ADMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADMODE`"]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `IMODE`"]
pub type IMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IMODE`"]
pub struct IMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSTRUCTION`"]
pub type INSTRUCTION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTRUCTION`"]
pub struct INSTRUCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRUCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Double data rate mode"]
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DDR hold half cycle"]
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Send instruction only once mode"]
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - Functional mode"]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Data mode"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Alternate bytes size"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Alternate bytes mode"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Address mode"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Instruction mode"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7 - Instruction"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Double data rate mode"]
    #[inline(always)]
    pub fn ddrm(&mut self) -> DDRM_W {
        DDRM_W { w: self }
    }
    #[doc = "Bit 30 - DDR hold half cycle"]
    #[inline(always)]
    pub fn dhhc(&mut self) -> DHHC_W {
        DHHC_W { w: self }
    }
    #[doc = "Bit 28 - Send instruction only once mode"]
    #[inline(always)]
    pub fn sioo(&mut self) -> SIOO_W {
        SIOO_W { w: self }
    }
    #[doc = "Bits 26:27 - Functional mode"]
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W {
        FMODE_W { w: self }
    }
    #[doc = "Bits 24:25 - Data mode"]
    #[inline(always)]
    pub fn dmode(&mut self) -> DMODE_W {
        DMODE_W { w: self }
    }
    #[doc = "Bits 18:22 - Number of dummy cycles"]
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W {
        DCYC_W { w: self }
    }
    #[doc = "Bits 16:17 - Alternate bytes size"]
    #[inline(always)]
    pub fn absize(&mut self) -> ABSIZE_W {
        ABSIZE_W { w: self }
    }
    #[doc = "Bits 14:15 - Alternate bytes mode"]
    #[inline(always)]
    pub fn abmode(&mut self) -> ABMODE_W {
        ABMODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline(always)]
    pub fn adsize(&mut self) -> ADSIZE_W {
        ADSIZE_W { w: self }
    }
    #[doc = "Bits 10:11 - Address mode"]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Instruction mode"]
    #[inline(always)]
    pub fn imode(&mut self) -> IMODE_W {
        IMODE_W { w: self }
    }
    #[doc = "Bits 0:7 - Instruction"]
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W {
        INSTRUCTION_W { w: self }
    }
}
