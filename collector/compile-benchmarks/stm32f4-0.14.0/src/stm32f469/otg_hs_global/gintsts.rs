#[doc = "Register `GINTSTS` reader"]
pub struct R(crate::R<GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTSTS` writer"]
pub struct W(crate::W<GINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTSTS_SPEC>;
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
impl From<crate::W<GINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMOD` reader - Current mode of operation"]
pub struct CMOD_R(crate::FieldReader<bool, bool>);
impl CMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMIS` reader - Mode mismatch interrupt"]
pub struct MMIS_R(crate::FieldReader<bool, bool>);
impl MMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMIS` writer - Mode mismatch interrupt"]
pub struct MMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMIS_W<'a> {
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
#[doc = "Field `OTGINT` reader - OTG interrupt"]
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
#[doc = "Field `SOF` reader - Start of frame"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` writer - Start of frame"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Field `RXFLVL` reader - RxFIFO nonempty"]
pub struct RXFLVL_R(crate::FieldReader<bool, bool>);
impl RXFLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFE` reader - Nonperiodic TxFIFO empty"]
pub struct NPTXFE_R(crate::FieldReader<bool, bool>);
impl NPTXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPTXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINAKEFF` reader - Global IN nonperiodic NAK effective"]
pub struct GINAKEFF_R(crate::FieldReader<bool, bool>);
impl GINAKEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINAKEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINAKEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOUTNAKEFF` reader - Global OUT NAK effective"]
pub struct BOUTNAKEFF_R(crate::FieldReader<bool, bool>);
impl BOUTNAKEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOUTNAKEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOUTNAKEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSP` reader - Early suspend"]
pub struct ESUSP_R(crate::FieldReader<bool, bool>);
impl ESUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESUSP` writer - Early suspend"]
pub struct ESUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSP_W<'a> {
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
#[doc = "Field `USBSUSP` reader - USB suspend"]
pub struct USBSUSP_R(crate::FieldReader<bool, bool>);
impl USBSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSUSP` writer - USB suspend"]
pub struct USBSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSP_W<'a> {
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
#[doc = "Field `USBRST` reader - USB reset"]
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
#[doc = "Field `USBRST` writer - USB reset"]
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
#[doc = "Field `ENUMDNE` reader - Enumeration done"]
pub struct ENUMDNE_R(crate::FieldReader<bool, bool>);
impl ENUMDNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENUMDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMDNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUMDNE` writer - Enumeration done"]
pub struct ENUMDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDNE_W<'a> {
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
#[doc = "Field `ISOODRP` reader - Isochronous OUT packet dropped interrupt"]
pub struct ISOODRP_R(crate::FieldReader<bool, bool>);
impl ISOODRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOODRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOODRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOODRP` writer - Isochronous OUT packet dropped interrupt"]
pub struct ISOODRP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOODRP_W<'a> {
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
#[doc = "Field `EOPF` reader - End of periodic frame interrupt"]
pub struct EOPF_R(crate::FieldReader<bool, bool>);
impl EOPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOPF` writer - End of periodic frame interrupt"]
pub struct EOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPF_W<'a> {
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
#[doc = "Field `IEPINT` reader - IN endpoint interrupt"]
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
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt"]
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
#[doc = "Field `IISOIXFR` reader - Incomplete isochronous IN transfer"]
pub struct IISOIXFR_R(crate::FieldReader<bool, bool>);
impl IISOIXFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IISOIXFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IISOIXFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IISOIXFR` writer - Incomplete isochronous IN transfer"]
pub struct IISOIXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IISOIXFR_W<'a> {
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
#[doc = "Field `PXFR_INCOMPISOOUT` reader - Incomplete periodic transfer"]
pub struct PXFR_INCOMPISOOUT_R(crate::FieldReader<bool, bool>);
impl PXFR_INCOMPISOOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PXFR_INCOMPISOOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXFR_INCOMPISOOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PXFR_INCOMPISOOUT` writer - Incomplete periodic transfer"]
pub struct PXFR_INCOMPISOOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> PXFR_INCOMPISOOUT_W<'a> {
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
#[doc = "Field `DATAFSUSP` reader - Data fetch suspended"]
pub struct DATAFSUSP_R(crate::FieldReader<bool, bool>);
impl DATAFSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAFSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAFSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAFSUSP` writer - Data fetch suspended"]
pub struct DATAFSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAFSUSP_W<'a> {
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
#[doc = "Field `HPRTINT` reader - Host port interrupt"]
pub struct HPRTINT_R(crate::FieldReader<bool, bool>);
impl HPRTINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPRTINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPRTINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCINT` reader - Host channels interrupt"]
pub struct HCINT_R(crate::FieldReader<bool, bool>);
impl HCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXFE` reader - Periodic TxFIFO empty"]
pub struct PTXFE_R(crate::FieldReader<bool, bool>);
impl PTXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIDSCHG` reader - Connector ID status change"]
pub struct CIDSCHG_R(crate::FieldReader<bool, bool>);
impl CIDSCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIDSCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDSCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIDSCHG` writer - Connector ID status change"]
pub struct CIDSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> CIDSCHG_W<'a> {
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
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt"]
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
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt"]
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
#[doc = "Field `SRQINT` reader - Session request/new session detected interrupt"]
pub struct SRQINT_R(crate::FieldReader<bool, bool>);
impl SRQINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRQINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRQINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRQINT` writer - Session request/new session detected interrupt"]
pub struct SRQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQINT_W<'a> {
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
#[doc = "Field `WKUINT` reader - Resume/remote wakeup detected interrupt"]
pub struct WKUINT_R(crate::FieldReader<bool, bool>);
impl WKUINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUINT` writer - Resume/remote wakeup detected interrupt"]
pub struct WKUINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUINT_W<'a> {
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
    #[doc = "Bit 0 - Current mode of operation"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO nonempty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Nonperiodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global IN nonperiodic NAK effective"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn boutnakeff(&self) -> BOUTNAKEFF_R {
        BOUTNAKEFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(&self) -> PXFR_INCOMPISOOUT_R {
        PXFR_INCOMPISOOUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkuint(&self) -> WKUINT_R {
        WKUINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W {
        MMIS_W { w: self }
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W {
        ESUSP_W { w: self }
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W {
        USBSUSP_W { w: self }
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W {
        ENUMDNE_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W {
        ISOODRP_W { w: self }
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W {
        EOPF_W { w: self }
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W {
        IISOIXFR_W { w: self }
    }
    #[doc = "Bit 21 - Incomplete periodic transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(&mut self) -> PXFR_INCOMPISOOUT_W {
        PXFR_INCOMPISOOUT_W { w: self }
    }
    #[doc = "Bit 22 - Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W {
        DATAFSUSP_W { w: self }
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W {
        CIDSCHG_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W {
        DISCINT_W { w: self }
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W {
        SRQINT_W { w: self }
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkuint(&mut self) -> WKUINT_W {
        WKUINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS core interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](index.html) module"]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts::R](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintsts::W](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTSTS to value 0x0400_0020"]
impl crate::Resettable for GINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0020
    }
}
