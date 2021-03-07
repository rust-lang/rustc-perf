#[doc = "Reader of register MACFCR"]
pub type R = crate::R<u32, super::MACFCR>;
#[doc = "Writer for register MACFCR"]
pub type W = crate::W<u32, super::MACFCR>;
#[doc = "Register MACFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCB`"]
pub type FCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCB`"]
pub struct FCB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TFCE`"]
pub type TFCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFCE`"]
pub struct TFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RFCE`"]
pub type RFCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFCE`"]
pub struct RFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UPFD`"]
pub type UPFD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPFD`"]
pub struct UPFD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPFD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PLT`"]
pub type PLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLT`"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ZQPD`"]
pub type ZQPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZQPD`"]
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FCB"]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TFCE"]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RFCE"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UPFD"]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - PLT"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FCB"]
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W {
        FCB_W { w: self }
    }
    #[doc = "Bit 1 - TFCE"]
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W {
        TFCE_W { w: self }
    }
    #[doc = "Bit 2 - RFCE"]
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W {
        RFCE_W { w: self }
    }
    #[doc = "Bit 3 - UPFD"]
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W {
        UPFD_W { w: self }
    }
    #[doc = "Bits 4:5 - PLT"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    #[doc = "Bit 7 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W {
        ZQPD_W { w: self }
    }
    #[doc = "Bits 16:31 - PT"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
}
