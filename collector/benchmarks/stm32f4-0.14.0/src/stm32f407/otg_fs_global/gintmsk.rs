#[doc = "Register `GINTMSK` reader"]
pub struct R(crate::R<GINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK` writer"]
pub struct W(crate::W<GINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_SPEC>;
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
impl From<crate::W<GINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMISM` reader - Mode mismatch interrupt mask"]
pub struct MMISM_R(crate::FieldReader<bool, bool>);
impl MMISM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMISM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMISM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMISM` writer - Mode mismatch interrupt mask"]
pub struct MMISM_W<'a> {
    w: &'a mut W,
}
impl<'a> MMISM_W<'a> {
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
#[doc = "Field `OTGINT` reader - OTG interrupt mask"]
pub struct OTGINT_R(crate::FieldReader<bool, bool>);
impl OTGINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGINT` writer - OTG interrupt mask"]
pub struct OTGINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGINT_W<'a> {
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
#[doc = "Field `SOFM` reader - Start of frame mask"]
pub struct SOFM_R(crate::FieldReader<bool, bool>);
impl SOFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFM` writer - Start of frame mask"]
pub struct SOFM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFM_W<'a> {
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
#[doc = "Field `RXFLVLM` reader - Receive FIFO non-empty mask"]
pub struct RXFLVLM_R(crate::FieldReader<bool, bool>);
impl RXFLVLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFLVLM` writer - Receive FIFO non-empty mask"]
pub struct RXFLVLM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLVLM_W<'a> {
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
#[doc = "Field `NPTXFEM` reader - Non-periodic TxFIFO empty mask"]
pub struct NPTXFEM_R(crate::FieldReader<bool, bool>);
impl NPTXFEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPTXFEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFEM` writer - Non-periodic TxFIFO empty mask"]
pub struct NPTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEM_W<'a> {
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
#[doc = "Field `GINAKEFFM` reader - Global non-periodic IN NAK effective mask"]
pub struct GINAKEFFM_R(crate::FieldReader<bool, bool>);
impl GINAKEFFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINAKEFFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINAKEFFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINAKEFFM` writer - Global non-periodic IN NAK effective mask"]
pub struct GINAKEFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> GINAKEFFM_W<'a> {
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
#[doc = "Field `GONAKEFFM` reader - Global OUT NAK effective mask"]
pub struct GONAKEFFM_R(crate::FieldReader<bool, bool>);
impl GONAKEFFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        GONAKEFFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GONAKEFFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GONAKEFFM` writer - Global OUT NAK effective mask"]
pub struct GONAKEFFM_W<'a> {
    w: &'a mut W,
}
impl<'a> GONAKEFFM_W<'a> {
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
#[doc = "Field `ESUSPM` reader - Early suspend mask"]
pub struct ESUSPM_R(crate::FieldReader<bool, bool>);
impl ESUSPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESUSPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESUSPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSPM` writer - Early suspend mask"]
pub struct ESUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSPM_W<'a> {
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
#[doc = "Field `USBSUSPM` reader - USB suspend mask"]
pub struct USBSUSPM_R(crate::FieldReader<bool, bool>);
impl USBSUSPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSUSPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSUSPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSUSPM` writer - USB suspend mask"]
pub struct USBSUSPM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSPM_W<'a> {
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
#[doc = "Field `USBRST` reader - USB reset mask"]
pub struct USBRST_R(crate::FieldReader<bool, bool>);
impl USBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRST` writer - USB reset mask"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Field `ENUMDNEM` reader - Enumeration done mask"]
pub struct ENUMDNEM_R(crate::FieldReader<bool, bool>);
impl ENUMDNEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENUMDNEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMDNEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUMDNEM` writer - Enumeration done mask"]
pub struct ENUMDNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDNEM_W<'a> {
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
#[doc = "Field `ISOODRPM` reader - Isochronous OUT packet dropped interrupt mask"]
pub struct ISOODRPM_R(crate::FieldReader<bool, bool>);
impl ISOODRPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOODRPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOODRPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOODRPM` writer - Isochronous OUT packet dropped interrupt mask"]
pub struct ISOODRPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOODRPM_W<'a> {
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
#[doc = "Field `EOPFM` reader - End of periodic frame interrupt mask"]
pub struct EOPFM_R(crate::FieldReader<bool, bool>);
impl EOPFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPFM` writer - End of periodic frame interrupt mask"]
pub struct EOPFM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `EPMISM` reader - Endpoint mismatch interrupt mask"]
pub struct EPMISM_R(crate::FieldReader<bool, bool>);
impl EPMISM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPMISM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPMISM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPMISM` writer - Endpoint mismatch interrupt mask"]
pub struct EPMISM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPMISM_W<'a> {
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
#[doc = "Field `IEPINT` reader - IN endpoints interrupt mask"]
pub struct IEPINT_R(crate::FieldReader<bool, bool>);
impl IEPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEPINT` writer - IN endpoints interrupt mask"]
pub struct IEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPINT_W<'a> {
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
#[doc = "Field `OEPINT` reader - OUT endpoints interrupt mask"]
pub struct OEPINT_R(crate::FieldReader<bool, bool>);
impl OEPINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEPINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEPINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEPINT` writer - OUT endpoints interrupt mask"]
pub struct OEPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPINT_W<'a> {
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
#[doc = "Field `IISOIXFRM` reader - Incomplete isochronous IN transfer mask"]
pub struct IISOIXFRM_R(crate::FieldReader<bool, bool>);
impl IISOIXFRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IISOIXFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IISOIXFRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IISOIXFRM` writer - Incomplete isochronous IN transfer mask"]
pub struct IISOIXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IISOIXFRM_W<'a> {
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
#[doc = "Field `IPXFRM_IISOOXFRM` reader - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub struct IPXFRM_IISOOXFRM_R(crate::FieldReader<bool, bool>);
impl IPXFRM_IISOOXFRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPXFRM_IISOOXFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPXFRM_IISOOXFRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPXFRM_IISOOXFRM` writer - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub struct IPXFRM_IISOOXFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPXFRM_IISOOXFRM_W<'a> {
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
#[doc = "Field `PRTIM` reader - Host port interrupt mask"]
pub struct PRTIM_R(crate::FieldReader<bool, bool>);
impl PRTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCIM` reader - Host channels interrupt mask"]
pub struct HCIM_R(crate::FieldReader<bool, bool>);
impl HCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCIM` writer - Host channels interrupt mask"]
pub struct HCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HCIM_W<'a> {
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
#[doc = "Field `PTXFEM` reader - Periodic TxFIFO empty mask"]
pub struct PTXFEM_R(crate::FieldReader<bool, bool>);
impl PTXFEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFEM` writer - Periodic TxFIFO empty mask"]
pub struct PTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEM_W<'a> {
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
#[doc = "Field `CIDSCHGM` reader - Connector ID status change mask"]
pub struct CIDSCHGM_R(crate::FieldReader<bool, bool>);
impl CIDSCHGM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIDSCHGM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDSCHGM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIDSCHGM` writer - Connector ID status change mask"]
pub struct CIDSCHGM_W<'a> {
    w: &'a mut W,
}
impl<'a> CIDSCHGM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt mask"]
pub struct DISCINT_R(crate::FieldReader<bool, bool>);
impl DISCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt mask"]
pub struct DISCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `SRQIM` reader - Session request/new session detected interrupt mask"]
pub struct SRQIM_R(crate::FieldReader<bool, bool>);
impl SRQIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQIM` writer - Session request/new session detected interrupt mask"]
pub struct SRQIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `WUIM` reader - Resume/remote wakeup detected interrupt mask"]
pub struct WUIM_R(crate::FieldReader<bool, bool>);
impl WUIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUIM` writer - Resume/remote wakeup detected interrupt mask"]
pub struct WUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WUIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&self) -> MMISM_R {
        MMISM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlm(&self) -> RXFLVLM_R {
        RXFLVLM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&self) -> NPTXFEM_R {
        NPTXFEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&self) -> GONAKEFFM_R {
        GONAKEFFM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    pub fn epmism(&self) -> EPMISM_R {
        EPMISM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn ipxfrm_iisooxfrm(&self) -> IPXFRM_IISOOXFRM_R {
        IPXFRM_IISOOXFRM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&self) -> HCIM_R {
        HCIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&self) -> PTXFEM_R {
        PTXFEM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&self) -> CIDSCHGM_R {
        CIDSCHGM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&self) -> SRQIM_R {
        SRQIM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&self) -> WUIM_R {
        WUIM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn mmism(&mut self) -> MMISM_W {
        MMISM_W { w: self }
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgint(&mut self) -> OTGINT_W {
        OTGINT_W { w: self }
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlm(&mut self) -> RXFLVLM_W {
        RXFLVLM_W { w: self }
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfem(&mut self) -> NPTXFEM_W {
        NPTXFEM_W { w: self }
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W {
        GINAKEFFM_W { w: self }
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn gonakeffm(&mut self) -> GONAKEFFM_W {
        GONAKEFFM_W { w: self }
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn esuspm(&mut self) -> ESUSPM_W {
        ESUSPM_W { w: self }
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W {
        USBSUSPM_W { w: self }
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W {
        ENUMDNEM_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W {
        ISOODRPM_W { w: self }
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfm(&mut self) -> EOPFM_W {
        EOPFM_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint mismatch interrupt mask"]
    #[inline(always)]
    pub fn epmism(&mut self) -> EPMISM_W {
        EPMISM_W { w: self }
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn iepint(&mut self) -> IEPINT_W {
        IEPINT_W { w: self }
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W {
        OEPINT_W { w: self }
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W {
        IISOIXFRM_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn ipxfrm_iisooxfrm(&mut self) -> IPXFRM_IISOOXFRM_W {
        IPXFRM_IISOOXFRM_W { w: self }
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hcim(&mut self) -> HCIM_W {
        HCIM_W { w: self }
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfem(&mut self) -> PTXFEM_W {
        PTXFEM_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn cidschgm(&mut self) -> CIDSCHGM_W {
        CIDSCHGM_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W {
        DISCINT_W { w: self }
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt mask"]
    #[inline(always)]
    pub fn srqim(&mut self) -> SRQIM_W {
        SRQIM_W { w: self }
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wuim(&mut self) -> WUIM_W {
        WUIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk](index.html) module"]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk::R](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
