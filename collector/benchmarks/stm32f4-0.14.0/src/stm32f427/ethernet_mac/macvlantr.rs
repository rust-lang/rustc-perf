#[doc = "Register `MACVLANTR` reader"]
pub struct R(crate::R<MACVLANTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVLANTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVLANTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVLANTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACVLANTR` writer"]
pub struct W(crate::W<MACVLANTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVLANTR_SPEC>;
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
impl From<crate::W<MACVLANTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVLANTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLANTI` reader - VLAN tag identifier (for receive frames)"]
pub struct VLANTI_R(crate::FieldReader<u16, u16>);
impl VLANTI_R {
    pub(crate) fn new(bits: u16) -> Self {
        VLANTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLANTI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLANTI` writer - VLAN tag identifier (for receive frames)"]
pub struct VLANTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "12-bit VLAN tag comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLANTC_A {
    #[doc = "0: Full 16 bit VLAN identifiers are used for comparison and filtering"]
    VLANTC16 = 0,
    #[doc = "1: 12 bit VLAN identifies are used for comparison and filtering"]
    VLANTC12 = 1,
}
impl From<VLANTC_A> for bool {
    #[inline(always)]
    fn from(variant: VLANTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLANTC` reader - 12-bit VLAN tag comparison"]
pub struct VLANTC_R(crate::FieldReader<bool, VLANTC_A>);
impl VLANTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLANTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLANTC_A {
        match self.bits {
            false => VLANTC_A::VLANTC16,
            true => VLANTC_A::VLANTC12,
        }
    }
    #[doc = "Checks if the value of the field is `VLANTC16`"]
    #[inline(always)]
    pub fn is_vlantc16(&self) -> bool {
        **self == VLANTC_A::VLANTC16
    }
    #[doc = "Checks if the value of the field is `VLANTC12`"]
    #[inline(always)]
    pub fn is_vlantc12(&self) -> bool {
        **self == VLANTC_A::VLANTC12
    }
}
impl core::ops::Deref for VLANTC_R {
    type Target = crate::FieldReader<bool, VLANTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLANTC` writer - 12-bit VLAN tag comparison"]
pub struct VLANTC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLANTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    #[inline(always)]
    pub fn vlantc16(self) -> &'a mut W {
        self.variant(VLANTC_A::VLANTC16)
    }
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    #[inline(always)]
    pub fn vlantc12(self) -> &'a mut W {
        self.variant(VLANTC_A::VLANTC12)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W {
        VLANTI_W { w: self }
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W {
        VLANTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvlantr](index.html) module"]
pub struct MACVLANTR_SPEC;
impl crate::RegisterSpec for MACVLANTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macvlantr::R](R) reader structure"]
impl crate::Readable for MACVLANTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macvlantr::W](W) writer structure"]
impl crate::Writable for MACVLANTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MACVLANTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
