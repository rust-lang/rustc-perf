#[doc = "Register `HCCHAR3` reader"]
pub struct R(crate::R<HCCHAR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR3` writer"]
pub struct W(crate::W<HCCHAR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR3_SPEC>;
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
impl From<crate::W<HCCHAR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub struct MPSIZ_R(crate::FieldReader<u16, u16>);
impl MPSIZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        MPSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPSIZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub struct MPSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub struct EPNUM_R(crate::FieldReader<u8, u8>);
impl EPNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub struct EPNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub struct EPDIR_R(crate::FieldReader<bool, bool>);
impl EPDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub struct EPDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIR_W<'a> {
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
#[doc = "Field `LSDEV` reader - Low-speed device"]
pub struct LSDEV_R(crate::FieldReader<bool, bool>);
impl LSDEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSDEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSDEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSDEV` writer - Low-speed device"]
pub struct LSDEV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSDEV_W<'a> {
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
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub struct EPTYP_R(crate::FieldReader<u8, u8>);
impl EPTYP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTYP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTYP` writer - Endpoint type"]
pub struct EPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `MCNT` reader - Multicount"]
pub struct MCNT_R(crate::FieldReader<u8, u8>);
impl MCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCNT` writer - Multicount"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DAD` reader - Device address"]
pub struct DAD_R(crate::FieldReader<u8, u8>);
impl DAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAD` writer - Device address"]
pub struct DAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 22)) | ((value as u32 & 0x7f) << 22);
        self.w
    }
}
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub struct ODDFRM_R(crate::FieldReader<bool, bool>);
impl ODDFRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODDFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODDFRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub struct ODDFRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ODDFRM_W<'a> {
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
#[doc = "Field `CHDIS` reader - Channel disable"]
pub struct CHDIS_R(crate::FieldReader<bool, bool>);
impl CHDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDIS` writer - Channel disable"]
pub struct CHDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIS_W<'a> {
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
#[doc = "Field `CHENA` reader - Channel enable"]
pub struct CHENA_R(crate::FieldReader<bool, bool>);
impl CHENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHENA` writer - Channel enable"]
pub struct CHENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CHENA_W<'a> {
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
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W {
        MPSIZ_W { w: self }
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EPNUM_W {
        EPNUM_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EPDIR_W {
        EPDIR_W { w: self }
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&mut self) -> LSDEV_W {
        LSDEV_W { w: self }
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W {
        EPTYP_W { w: self }
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W {
        DAD_W { w: self }
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> ODDFRM_W {
        ODDFRM_W { w: self }
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> CHDIS_W {
        CHDIS_W { w: self }
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&mut self) -> CHENA_W {
        CHENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar3](index.html) module"]
pub struct HCCHAR3_SPEC;
impl crate::RegisterSpec for HCCHAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar3::R](R) reader structure"]
impl crate::Readable for HCCHAR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar3::W](W) writer structure"]
impl crate::Writable for HCCHAR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCHAR3 to value 0"]
impl crate::Resettable for HCCHAR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
