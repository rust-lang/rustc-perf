#[doc = "Register `MACA2HR` reader"]
pub struct R(crate::R<MACA2HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA2HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA2HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA2HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA2HR` writer"]
pub struct W(crate::W<MACA2HR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA2HR_SPEC>;
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
impl From<crate::W<MACA2HR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA2HR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA2H` reader - MAC2AH"]
pub struct MACA2H_R(crate::FieldReader<u16, u16>);
impl MACA2H_R {
    pub(crate) fn new(bits: u16) -> Self {
        MACA2H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACA2H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACA2H` writer - MAC2AH"]
pub struct MACA2H_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA2H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MBC` reader - MBC"]
pub struct MBC_R(crate::FieldReader<u8, u8>);
impl MBC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBC` writer - MBC"]
pub struct MBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "SA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_A {
    #[doc = "0: This address is used for comparison with DA fields of the received frame"]
    DESTINATION = 0,
    #[doc = "1: This address is used for comparison with SA fields of received frames"]
    SOURCE = 1,
}
impl From<SA_A> for bool {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SA` reader - SA"]
pub struct SA_R(crate::FieldReader<bool, SA_A>);
impl SA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SA_A {
        match self.bits {
            false => SA_A::DESTINATION,
            true => SA_A::SOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `DESTINATION`"]
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        **self == SA_A::DESTINATION
    }
    #[doc = "Checks if the value of the field is `SOURCE`"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        **self == SA_A::SOURCE
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<bool, SA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` writer - SA"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    #[inline(always)]
    pub fn destination(self) -> &'a mut W {
        self.variant(SA_A::DESTINATION)
    }
    #[doc = "This address is used for comparison with SA fields of received frames"]
    #[inline(always)]
    pub fn source(self) -> &'a mut W {
        self.variant(SA_A::SOURCE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "AE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AE_A {
    #[doc = "0: Address filters ignore this address"]
    DISABLED = 0,
    #[doc = "1: Address filters use this address"]
    ENABLED = 1,
}
impl From<AE_A> for bool {
    #[inline(always)]
    fn from(variant: AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AE` reader - AE"]
pub struct AE_R(crate::FieldReader<bool, AE_A>);
impl AE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AE_A {
        match self.bits {
            false => AE_A::DISABLED,
            true => AE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AE_A::ENABLED
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, AE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE` writer - AE"]
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address filters ignore this address"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AE_A::DISABLED)
    }
    #[doc = "Address filters use this address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MAC2AH"]
    #[inline(always)]
    pub fn maca2h(&self) -> MACA2H_R {
        MACA2H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC2AH"]
    #[inline(always)]
    pub fn maca2h(&mut self) -> MACA2H_W {
        MACA2H_W { w: self }
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W {
        MBC_W { w: self }
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 2 high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca2hr](index.html) module"]
pub struct MACA2HR_SPEC;
impl crate::RegisterSpec for MACA2HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca2hr::R](R) reader structure"]
impl crate::Readable for MACA2HR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca2hr::W](W) writer structure"]
impl crate::Writable for MACA2HR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA2HR to value 0xffff"]
impl crate::Resettable for MACA2HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
