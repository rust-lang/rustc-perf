#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDIFEN` reader - Peripheral Block Enable"]
pub struct SPDIFEN_R(crate::FieldReader<u8, u8>);
impl SPDIFEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPDIFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDIFEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIFEN` writer - Peripheral Block Enable"]
pub struct SPDIFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `RXDMAEN` reader - Receiver DMA ENable for data flow"]
pub struct RXDMAEN_R(crate::FieldReader<bool, bool>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAEN` writer - Receiver DMA ENable for data flow"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
#[doc = "Field `RXSTEO` reader - STerEO Mode"]
pub struct RXSTEO_R(crate::FieldReader<bool, bool>);
impl RXSTEO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTEO` writer - STerEO Mode"]
pub struct RXSTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTEO_W<'a> {
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
#[doc = "Field `DRFMT` reader - RX Data format"]
pub struct DRFMT_R(crate::FieldReader<u8, u8>);
impl DRFMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRFMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRFMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRFMT` writer - RX Data format"]
pub struct DRFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRFMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PMSK` reader - Mask Parity error bit"]
pub struct PMSK_R(crate::FieldReader<bool, bool>);
impl PMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMSK` writer - Mask Parity error bit"]
pub struct PMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PMSK_W<'a> {
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
#[doc = "Field `VMSK` reader - Mask of Validity bit"]
pub struct VMSK_R(crate::FieldReader<bool, bool>);
impl VMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMSK` writer - Mask of Validity bit"]
pub struct VMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> VMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CUMSK` reader - Mask of channel status and user bits"]
pub struct CUMSK_R(crate::FieldReader<bool, bool>);
impl CUMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CUMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUMSK` writer - Mask of channel status and user bits"]
pub struct CUMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CUMSK_W<'a> {
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
#[doc = "Field `PTMSK` reader - Mask of Preamble Type bits"]
pub struct PTMSK_R(crate::FieldReader<bool, bool>);
impl PTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTMSK` writer - Mask of Preamble Type bits"]
pub struct PTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PTMSK_W<'a> {
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
#[doc = "Field `CBDMAEN` reader - Control Buffer DMA ENable for control flow"]
pub struct CBDMAEN_R(crate::FieldReader<bool, bool>);
impl CBDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBDMAEN` writer - Control Buffer DMA ENable for control flow"]
pub struct CBDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDMAEN_W<'a> {
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
#[doc = "Field `CHSEL` reader - Channel Selection"]
pub struct CHSEL_R(crate::FieldReader<bool, bool>);
impl CHSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - Channel Selection"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
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
#[doc = "Field `NBTR` reader - Maximum allowed re-tries during synchronization phase"]
pub struct NBTR_R(crate::FieldReader<u8, u8>);
impl NBTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBTR` writer - Maximum allowed re-tries during synchronization phase"]
pub struct NBTR_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `WFA` reader - Wait For Activity"]
pub struct WFA_R(crate::FieldReader<bool, bool>);
impl WFA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFA` writer - Wait For Activity"]
pub struct WFA_W<'a> {
    w: &'a mut W,
}
impl<'a> WFA_W<'a> {
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
#[doc = "Field `INSEL` reader - input selection"]
pub struct INSEL_R(crate::FieldReader<u8, u8>);
impl INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL` writer - input selection"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral Block Enable"]
    #[inline(always)]
    pub fn spdifen(&self) -> SPDIFEN_R {
        SPDIFEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receiver DMA ENable for data flow"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STerEO Mode"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - RX Data format"]
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Mask Parity error bit"]
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask of Validity bit"]
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask of channel status and user bits"]
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask of Preamble Type bits"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Control Buffer DMA ENable for control flow"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel Selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Maximum allowed re-tries during synchronization phase"]
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Wait For Activity"]
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - input selection"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral Block Enable"]
    #[inline(always)]
    pub fn spdifen(&mut self) -> SPDIFEN_W {
        SPDIFEN_W { w: self }
    }
    #[doc = "Bit 2 - Receiver DMA ENable for data flow"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - STerEO Mode"]
    #[inline(always)]
    pub fn rxsteo(&mut self) -> RXSTEO_W {
        RXSTEO_W { w: self }
    }
    #[doc = "Bits 4:5 - RX Data format"]
    #[inline(always)]
    pub fn drfmt(&mut self) -> DRFMT_W {
        DRFMT_W { w: self }
    }
    #[doc = "Bit 6 - Mask Parity error bit"]
    #[inline(always)]
    pub fn pmsk(&mut self) -> PMSK_W {
        PMSK_W { w: self }
    }
    #[doc = "Bit 7 - Mask of Validity bit"]
    #[inline(always)]
    pub fn vmsk(&mut self) -> VMSK_W {
        VMSK_W { w: self }
    }
    #[doc = "Bit 8 - Mask of channel status and user bits"]
    #[inline(always)]
    pub fn cumsk(&mut self) -> CUMSK_W {
        CUMSK_W { w: self }
    }
    #[doc = "Bit 9 - Mask of Preamble Type bits"]
    #[inline(always)]
    pub fn ptmsk(&mut self) -> PTMSK_W {
        PTMSK_W { w: self }
    }
    #[doc = "Bit 10 - Control Buffer DMA ENable for control flow"]
    #[inline(always)]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W {
        CBDMAEN_W { w: self }
    }
    #[doc = "Bit 11 - Channel Selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Maximum allowed re-tries during synchronization phase"]
    #[inline(always)]
    pub fn nbtr(&mut self) -> NBTR_W {
        NBTR_W { w: self }
    }
    #[doc = "Bit 14 - Wait For Activity"]
    #[inline(always)]
    pub fn wfa(&mut self) -> WFA_W {
        WFA_W { w: self }
    }
    #[doc = "Bits 16:18 - input selection"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
