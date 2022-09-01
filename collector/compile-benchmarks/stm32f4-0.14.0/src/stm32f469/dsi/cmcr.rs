#[doc = "Register `CMCR` reader"]
pub struct R(crate::R<CMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMCR` writer"]
pub struct W(crate::W<CMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMCR_SPEC>;
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
impl From<crate::W<CMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRDPS` reader - Maximum Read Packet Size"]
pub struct MRDPS_R(crate::FieldReader<bool, bool>);
impl MRDPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRDPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRDPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRDPS` writer - Maximum Read Packet Size"]
pub struct MRDPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MRDPS_W<'a> {
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
#[doc = "Field `DLWTX` reader - DCS Long Write Transmission"]
pub struct DLWTX_R(crate::FieldReader<bool, bool>);
impl DLWTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLWTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLWTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLWTX` writer - DCS Long Write Transmission"]
pub struct DLWTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DLWTX_W<'a> {
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
#[doc = "Field `DSR0TX` reader - DCS Short Read Zero parameter Transmission"]
pub struct DSR0TX_R(crate::FieldReader<bool, bool>);
impl DSR0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR0TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR0TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR0TX` writer - DCS Short Read Zero parameter Transmission"]
pub struct DSR0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR0TX_W<'a> {
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
#[doc = "Field `DSW1TX` reader - DCS Short Read One parameter Transmission"]
pub struct DSW1TX_R(crate::FieldReader<bool, bool>);
impl DSW1TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSW1TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSW1TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSW1TX` writer - DCS Short Read One parameter Transmission"]
pub struct DSW1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSW1TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DSW0TX` reader - DCS Short Write Zero parameter Transmission"]
pub struct DSW0TX_R(crate::FieldReader<bool, bool>);
impl DSW0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSW0TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSW0TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSW0TX` writer - DCS Short Write Zero parameter Transmission"]
pub struct DSW0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSW0TX_W<'a> {
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
#[doc = "Field `GLWTX` reader - Generic Long Write Transmission"]
pub struct GLWTX_R(crate::FieldReader<bool, bool>);
impl GLWTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLWTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLWTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLWTX` writer - Generic Long Write Transmission"]
pub struct GLWTX_W<'a> {
    w: &'a mut W,
}
impl<'a> GLWTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `GSR2TX` reader - Generic Short Read Two parameters Transmission"]
pub struct GSR2TX_R(crate::FieldReader<bool, bool>);
impl GSR2TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSR2TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSR2TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSR2TX` writer - Generic Short Read Two parameters Transmission"]
pub struct GSR2TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSR2TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `GSR1TX` reader - Generic Short Read One parameters Transmission"]
pub struct GSR1TX_R(crate::FieldReader<bool, bool>);
impl GSR1TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSR1TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSR1TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSR1TX` writer - Generic Short Read One parameters Transmission"]
pub struct GSR1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSR1TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `GSR0TX` reader - Generic Short Read Zero parameters Transmission"]
pub struct GSR0TX_R(crate::FieldReader<bool, bool>);
impl GSR0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSR0TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSR0TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSR0TX` writer - Generic Short Read Zero parameters Transmission"]
pub struct GSR0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSR0TX_W<'a> {
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
#[doc = "Field `GSW2TX` reader - Generic Short Write Two parameters Transmission"]
pub struct GSW2TX_R(crate::FieldReader<bool, bool>);
impl GSW2TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSW2TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSW2TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSW2TX` writer - Generic Short Write Two parameters Transmission"]
pub struct GSW2TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSW2TX_W<'a> {
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
#[doc = "Field `GSW1TX` reader - Generic Short Write One parameters Transmission"]
pub struct GSW1TX_R(crate::FieldReader<bool, bool>);
impl GSW1TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSW1TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSW1TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSW1TX` writer - Generic Short Write One parameters Transmission"]
pub struct GSW1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSW1TX_W<'a> {
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
#[doc = "Field `GSW0TX` reader - Generic Short Write Zero parameters Transmission"]
pub struct GSW0TX_R(crate::FieldReader<bool, bool>);
impl GSW0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSW0TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSW0TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSW0TX` writer - Generic Short Write Zero parameters Transmission"]
pub struct GSW0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> GSW0TX_W<'a> {
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
#[doc = "Field `ARE` reader - Acknowledge Request Enable"]
pub struct ARE_R(crate::FieldReader<bool, bool>);
impl ARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARE` writer - Acknowledge Request Enable"]
pub struct ARE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TEARE` reader - Tearing Effect Acknowledge Request Enable"]
pub struct TEARE_R(crate::FieldReader<bool, bool>);
impl TEARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEARE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEARE` writer - Tearing Effect Acknowledge Request Enable"]
pub struct TEARE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEARE_W<'a> {
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
impl R {
    #[doc = "Bit 24 - Maximum Read Packet Size"]
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DCS Long Write Transmission"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DCS Short Read Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DCS Short Read One parameter Transmission"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DCS Short Write Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Generic Long Write Transmission"]
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Generic Short Read Two parameters Transmission"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Generic Short Read One parameters Transmission"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic Short Read Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Short Write Two parameters Transmission"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Short Write One parameters Transmission"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Short Write Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Maximum Read Packet Size"]
    #[inline(always)]
    pub fn mrdps(&mut self) -> MRDPS_W {
        MRDPS_W { w: self }
    }
    #[doc = "Bit 19 - DCS Long Write Transmission"]
    #[inline(always)]
    pub fn dlwtx(&mut self) -> DLWTX_W {
        DLWTX_W { w: self }
    }
    #[doc = "Bit 18 - DCS Short Read Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsr0tx(&mut self) -> DSR0TX_W {
        DSR0TX_W { w: self }
    }
    #[doc = "Bit 17 - DCS Short Read One parameter Transmission"]
    #[inline(always)]
    pub fn dsw1tx(&mut self) -> DSW1TX_W {
        DSW1TX_W { w: self }
    }
    #[doc = "Bit 16 - DCS Short Write Zero parameter Transmission"]
    #[inline(always)]
    pub fn dsw0tx(&mut self) -> DSW0TX_W {
        DSW0TX_W { w: self }
    }
    #[doc = "Bit 14 - Generic Long Write Transmission"]
    #[inline(always)]
    pub fn glwtx(&mut self) -> GLWTX_W {
        GLWTX_W { w: self }
    }
    #[doc = "Bit 13 - Generic Short Read Two parameters Transmission"]
    #[inline(always)]
    pub fn gsr2tx(&mut self) -> GSR2TX_W {
        GSR2TX_W { w: self }
    }
    #[doc = "Bit 12 - Generic Short Read One parameters Transmission"]
    #[inline(always)]
    pub fn gsr1tx(&mut self) -> GSR1TX_W {
        GSR1TX_W { w: self }
    }
    #[doc = "Bit 11 - Generic Short Read Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsr0tx(&mut self) -> GSR0TX_W {
        GSR0TX_W { w: self }
    }
    #[doc = "Bit 10 - Generic Short Write Two parameters Transmission"]
    #[inline(always)]
    pub fn gsw2tx(&mut self) -> GSW2TX_W {
        GSW2TX_W { w: self }
    }
    #[doc = "Bit 9 - Generic Short Write One parameters Transmission"]
    #[inline(always)]
    pub fn gsw1tx(&mut self) -> GSW1TX_W {
        GSW1TX_W { w: self }
    }
    #[doc = "Bit 8 - Generic Short Write Zero parameters Transmission"]
    #[inline(always)]
    pub fn gsw0tx(&mut self) -> GSW0TX_W {
        GSW0TX_W { w: self }
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn are(&mut self) -> ARE_W {
        ARE_W { w: self }
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn teare(&mut self) -> TEARE_W {
        TEARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Command mode Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcr](index.html) module"]
pub struct CMCR_SPEC;
impl crate::RegisterSpec for CMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmcr::R](R) reader structure"]
impl crate::Readable for CMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmcr::W](W) writer structure"]
impl crate::Writable for CMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMCR to value 0"]
impl crate::Resettable for CMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
