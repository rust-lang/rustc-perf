#[doc = "Writer for register HIFCR"]
pub type W = crate::W<u32, super::HIFCR>;
#[doc = "Register HIFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HIFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF7_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    CLEAR = 1,
}
impl From<CTCIF7_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTCIF7`"]
pub struct CTCIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear half transfer interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF7_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    CLEAR = 1,
}
impl From<CHTIF7_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CHTIF7`"]
pub struct CHTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear transfer error interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF7_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    CLEAR = 1,
}
impl From<CTEIF7_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTEIF7`"]
pub struct CTEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear direct mode error interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDMEIF7_AW {
    #[doc = "1: Clear the corresponding DMEIFx flag"]
    CLEAR = 1,
}
impl From<CDMEIF7_AW> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CDMEIF7`"]
pub struct CDMEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear FIFO error interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEIF7_AW {
    #[doc = "1: Clear the corresponding CFEIFx flag"]
    CLEAR = 1,
}
impl From<CFEIF7_AW> for bool {
    #[inline(always)]
    fn from(variant: CFEIF7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CFEIF7`"]
pub struct CFEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF6_AW = CTCIF7_AW;
#[doc = "Write proxy for field `CTCIF6`"]
pub struct CTCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF6_AW = CHTIF7_AW;
#[doc = "Write proxy for field `CHTIF6`"]
pub struct CHTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF6_AW = CTEIF7_AW;
#[doc = "Write proxy for field `CTEIF6`"]
pub struct CTEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF6_AW = CDMEIF7_AW;
#[doc = "Write proxy for field `CDMEIF6`"]
pub struct CDMEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF6_AW = CFEIF7_AW;
#[doc = "Write proxy for field `CFEIF6`"]
pub struct CFEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF5_AW = CTCIF7_AW;
#[doc = "Write proxy for field `CTCIF5`"]
pub struct CTCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF5_AW = CHTIF7_AW;
#[doc = "Write proxy for field `CHTIF5`"]
pub struct CHTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF5_AW = CTEIF7_AW;
#[doc = "Write proxy for field `CTEIF5`"]
pub struct CTEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF5_AW = CDMEIF7_AW;
#[doc = "Write proxy for field `CDMEIF5`"]
pub struct CDMEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF5_AW = CFEIF7_AW;
#[doc = "Write proxy for field `CFEIF5`"]
pub struct CFEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF4_AW = CTCIF7_AW;
#[doc = "Write proxy for field `CTCIF4`"]
pub struct CTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF4_AW = CHTIF7_AW;
#[doc = "Write proxy for field `CHTIF4`"]
pub struct CHTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF4_AW = CTEIF7_AW;
#[doc = "Write proxy for field `CTEIF4`"]
pub struct CTEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF4_AW = CDMEIF7_AW;
#[doc = "Write proxy for field `CDMEIF4`"]
pub struct CDMEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF7_AW::CLEAR)
    }
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
#[doc = "Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF4_AW = CFEIF7_AW;
#[doc = "Write proxy for field `CFEIF4`"]
pub struct CFEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF7_AW::CLEAR)
    }
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
impl W {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W {
        CTCIF7_W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W {
        CHTIF7_W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W {
        CTEIF7_W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W {
        CDMEIF7_W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif7(&mut self) -> CFEIF7_W {
        CFEIF7_W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W {
        CTCIF6_W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W {
        CHTIF6_W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W {
        CTEIF6_W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W {
        CDMEIF6_W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif6(&mut self) -> CFEIF6_W {
        CFEIF6_W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W {
        CTCIF5_W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W {
        CHTIF5_W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W {
        CTEIF5_W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W {
        CDMEIF5_W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif5(&mut self) -> CFEIF5_W {
        CFEIF5_W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W {
        CTCIF4_W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W {
        CHTIF4_W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W {
        CDMEIF4_W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif4(&mut self) -> CFEIF4_W {
        CFEIF4_W { w: self }
    }
}
