#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ECCPS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECCPS_A {
    #[doc = "0: ECC page size 256 bytes"]
    BYTES256 = 0,
    #[doc = "1: ECC page size 512 bytes"]
    BYTES512 = 1,
    #[doc = "2: ECC page size 1024 bytes"]
    BYTES1024 = 2,
    #[doc = "3: ECC page size 2048 bytes"]
    BYTES2048 = 3,
    #[doc = "4: ECC page size 4096 bytes"]
    BYTES4096 = 4,
    #[doc = "5: ECC page size 8192 bytes"]
    BYTES8192 = 5,
}
impl From<ECCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECCPS` reader - ECCPS"]
pub struct ECCPS_R(crate::FieldReader<u8, ECCPS_A>);
impl ECCPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECCPS_A> {
        match self.bits {
            0 => Some(ECCPS_A::BYTES256),
            1 => Some(ECCPS_A::BYTES512),
            2 => Some(ECCPS_A::BYTES1024),
            3 => Some(ECCPS_A::BYTES2048),
            4 => Some(ECCPS_A::BYTES4096),
            5 => Some(ECCPS_A::BYTES8192),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTES256`"]
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        **self == ECCPS_A::BYTES256
    }
    #[doc = "Checks if the value of the field is `BYTES512`"]
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        **self == ECCPS_A::BYTES512
    }
    #[doc = "Checks if the value of the field is `BYTES1024`"]
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        **self == ECCPS_A::BYTES1024
    }
    #[doc = "Checks if the value of the field is `BYTES2048`"]
    #[inline(always)]
    pub fn is_bytes2048(&self) -> bool {
        **self == ECCPS_A::BYTES2048
    }
    #[doc = "Checks if the value of the field is `BYTES4096`"]
    #[inline(always)]
    pub fn is_bytes4096(&self) -> bool {
        **self == ECCPS_A::BYTES4096
    }
    #[doc = "Checks if the value of the field is `BYTES8192`"]
    #[inline(always)]
    pub fn is_bytes8192(&self) -> bool {
        **self == ECCPS_A::BYTES8192
    }
}
impl core::ops::Deref for ECCPS_R {
    type Target = crate::FieldReader<u8, ECCPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCPS` writer - ECCPS"]
pub struct ECCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ECC page size 256 bytes"]
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES256)
    }
    #[doc = "ECC page size 512 bytes"]
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES512)
    }
    #[doc = "ECC page size 1024 bytes"]
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES1024)
    }
    #[doc = "ECC page size 2048 bytes"]
    #[inline(always)]
    pub fn bytes2048(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES2048)
    }
    #[doc = "ECC page size 4096 bytes"]
    #[inline(always)]
    pub fn bytes4096(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES4096)
    }
    #[doc = "ECC page size 8192 bytes"]
    #[inline(always)]
    pub fn bytes8192(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES8192)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `TAR` reader - TAR"]
pub struct TAR_R(crate::FieldReader<u8, u8>);
impl TAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAR` writer - TAR"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `TCLR` reader - TCLR"]
pub struct TCLR_R(crate::FieldReader<u8, u8>);
impl TCLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLR` writer - TCLR"]
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "ECCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCEN_A {
    #[doc = "0: ECC logic is disabled and reset"]
    DISABLED = 0,
    #[doc = "1: ECC logic is enabled"]
    ENABLED = 1,
}
impl From<ECCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCEN` reader - ECCEN"]
pub struct ECCEN_R(crate::FieldReader<bool, ECCEN_A>);
impl ECCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCEN_A {
        match self.bits {
            false => ECCEN_A::DISABLED,
            true => ECCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ECCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ECCEN_A::ENABLED
    }
}
impl core::ops::Deref for ECCEN_R {
    type Target = crate::FieldReader<bool, ECCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCEN` writer - ECCEN"]
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ECC logic is disabled and reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCEN_A::DISABLED)
    }
    #[doc = "ECC logic is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCEN_A::ENABLED)
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
#[doc = "PWID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWID_A {
    #[doc = "0: External memory device width 8 bits"]
    BITS8 = 0,
    #[doc = "1: External memory device width 16 bits"]
    BITS16 = 1,
}
impl From<PWID_A> for u8 {
    #[inline(always)]
    fn from(variant: PWID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWID` reader - PWID"]
pub struct PWID_R(crate::FieldReader<u8, PWID_A>);
impl PWID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWID_A> {
        match self.bits {
            0 => Some(PWID_A::BITS8),
            1 => Some(PWID_A::BITS16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        **self == PWID_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == PWID_A::BITS16
    }
}
impl core::ops::Deref for PWID_R {
    type Target = crate::FieldReader<u8, PWID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWID` writer - PWID"]
pub struct PWID_W<'a> {
    w: &'a mut W,
}
impl<'a> PWID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External memory device width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PWID_A::BITS8)
    }
    #[doc = "External memory device width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PWID_A::BITS16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "PTYP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYP_A {
    #[doc = "1: NAND Flash"]
    NANDFLASH = 1,
}
impl From<PTYP_A> for bool {
    #[inline(always)]
    fn from(variant: PTYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTYP` reader - PTYP"]
pub struct PTYP_R(crate::FieldReader<bool, PTYP_A>);
impl PTYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTYP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTYP_A> {
        match self.bits {
            true => Some(PTYP_A::NANDFLASH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NANDFLASH`"]
    #[inline(always)]
    pub fn is_nandflash(&self) -> bool {
        **self == PTYP_A::NANDFLASH
    }
}
impl core::ops::Deref for PTYP_R {
    type Target = crate::FieldReader<bool, PTYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTYP` writer - PTYP"]
pub struct PTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NAND Flash"]
    #[inline(always)]
    pub fn nandflash(self) -> &'a mut W {
        self.variant(PTYP_A::NANDFLASH)
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
#[doc = "PBKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBKEN_A {
    #[doc = "0: Corresponding memory bank is disabled"]
    DISABLED = 0,
    #[doc = "1: Corresponding memory bank is enabled"]
    ENABLED = 1,
}
impl From<PBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PBKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBKEN` reader - PBKEN"]
pub struct PBKEN_R(crate::FieldReader<bool, PBKEN_A>);
impl PBKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBKEN_A {
        match self.bits {
            false => PBKEN_A::DISABLED,
            true => PBKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PBKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PBKEN_A::ENABLED
    }
}
impl core::ops::Deref for PBKEN_R {
    type Target = crate::FieldReader<bool, PBKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBKEN` writer - PBKEN"]
pub struct PBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PBKEN_A::DISABLED)
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PBKEN_A::ENABLED)
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
#[doc = "PWAITEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWAITEN_A {
    #[doc = "0: Wait feature disabled"]
    DISABLED = 0,
    #[doc = "1: Wait feature enabled"]
    ENABLED = 1,
}
impl From<PWAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWAITEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub struct PWAITEN_R(crate::FieldReader<bool, PWAITEN_A>);
impl PWAITEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWAITEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWAITEN_A {
        match self.bits {
            false => PWAITEN_A::DISABLED,
            true => PWAITEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PWAITEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PWAITEN_A::ENABLED
    }
}
impl core::ops::Deref for PWAITEN_R {
    type Target = crate::FieldReader<bool, PWAITEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub struct PWAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWAITEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWAITEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wait feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWAITEN_A::DISABLED)
    }
    #[doc = "Wait feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWAITEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W {
        ECCPS_W { w: self }
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W {
        PWID_W { w: self }
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W {
        PTYP_W { w: self }
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W {
        PBKEN_W { w: self }
    }
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W {
        PWAITEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC Card/NAND Flash control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR to value 0x18"]
impl crate::Resettable for PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}
