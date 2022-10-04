#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTH_A {
    #[doc = "0: FIFO empty"]
    EMPTY = 0,
    #[doc = "1: 1⁄4 FIFO"]
    QUARTER1 = 1,
    #[doc = "2: 1⁄2 FIFO"]
    QUARTER2 = 2,
    #[doc = "3: 3⁄4 FIFO"]
    QUARTER3 = 3,
    #[doc = "4: FIFO full"]
    FULL = 4,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTH` reader - FIFO threshold"]
pub struct FTH_R(crate::FieldReader<u8, FTH_A>);
impl FTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTH_A> {
        match self.bits {
            0 => Some(FTH_A::EMPTY),
            1 => Some(FTH_A::QUARTER1),
            2 => Some(FTH_A::QUARTER2),
            3 => Some(FTH_A::QUARTER3),
            4 => Some(FTH_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == FTH_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `QUARTER1`"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        **self == FTH_A::QUARTER1
    }
    #[doc = "Checks if the value of the field is `QUARTER2`"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        **self == FTH_A::QUARTER2
    }
    #[doc = "Checks if the value of the field is `QUARTER3`"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        **self == FTH_A::QUARTER3
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == FTH_A::FULL
    }
}
impl core::ops::Deref for FTH_R {
    type Target = crate::FieldReader<u8, FTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTH` writer - FIFO threshold"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(FTH_A::EMPTY)
    }
    #[doc = "1⁄4 FIFO"]
    #[inline(always)]
    pub fn quarter1(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER1)
    }
    #[doc = "1⁄2 FIFO"]
    #[inline(always)]
    pub fn quarter2(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER2)
    }
    #[doc = "3⁄4 FIFO"]
    #[inline(always)]
    pub fn quarter3(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER3)
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "FIFO flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLUSH_A {
    #[doc = "0: No FIFO flush"]
    NOFLUSH = 0,
    #[doc = "1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared"]
    FLUSH = 1,
}
impl From<FFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLUSH` reader - FIFO flush"]
pub struct FFLUSH_R(crate::FieldReader<bool, FFLUSH_A>);
impl FFLUSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFLUSH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLUSH_A {
        match self.bits {
            false => FFLUSH_A::NOFLUSH,
            true => FFLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOFLUSH`"]
    #[inline(always)]
    pub fn is_no_flush(&self) -> bool {
        **self == FFLUSH_A::NOFLUSH
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        **self == FFLUSH_A::FLUSH
    }
}
impl core::ops::Deref for FFLUSH_R {
    type Target = crate::FieldReader<bool, FFLUSH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub struct FFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLUSH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No FIFO flush"]
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::NOFLUSH)
    }
    #[doc = "FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::FLUSH)
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
#[doc = "Field `TRIS` reader - Tristate management on data line"]
pub struct TRIS_R(crate::FieldReader<bool, bool>);
impl TRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIS` writer - Tristate management on data line"]
pub struct TRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIS_W<'a> {
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
#[doc = "Mute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_A {
    #[doc = "0: No mute mode"]
    DISABLED = 0,
    #[doc = "1: Mute mode enabled"]
    ENABLED = 1,
}
impl From<MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTE` reader - Mute"]
pub struct MUTE_R(crate::FieldReader<bool, MUTE_A>);
impl MUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_A {
        match self.bits {
            false => MUTE_A::DISABLED,
            true => MUTE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MUTE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MUTE_A::ENABLED
    }
}
impl core::ops::Deref for MUTE_R {
    type Target = crate::FieldReader<bool, MUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTE` writer - Mute"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No mute mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTE_A::DISABLED)
    }
    #[doc = "Mute mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTE_A::ENABLED)
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
#[doc = "Mute value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEVAL_A {
    #[doc = "0: Bit value 0 is sent during the mute mode"]
    SENDZERO = 0,
    #[doc = "1: Last values are sent during the mute mode"]
    SENDLAST = 1,
}
impl From<MUTEVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEVAL` reader - Mute value"]
pub struct MUTEVAL_R(crate::FieldReader<bool, MUTEVAL_A>);
impl MUTEVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTEVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEVAL_A {
        match self.bits {
            false => MUTEVAL_A::SENDZERO,
            true => MUTEVAL_A::SENDLAST,
        }
    }
    #[doc = "Checks if the value of the field is `SENDZERO`"]
    #[inline(always)]
    pub fn is_send_zero(&self) -> bool {
        **self == MUTEVAL_A::SENDZERO
    }
    #[doc = "Checks if the value of the field is `SENDLAST`"]
    #[inline(always)]
    pub fn is_send_last(&self) -> bool {
        **self == MUTEVAL_A::SENDLAST
    }
}
impl core::ops::Deref for MUTEVAL_R {
    type Target = crate::FieldReader<bool, MUTEVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTEVAL` writer - Mute value"]
pub struct MUTEVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTEVAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bit value 0 is sent during the mute mode"]
    #[inline(always)]
    pub fn send_zero(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SENDZERO)
    }
    #[doc = "Last values are sent during the mute mode"]
    #[inline(always)]
    pub fn send_last(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SENDLAST)
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
#[doc = "Field `MUTECNT` reader - Mute counter"]
pub struct MUTECNT_R(crate::FieldReader<u8, u8>);
impl MUTECNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUTECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTECNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTECNT` writer - Mute counter"]
pub struct MUTECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Complement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPL_A {
    #[doc = "0: 1’s complement representation"]
    ONESCOMPLEMENT = 0,
    #[doc = "1: 2’s complement representation"]
    TWOSCOMPLEMENT = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPL` reader - Complement bit"]
pub struct CPL_R(crate::FieldReader<bool, CPL_A>);
impl CPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::ONESCOMPLEMENT,
            true => CPL_A::TWOSCOMPLEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `ONESCOMPLEMENT`"]
    #[inline(always)]
    pub fn is_ones_complement(&self) -> bool {
        **self == CPL_A::ONESCOMPLEMENT
    }
    #[doc = "Checks if the value of the field is `TWOSCOMPLEMENT`"]
    #[inline(always)]
    pub fn is_twos_complement(&self) -> bool {
        **self == CPL_A::TWOSCOMPLEMENT
    }
}
impl core::ops::Deref for CPL_R {
    type Target = crate::FieldReader<bool, CPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPL` writer - Complement bit"]
pub struct CPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1’s complement representation"]
    #[inline(always)]
    pub fn ones_complement(self) -> &'a mut W {
        self.variant(CPL_A::ONESCOMPLEMENT)
    }
    #[doc = "2’s complement representation"]
    #[inline(always)]
    pub fn twos_complement(self) -> &'a mut W {
        self.variant(CPL_A::TWOSCOMPLEMENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Companding mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_A {
    #[doc = "0: No companding algorithm"]
    NOCOMPANDING = 0,
    #[doc = "2: μ-Law algorithm"]
    MULAW = 2,
    #[doc = "3: A-Law algorithm"]
    ALAW = 3,
}
impl From<COMP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP` reader - Companding mode"]
pub struct COMP_R(crate::FieldReader<u8, COMP_A>);
impl COMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP_A> {
        match self.bits {
            0 => Some(COMP_A::NOCOMPANDING),
            2 => Some(COMP_A::MULAW),
            3 => Some(COMP_A::ALAW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOCOMPANDING`"]
    #[inline(always)]
    pub fn is_no_companding(&self) -> bool {
        **self == COMP_A::NOCOMPANDING
    }
    #[doc = "Checks if the value of the field is `MULAW`"]
    #[inline(always)]
    pub fn is_mu_law(&self) -> bool {
        **self == COMP_A::MULAW
    }
    #[doc = "Checks if the value of the field is `ALAW`"]
    #[inline(always)]
    pub fn is_alaw(&self) -> bool {
        **self == COMP_A::ALAW
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<u8, COMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - Companding mode"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No companding algorithm"]
    #[inline(always)]
    pub fn no_companding(self) -> &'a mut W {
        self.variant(COMP_A::NOCOMPANDING)
    }
    #[doc = "μ-Law algorithm"]
    #[inline(always)]
    pub fn mu_law(self) -> &'a mut W {
        self.variant(COMP_A::MULAW)
    }
    #[doc = "A-Law algorithm"]
    #[inline(always)]
    pub fn alaw(self) -> &'a mut W {
        self.variant(COMP_A::ALAW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FIFO threshold"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tristate management on data line"]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mute"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mute value"]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:12 - Mute counter"]
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Complement bit"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Companding mode"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO threshold"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
    #[doc = "Bit 3 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W { w: self }
    }
    #[doc = "Bit 4 - Tristate management on data line"]
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W {
        TRIS_W { w: self }
    }
    #[doc = "Bit 5 - Mute"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Bit 6 - Mute value"]
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W {
        MUTEVAL_W { w: self }
    }
    #[doc = "Bits 7:12 - Mute counter"]
    #[inline(always)]
    pub fn mutecnt(&mut self) -> MUTECNT_W {
        MUTECNT_W { w: self }
    }
    #[doc = "Bit 13 - Complement bit"]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W { w: self }
    }
    #[doc = "Bits 14:15 - Companding mode"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI AConfiguration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0x40"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
