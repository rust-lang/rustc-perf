#[doc = "Reader of register DSI_WPCR1"]
pub type R = crate::R<u32, super::DSI_WPCR1>;
#[doc = "Writer for register DSI_WPCR1"]
pub type W = crate::W<u32, super::DSI_WPCR1>;
#[doc = "Register DSI_WPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WPCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCLKPOSTEN`"]
pub type TCLKPOSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCLKPOSTEN`"]
pub struct TCLKPOSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPOSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `TLPXCEN`"]
pub type TLPXCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TLPXCEN`"]
pub struct TLPXCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `THSEXITEN`"]
pub type THSEXITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THSEXITEN`"]
pub struct THSEXITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSEXITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `TLPXDEN`"]
pub type TLPXDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TLPXDEN`"]
pub struct TLPXDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `THSZEROEN`"]
pub type THSZEROEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THSZEROEN`"]
pub struct THSZEROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSZEROEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `THSTRAILEN`"]
pub type THSTRAILEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THSTRAILEN`"]
pub struct THSTRAILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSTRAILEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `THSPREPEN`"]
pub type THSPREPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THSPREPEN`"]
pub struct THSPREPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THSPREPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TCLKZEROEN`"]
pub type TCLKZEROEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCLKZEROEN`"]
pub struct TCLKZEROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKZEROEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TCLKPREPEN`"]
pub type TCLKPREPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCLKPREPEN`"]
pub struct TCLKPREPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPREPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PDEN`"]
pub type PDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN`"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TDDL`"]
pub type TDDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDDL`"]
pub struct TDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDDL_W<'a> {
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
#[doc = "Reader of field `CDOFFDL`"]
pub type CDOFFDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDOFFDL`"]
pub struct CDOFFDL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFFDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FTXSMDL`"]
pub type FTXSMDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTXSMDL`"]
pub struct FTXSMDL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FTXSMCL`"]
pub type FTXSMCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTXSMCL`"]
pub struct FTXSMCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `HSIDL1`"]
pub type HSIDL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIDL1`"]
pub struct HSIDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `HSIDL0`"]
pub type HSIDL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIDL0`"]
pub struct HSIDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `HSICL`"]
pub type HSICL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSICL`"]
pub struct HSICL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SWDL1`"]
pub type SWDL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWDL1`"]
pub struct SWDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SWDL0`"]
pub type SWDL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWDL0`"]
pub struct SWDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL0_W<'a> {
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
#[doc = "Reader of field `SWCL`"]
pub type SWCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWCL`"]
pub struct SWCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UIX4`"]
pub type UIX4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UIX4`"]
pub struct UIX4_W<'a> {
    w: &'a mut W,
}
impl<'a> UIX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - custom time for tCLK-POST Enable"]
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - custom time for tLPX for Clock lane Enable"]
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - custom time for tHS-EXIT Enable"]
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - custom time for tLPX for Data lanes Enable"]
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - custom time for tHS-ZERO Enable"]
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - custom time for tHS-TRAIL Enable"]
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - custom time for tHS-PREPARE Enable"]
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - custom time for tCLK-ZERO Enable"]
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - custom time for tCLK-PREPARE Enable"]
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Turn Disable Data Lanes"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Contention Detection OFF on Data Lanes"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force in TX Stop Mode the Data Lanes"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force in TX Stop Mode the Clock Lane"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Invert the High-Speed data signal on Data Lane 1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invert the Hight-Speed data signal on Data Lane 0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Invert Hight-Speed data signal on Clock Lane"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Swap Data Lane 1 pins"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Swap Data Lane 0 pins"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Swap Clock Lane pins"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Unit Interval multiplied by 4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 27 - custom time for tCLK-POST Enable"]
    #[inline(always)]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W {
        TCLKPOSTEN_W { w: self }
    }
    #[doc = "Bit 26 - custom time for tLPX for Clock lane Enable"]
    #[inline(always)]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W {
        TLPXCEN_W { w: self }
    }
    #[doc = "Bit 25 - custom time for tHS-EXIT Enable"]
    #[inline(always)]
    pub fn thsexiten(&mut self) -> THSEXITEN_W {
        THSEXITEN_W { w: self }
    }
    #[doc = "Bit 24 - custom time for tLPX for Data lanes Enable"]
    #[inline(always)]
    pub fn tlpxden(&mut self) -> TLPXDEN_W {
        TLPXDEN_W { w: self }
    }
    #[doc = "Bit 23 - custom time for tHS-ZERO Enable"]
    #[inline(always)]
    pub fn thszeroen(&mut self) -> THSZEROEN_W {
        THSZEROEN_W { w: self }
    }
    #[doc = "Bit 22 - custom time for tHS-TRAIL Enable"]
    #[inline(always)]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W {
        THSTRAILEN_W { w: self }
    }
    #[doc = "Bit 21 - custom time for tHS-PREPARE Enable"]
    #[inline(always)]
    pub fn thsprepen(&mut self) -> THSPREPEN_W {
        THSPREPEN_W { w: self }
    }
    #[doc = "Bit 20 - custom time for tCLK-ZERO Enable"]
    #[inline(always)]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W {
        TCLKZEROEN_W { w: self }
    }
    #[doc = "Bit 19 - custom time for tCLK-PREPARE Enable"]
    #[inline(always)]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W {
        TCLKPREPEN_W { w: self }
    }
    #[doc = "Bit 18 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 16 - Turn Disable Data Lanes"]
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W {
        TDDL_W { w: self }
    }
    #[doc = "Bit 14 - Contention Detection OFF on Data Lanes"]
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W {
        CDOFFDL_W { w: self }
    }
    #[doc = "Bit 13 - Force in TX Stop Mode the Data Lanes"]
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W {
        FTXSMDL_W { w: self }
    }
    #[doc = "Bit 12 - Force in TX Stop Mode the Clock Lane"]
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W {
        FTXSMCL_W { w: self }
    }
    #[doc = "Bit 11 - Invert the High-Speed data signal on Data Lane 1"]
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W {
        HSIDL1_W { w: self }
    }
    #[doc = "Bit 10 - Invert the Hight-Speed data signal on Data Lane 0"]
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W {
        HSIDL0_W { w: self }
    }
    #[doc = "Bit 9 - Invert Hight-Speed data signal on Clock Lane"]
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W {
        HSICL_W { w: self }
    }
    #[doc = "Bit 8 - Swap Data Lane 1 pins"]
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W {
        SWDL1_W { w: self }
    }
    #[doc = "Bit 7 - Swap Data Lane 0 pins"]
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W {
        SWDL0_W { w: self }
    }
    #[doc = "Bit 6 - Swap Clock Lane pins"]
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W {
        SWCL_W { w: self }
    }
    #[doc = "Bits 0:5 - Unit Interval multiplied by 4"]
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W {
        UIX4_W { w: self }
    }
}
