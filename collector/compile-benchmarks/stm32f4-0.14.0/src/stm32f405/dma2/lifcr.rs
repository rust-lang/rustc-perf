#[doc = "Register `LIFCR` writer"]
pub struct W(crate::W<LIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF3_AW = CTCIF0_AW;
#[doc = "Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF3_AW = CHTIF0_AW;
#[doc = "Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF3_AW = CTEIF0_AW;
#[doc = "Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF3_AW = CDMEIF0_AW;
#[doc = "Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF3_AW = CFEIF0_AW;
#[doc = "Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF2_AW = CTCIF0_AW;
#[doc = "Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF2_AW = CHTIF0_AW;
#[doc = "Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF2_AW = CTEIF0_AW;
#[doc = "Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF2_AW = CDMEIF0_AW;
#[doc = "Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF2_AW = CFEIF0_AW;
#[doc = "Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF1_AW = CTCIF0_AW;
#[doc = "Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF1_AW = CHTIF0_AW;
#[doc = "Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF1_AW = CTEIF0_AW;
#[doc = "Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF1_AW = CDMEIF0_AW;
#[doc = "Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF1_AW = CFEIF0_AW;
#[doc = "Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF1_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF0_AW {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    CLEAR = 1,
}
impl From<CTCIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF0_AW {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    CLEAR = 1,
}
impl From<CHTIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHTIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF0_AW {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    CLEAR = 1,
}
impl From<CTEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDMEIF0_AW {
    #[doc = "1: Clear the corresponding DMEIFx flag"]
    CLEAR = 1,
}
impl From<CDMEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDMEIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CDMEIF0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEIF0_AW {
    #[doc = "1: Clear the corresponding CFEIFx flag"]
    CLEAR = 1,
}
impl From<CFEIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: CFEIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFEIF0_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W {
        CTCIF3_W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W {
        CHTIF3_W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W {
        CDMEIF3_W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W {
        CFEIF3_W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W {
        CTCIF2_W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W {
        CHTIF2_W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W {
        CDMEIF2_W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W {
        CFEIF2_W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W {
        CTCIF1_W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W {
        CHTIF1_W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W {
        CDMEIF1_W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W {
        CFEIF1_W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W {
        CTCIF0_W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W {
        CHTIF0_W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W {
        CTEIF0_W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W {
        CDMEIF0_W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W {
        CFEIF0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "low interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lifcr](index.html) module"]
pub struct LIFCR_SPEC;
impl crate::RegisterSpec for LIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lifcr::W](W) writer structure"]
impl crate::Writable for LIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIFCR to value 0"]
impl crate::Resettable for LIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
