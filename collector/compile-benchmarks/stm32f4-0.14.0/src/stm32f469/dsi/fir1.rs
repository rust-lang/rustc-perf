#[doc = "Register `FIR1` reader"]
pub struct R(crate::R<FIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIR1` writer"]
pub struct W(crate::W<FIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIR1_SPEC>;
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
impl From<crate::W<FIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGPRXE` reader - Force Generic Payload Receive Error"]
pub struct FGPRXE_R(crate::FieldReader<bool, bool>);
impl FGPRXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FGPRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FGPRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FGPRXE` writer - Force Generic Payload Receive Error"]
pub struct FGPRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPRXE_W<'a> {
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
#[doc = "Field `FGPRDE` reader - Force Generic Payload Read Error"]
pub struct FGPRDE_R(crate::FieldReader<bool, bool>);
impl FGPRDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FGPRDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FGPRDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FGPRDE` writer - Force Generic Payload Read Error"]
pub struct FGPRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPRDE_W<'a> {
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
#[doc = "Field `FGPTXE` reader - Force Generic Payload Transmit Error"]
pub struct FGPTXE_R(crate::FieldReader<bool, bool>);
impl FGPTXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FGPTXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FGPTXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FGPTXE` writer - Force Generic Payload Transmit Error"]
pub struct FGPTXE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPTXE_W<'a> {
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
#[doc = "Field `FGPWRE` reader - Force Generic Payload Write Error"]
pub struct FGPWRE_R(crate::FieldReader<bool, bool>);
impl FGPWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FGPWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FGPWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FGPWRE` writer - Force Generic Payload Write Error"]
pub struct FGPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGPWRE_W<'a> {
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
#[doc = "Field `FGCWRE` reader - Force Generic Command Write Error"]
pub struct FGCWRE_R(crate::FieldReader<bool, bool>);
impl FGCWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FGCWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FGCWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FGCWRE` writer - Force Generic Command Write Error"]
pub struct FGCWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FGCWRE_W<'a> {
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
#[doc = "Field `FLPWRE` reader - Force LTDC Payload Write Error"]
pub struct FLPWRE_R(crate::FieldReader<bool, bool>);
impl FLPWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLPWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLPWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLPWRE` writer - Force LTDC Payload Write Error"]
pub struct FLPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPWRE_W<'a> {
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
#[doc = "Field `FEOTPE` reader - Force EoTp Error"]
pub struct FEOTPE_R(crate::FieldReader<bool, bool>);
impl FEOTPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEOTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEOTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEOTPE` writer - Force EoTp Error"]
pub struct FEOTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEOTPE_W<'a> {
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
#[doc = "Field `FPSE` reader - Force Packet Size Error"]
pub struct FPSE_R(crate::FieldReader<bool, bool>);
impl FPSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPSE` writer - Force Packet Size Error"]
pub struct FPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSE_W<'a> {
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
#[doc = "Field `FCRCE` reader - Force CRC Error"]
pub struct FCRCE_R(crate::FieldReader<bool, bool>);
impl FCRCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCRCE` writer - Force CRC Error"]
pub struct FCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRCE_W<'a> {
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
#[doc = "Field `FECCME` reader - Force ECC Multi-bit Error"]
pub struct FECCME_R(crate::FieldReader<bool, bool>);
impl FECCME_R {
    pub(crate) fn new(bits: bool) -> Self {
        FECCME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FECCME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FECCME` writer - Force ECC Multi-bit Error"]
pub struct FECCME_W<'a> {
    w: &'a mut W,
}
impl<'a> FECCME_W<'a> {
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
#[doc = "Field `FECCSE` reader - Force ECC Single-bit Error"]
pub struct FECCSE_R(crate::FieldReader<bool, bool>);
impl FECCSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FECCSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FECCSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FECCSE` writer - Force ECC Single-bit Error"]
pub struct FECCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FECCSE_W<'a> {
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
#[doc = "Field `FTOLPRX` reader - Force Timeout Low-Power Reception"]
pub struct FTOLPRX_R(crate::FieldReader<bool, bool>);
impl FTOLPRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTOLPRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTOLPRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTOLPRX` writer - Force Timeout Low-Power Reception"]
pub struct FTOLPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> FTOLPRX_W<'a> {
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
#[doc = "Field `FTOHSTX` reader - Force Timeout High-Speed Transmission"]
pub struct FTOHSTX_R(crate::FieldReader<bool, bool>);
impl FTOHSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTOHSTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTOHSTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTOHSTX` writer - Force Timeout High-Speed Transmission"]
pub struct FTOHSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> FTOHSTX_W<'a> {
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
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    pub fn fgprxe(&self) -> FGPRXE_R {
        FGPRXE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    pub fn fgprde(&self) -> FGPRDE_R {
        FGPRDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn fgptxe(&self) -> FGPTXE_R {
        FGPTXE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    pub fn fgpwre(&self) -> FGPWRE_R {
        FGPWRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    pub fn fgcwre(&self) -> FGCWRE_R {
        FGCWRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    pub fn flpwre(&self) -> FLPWRE_R {
        FLPWRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    pub fn feotpe(&self) -> FEOTPE_R {
        FEOTPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    pub fn fpse(&self) -> FPSE_R {
        FPSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    pub fn fcrce(&self) -> FCRCE_R {
        FCRCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    pub fn feccme(&self) -> FECCME_R {
        FECCME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    pub fn feccse(&self) -> FECCSE_R {
        FECCSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn ftolprx(&self) -> FTOLPRX_R {
        FTOLPRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn ftohstx(&self) -> FTOHSTX_R {
        FTOHSTX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Force Generic Payload Receive Error"]
    #[inline(always)]
    pub fn fgprxe(&mut self) -> FGPRXE_W {
        FGPRXE_W { w: self }
    }
    #[doc = "Bit 11 - Force Generic Payload Read Error"]
    #[inline(always)]
    pub fn fgprde(&mut self) -> FGPRDE_W {
        FGPRDE_W { w: self }
    }
    #[doc = "Bit 10 - Force Generic Payload Transmit Error"]
    #[inline(always)]
    pub fn fgptxe(&mut self) -> FGPTXE_W {
        FGPTXE_W { w: self }
    }
    #[doc = "Bit 9 - Force Generic Payload Write Error"]
    #[inline(always)]
    pub fn fgpwre(&mut self) -> FGPWRE_W {
        FGPWRE_W { w: self }
    }
    #[doc = "Bit 8 - Force Generic Command Write Error"]
    #[inline(always)]
    pub fn fgcwre(&mut self) -> FGCWRE_W {
        FGCWRE_W { w: self }
    }
    #[doc = "Bit 7 - Force LTDC Payload Write Error"]
    #[inline(always)]
    pub fn flpwre(&mut self) -> FLPWRE_W {
        FLPWRE_W { w: self }
    }
    #[doc = "Bit 6 - Force EoTp Error"]
    #[inline(always)]
    pub fn feotpe(&mut self) -> FEOTPE_W {
        FEOTPE_W { w: self }
    }
    #[doc = "Bit 5 - Force Packet Size Error"]
    #[inline(always)]
    pub fn fpse(&mut self) -> FPSE_W {
        FPSE_W { w: self }
    }
    #[doc = "Bit 4 - Force CRC Error"]
    #[inline(always)]
    pub fn fcrce(&mut self) -> FCRCE_W {
        FCRCE_W { w: self }
    }
    #[doc = "Bit 3 - Force ECC Multi-bit Error"]
    #[inline(always)]
    pub fn feccme(&mut self) -> FECCME_W {
        FECCME_W { w: self }
    }
    #[doc = "Bit 2 - Force ECC Single-bit Error"]
    #[inline(always)]
    pub fn feccse(&mut self) -> FECCSE_W {
        FECCSE_W { w: self }
    }
    #[doc = "Bit 1 - Force Timeout Low-Power Reception"]
    #[inline(always)]
    pub fn ftolprx(&mut self) -> FTOLPRX_W {
        FTOLPRX_W { w: self }
    }
    #[doc = "Bit 0 - Force Timeout High-Speed Transmission"]
    #[inline(always)]
    pub fn ftohstx(&mut self) -> FTOHSTX_W {
        FTOHSTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Force Interrupt Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir1](index.html) module"]
pub struct FIR1_SPEC;
impl crate::RegisterSpec for FIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fir1::R](R) reader structure"]
impl crate::Readable for FIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fir1::W](W) writer structure"]
impl crate::Writable for FIR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIR1 to value 0"]
impl crate::Resettable for FIR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
