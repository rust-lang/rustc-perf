#[doc = "Register `TIR` reader"]
pub struct R(crate::R<TIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIR` writer"]
pub struct W(crate::W<TIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIR_SPEC>;
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
impl From<crate::W<TIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STID` reader - STID"]
pub struct STID_R(crate::FieldReader<u16, u16>);
impl STID_R {
    pub(crate) fn new(bits: u16) -> Self {
        STID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STID` writer - STID"]
pub struct STID_W<'a> {
    w: &'a mut W,
}
impl<'a> STID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | ((value as u32 & 0x07ff) << 21);
        self.w
    }
}
#[doc = "Field `EXID` reader - EXID"]
pub struct EXID_R(crate::FieldReader<u32, u32>);
impl EXID_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXID` writer - EXID"]
pub struct EXID_W<'a> {
    w: &'a mut W,
}
impl<'a> EXID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 3)) | ((value as u32 & 0x0003_ffff) << 3);
        self.w
    }
}
#[doc = "IDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    #[doc = "0: Standard identifier"]
    STANDARD = 0,
    #[doc = "1: Extended identifier"]
    EXTENDED = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - IDE"]
pub struct IDE_R(crate::FieldReader<bool, IDE_A>);
impl IDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::STANDARD,
            true => IDE_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == IDE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        **self == IDE_A::EXTENDED
    }
}
impl core::ops::Deref for IDE_R {
    type Target = crate::FieldReader<bool, IDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDE` writer - IDE"]
pub struct IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard identifier"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(IDE_A::STANDARD)
    }
    #[doc = "Extended identifier"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(IDE_A::EXTENDED)
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
#[doc = "RTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    #[doc = "0: Data frame"]
    DATA = 0,
    #[doc = "1: Remote frame"]
    REMOTE = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub struct RTR_R(crate::FieldReader<bool, RTR_A>);
impl RTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::DATA,
            true => RTR_A::REMOTE,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        **self == RTR_A::DATA
    }
    #[doc = "Checks if the value of the field is `REMOTE`"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        **self == RTR_A::REMOTE
    }
}
impl core::ops::Deref for RTR_R {
    type Target = crate::FieldReader<bool, RTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTR` writer - RTR"]
pub struct RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data frame"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(RTR_A::DATA)
    }
    #[doc = "Remote frame"]
    #[inline(always)]
    pub fn remote(self) -> &'a mut W {
        self.variant(RTR_A::REMOTE)
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
#[doc = "Field `TXRQ` reader - TXRQ"]
pub struct TXRQ_R(crate::FieldReader<bool, bool>);
impl TXRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRQ` writer - TXRQ"]
pub struct TXRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQ_W<'a> {
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
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&mut self) -> STID_W {
        STID_W { w: self }
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&mut self) -> EXID_W {
        EXID_W { w: self }
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W { w: self }
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W {
        RTR_W { w: self }
    }
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W {
        TXRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tir](index.html) module"]
pub struct TIR_SPEC;
impl crate::RegisterSpec for TIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tir::R](R) reader structure"]
impl crate::Readable for TIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tir::W](W) writer structure"]
impl crate::Writable for TIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIR to value 0"]
impl crate::Resettable for TIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
