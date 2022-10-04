#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states"]
    WS0 = 0,
    #[doc = "1: 1 wait states"]
    WS1 = 1,
    #[doc = "2: 2 wait states"]
    WS2 = 2,
    #[doc = "3: 3 wait states"]
    WS3 = 3,
    #[doc = "4: 4 wait states"]
    WS4 = 4,
    #[doc = "5: 5 wait states"]
    WS5 = 5,
    #[doc = "6: 6 wait states"]
    WS6 = 6,
    #[doc = "7: 7 wait states"]
    WS7 = 7,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub struct LATENCY_R(crate::FieldReader<u8, LATENCY_A>);
impl LATENCY_R {
    pub(crate) fn new(bits: u8) -> Self {
        LATENCY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            0 => LATENCY_A::WS0,
            1 => LATENCY_A::WS1,
            2 => LATENCY_A::WS2,
            3 => LATENCY_A::WS3,
            4 => LATENCY_A::WS4,
            5 => LATENCY_A::WS5,
            6 => LATENCY_A::WS6,
            7 => LATENCY_A::WS7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        **self == LATENCY_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        **self == LATENCY_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        **self == LATENCY_A::WS2
    }
    #[doc = "Checks if the value of the field is `WS3`"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        **self == LATENCY_A::WS3
    }
    #[doc = "Checks if the value of the field is `WS4`"]
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        **self == LATENCY_A::WS4
    }
    #[doc = "Checks if the value of the field is `WS5`"]
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        **self == LATENCY_A::WS5
    }
    #[doc = "Checks if the value of the field is `WS6`"]
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        **self == LATENCY_A::WS6
    }
    #[doc = "Checks if the value of the field is `WS7`"]
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        **self == LATENCY_A::WS7
    }
}
impl core::ops::Deref for LATENCY_R {
    type Target = crate::FieldReader<u8, LATENCY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LATENCY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::WS0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::WS1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::WS2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(LATENCY_A::WS3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn ws4(self) -> &'a mut W {
        self.variant(LATENCY_A::WS4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn ws5(self) -> &'a mut W {
        self.variant(LATENCY_A::WS5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn ws6(self) -> &'a mut W {
        self.variant(LATENCY_A::WS6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn ws7(self) -> &'a mut W {
        self.variant(LATENCY_A::WS7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTEN_A {
    #[doc = "0: Prefetch is disabled"]
    DISABLED = 0,
    #[doc = "1: Prefetch is enabled"]
    ENABLED = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub struct PRFTEN_R(crate::FieldReader<bool, PRFTEN_A>);
impl PRFTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRFTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::DISABLED,
            true => PRFTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PRFTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PRFTEN_A::ENABLED
    }
}
impl core::ops::Deref for PRFTEN_R {
    type Target = crate::FieldReader<bool, PRFTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRFTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::DISABLED)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Instruction cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEN_A {
    #[doc = "0: Instruction cache is disabled"]
    DISABLED = 0,
    #[doc = "1: Instruction cache is enabled"]
    ENABLED = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub struct ICEN_R(crate::FieldReader<bool, ICEN_A>);
impl ICEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::DISABLED,
            true => ICEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ICEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ICEN_A::ENABLED
    }
}
impl core::ops::Deref for ICEN_R {
    type Target = crate::FieldReader<bool, ICEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICEN_A::DISABLED)
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Data cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN_A {
    #[doc = "0: Data cache is disabled"]
    DISABLED = 0,
    #[doc = "1: Data cache is enabled"]
    ENABLED = 1,
}
impl From<DCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCEN` reader - Data cache enable"]
pub struct DCEN_R(crate::FieldReader<bool, DCEN_A>);
impl DCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCEN_A {
        match self.bits {
            false => DCEN_A::DISABLED,
            true => DCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DCEN_A::ENABLED
    }
}
impl core::ops::Deref for DCEN_R {
    type Target = crate::FieldReader<bool, DCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCEN` writer - Data cache enable"]
pub struct DCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCEN_A::DISABLED)
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICRST_AW {
    #[doc = "0: Instruction cache is not reset"]
    NOTRESET = 0,
    #[doc = "1: Instruction cache is reset"]
    RESET = 1,
}
impl From<ICRST_AW> for bool {
    #[inline(always)]
    fn from(variant: ICRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub struct ICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICRST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ICRST_AW::NOTRESET)
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ICRST_AW::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Data cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCRST_A {
    #[doc = "0: Data cache is not reset"]
    NOTRESET = 0,
    #[doc = "1: Data cache is reset"]
    RESET = 1,
}
impl From<DCRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRST` reader - Data cache reset"]
pub struct DCRST_R(crate::FieldReader<bool, DCRST_A>);
impl DCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCRST_A {
        match self.bits {
            false => DCRST_A::NOTRESET,
            true => DCRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        **self == DCRST_A::NOTRESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == DCRST_A::RESET
    }
}
impl core::ops::Deref for DCRST_R {
    type Target = crate::FieldReader<bool, DCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRST` writer - Data cache reset"]
pub struct DCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(DCRST_A::NOTRESET)
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W {
        DCEN_W { w: self }
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W {
        ICRST_W { w: self }
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W {
        DCRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
