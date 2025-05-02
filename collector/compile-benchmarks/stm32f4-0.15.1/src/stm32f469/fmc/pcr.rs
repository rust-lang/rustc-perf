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
    Bytes256 = 0,
    #[doc = "1: ECC page size 512 bytes"]
    Bytes512 = 1,
    #[doc = "2: ECC page size 1024 bytes"]
    Bytes1024 = 2,
    #[doc = "3: ECC page size 2048 bytes"]
    Bytes2048 = 3,
    #[doc = "4: ECC page size 4096 bytes"]
    Bytes4096 = 4,
    #[doc = "5: ECC page size 8192 bytes"]
    Bytes8192 = 5,
}
impl From<ECCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ECCPS` reader - ECCPS"]
pub type ECCPS_R = crate::FieldReader<u8, ECCPS_A>;
impl ECCPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECCPS_A> {
        match self.bits {
            0 => Some(ECCPS_A::Bytes256),
            1 => Some(ECCPS_A::Bytes512),
            2 => Some(ECCPS_A::Bytes1024),
            3 => Some(ECCPS_A::Bytes2048),
            4 => Some(ECCPS_A::Bytes4096),
            5 => Some(ECCPS_A::Bytes8192),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bytes256`"]
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == ECCPS_A::Bytes256
    }
    #[doc = "Checks if the value of the field is `Bytes512`"]
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == ECCPS_A::Bytes512
    }
    #[doc = "Checks if the value of the field is `Bytes1024`"]
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == ECCPS_A::Bytes1024
    }
    #[doc = "Checks if the value of the field is `Bytes2048`"]
    #[inline(always)]
    pub fn is_bytes2048(&self) -> bool {
        *self == ECCPS_A::Bytes2048
    }
    #[doc = "Checks if the value of the field is `Bytes4096`"]
    #[inline(always)]
    pub fn is_bytes4096(&self) -> bool {
        *self == ECCPS_A::Bytes4096
    }
    #[doc = "Checks if the value of the field is `Bytes8192`"]
    #[inline(always)]
    pub fn is_bytes8192(&self) -> bool {
        *self == ECCPS_A::Bytes8192
    }
}
#[doc = "Field `ECCPS` writer - ECCPS"]
pub type ECCPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, ECCPS_A, 3, O>;
impl<'a, const O: u8> ECCPS_W<'a, O> {
    #[doc = "ECC page size 256 bytes"]
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut W {
        self.variant(ECCPS_A::Bytes256)
    }
    #[doc = "ECC page size 512 bytes"]
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut W {
        self.variant(ECCPS_A::Bytes512)
    }
    #[doc = "ECC page size 1024 bytes"]
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut W {
        self.variant(ECCPS_A::Bytes1024)
    }
    #[doc = "ECC page size 2048 bytes"]
    #[inline(always)]
    pub fn bytes2048(self) -> &'a mut W {
        self.variant(ECCPS_A::Bytes2048)
    }
    #[doc = "ECC page size 4096 bytes"]
    #[inline(always)]
    pub fn bytes4096(self) -> &'a mut W {
        self.variant(ECCPS_A::Bytes4096)
    }
    #[doc = "ECC page size 8192 bytes"]
    #[inline(always)]
    pub fn bytes8192(self) -> &'a mut W {
        self.variant(ECCPS_A::Bytes8192)
    }
}
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TCLR` reader - TCLR"]
pub type TCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PCR_SPEC, u8, u8, 4, O>;
#[doc = "ECCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCEN_A {
    #[doc = "0: ECC logic is disabled and reset"]
    Disabled = 0,
    #[doc = "1: ECC logic is enabled"]
    Enabled = 1,
}
impl From<ECCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader<ECCEN_A>;
impl ECCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCEN_A {
        match self.bits {
            false => ECCEN_A::Disabled,
            true => ECCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCEN_A::Enabled
    }
}
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, ECCEN_A, O>;
impl<'a, const O: u8> ECCEN_W<'a, O> {
    #[doc = "ECC logic is disabled and reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCEN_A::Disabled)
    }
    #[doc = "ECC logic is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCEN_A::Enabled)
    }
}
#[doc = "PWID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWID_A {
    #[doc = "0: External memory device width 8 bits"]
    Bits8 = 0,
    #[doc = "1: External memory device width 16 bits"]
    Bits16 = 1,
}
impl From<PWID_A> for u8 {
    #[inline(always)]
    fn from(variant: PWID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWID` reader - PWID"]
pub type PWID_R = crate::FieldReader<u8, PWID_A>;
impl PWID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWID_A> {
        match self.bits {
            0 => Some(PWID_A::Bits8),
            1 => Some(PWID_A::Bits16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PWID_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PWID_A::Bits16
    }
}
#[doc = "Field `PWID` writer - PWID"]
pub type PWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, PWID_A, 2, O>;
impl<'a, const O: u8> PWID_W<'a, O> {
    #[doc = "External memory device width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PWID_A::Bits8)
    }
    #[doc = "External memory device width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PWID_A::Bits16)
    }
}
#[doc = "PTYP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYP_A {
    #[doc = "1: NAND Flash"]
    Nandflash = 1,
}
impl From<PTYP_A> for bool {
    #[inline(always)]
    fn from(variant: PTYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTYP` reader - PTYP"]
pub type PTYP_R = crate::BitReader<PTYP_A>;
impl PTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTYP_A> {
        match self.bits {
            true => Some(PTYP_A::Nandflash),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Nandflash`"]
    #[inline(always)]
    pub fn is_nandflash(&self) -> bool {
        *self == PTYP_A::Nandflash
    }
}
#[doc = "Field `PTYP` writer - PTYP"]
pub type PTYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, PTYP_A, O>;
impl<'a, const O: u8> PTYP_W<'a, O> {
    #[doc = "NAND Flash"]
    #[inline(always)]
    pub fn nandflash(self) -> &'a mut W {
        self.variant(PTYP_A::Nandflash)
    }
}
#[doc = "PBKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBKEN_A {
    #[doc = "0: Corresponding memory bank is disabled"]
    Disabled = 0,
    #[doc = "1: Corresponding memory bank is enabled"]
    Enabled = 1,
}
impl From<PBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PBKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PBKEN_R = crate::BitReader<PBKEN_A>;
impl PBKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBKEN_A {
        match self.bits {
            false => PBKEN_A::Disabled,
            true => PBKEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PBKEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PBKEN_A::Enabled
    }
}
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, PBKEN_A, O>;
impl<'a, const O: u8> PBKEN_W<'a, O> {
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PBKEN_A::Disabled)
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PBKEN_A::Enabled)
    }
}
#[doc = "PWAITEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWAITEN_A {
    #[doc = "0: Wait feature disabled"]
    Disabled = 0,
    #[doc = "1: Wait feature enabled"]
    Enabled = 1,
}
impl From<PWAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWAITEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PWAITEN_R = crate::BitReader<PWAITEN_A>;
impl PWAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWAITEN_A {
        match self.bits {
            false => PWAITEN_A::Disabled,
            true => PWAITEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWAITEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWAITEN_A::Enabled
    }
}
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PWAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, PWAITEN_A, O>;
impl<'a, const O: u8> PWAITEN_W<'a, O> {
    #[doc = "Wait feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWAITEN_A::Disabled)
    }
    #[doc = "Wait feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWAITEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
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
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W<17> {
        ECCPS_W::new(self)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<13> {
        TAR_W::new(self)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<9> {
        TCLR_W::new(self)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<6> {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W<4> {
        PWID_W::new(self)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W<3> {
        PTYP_W::new(self)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W<2> {
        PBKEN_W::new(self)
    }
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W<1> {
        PWAITEN_W::new(self)
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
