#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 9 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP9_A {
    #[doc = "0: 3 cycles"]
    CYCLES3 = 0,
    #[doc = "1: 15 cycles"]
    CYCLES15 = 1,
    #[doc = "2: 28 cycles"]
    CYCLES28 = 2,
    #[doc = "3: 56 cycles"]
    CYCLES56 = 3,
    #[doc = "4: 84 cycles"]
    CYCLES84 = 4,
    #[doc = "5: 112 cycles"]
    CYCLES112 = 5,
    #[doc = "6: 144 cycles"]
    CYCLES144 = 6,
    #[doc = "7: 480 cycles"]
    CYCLES480 = 7,
}
impl From<SMP9_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP9` reader - Channel 9 sampling time selection"]
pub struct SMP9_R(crate::FieldReader<u8, SMP9_A>);
impl SMP9_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP9_A {
        match self.bits {
            0 => SMP9_A::CYCLES3,
            1 => SMP9_A::CYCLES15,
            2 => SMP9_A::CYCLES28,
            3 => SMP9_A::CYCLES56,
            4 => SMP9_A::CYCLES84,
            5 => SMP9_A::CYCLES112,
            6 => SMP9_A::CYCLES144,
            7 => SMP9_A::CYCLES480,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES3`"]
    #[inline(always)]
    pub fn is_cycles3(&self) -> bool {
        **self == SMP9_A::CYCLES3
    }
    #[doc = "Checks if the value of the field is `CYCLES15`"]
    #[inline(always)]
    pub fn is_cycles15(&self) -> bool {
        **self == SMP9_A::CYCLES15
    }
    #[doc = "Checks if the value of the field is `CYCLES28`"]
    #[inline(always)]
    pub fn is_cycles28(&self) -> bool {
        **self == SMP9_A::CYCLES28
    }
    #[doc = "Checks if the value of the field is `CYCLES56`"]
    #[inline(always)]
    pub fn is_cycles56(&self) -> bool {
        **self == SMP9_A::CYCLES56
    }
    #[doc = "Checks if the value of the field is `CYCLES84`"]
    #[inline(always)]
    pub fn is_cycles84(&self) -> bool {
        **self == SMP9_A::CYCLES84
    }
    #[doc = "Checks if the value of the field is `CYCLES112`"]
    #[inline(always)]
    pub fn is_cycles112(&self) -> bool {
        **self == SMP9_A::CYCLES112
    }
    #[doc = "Checks if the value of the field is `CYCLES144`"]
    #[inline(always)]
    pub fn is_cycles144(&self) -> bool {
        **self == SMP9_A::CYCLES144
    }
    #[doc = "Checks if the value of the field is `CYCLES480`"]
    #[inline(always)]
    pub fn is_cycles480(&self) -> bool {
        **self == SMP9_A::CYCLES480
    }
}
impl core::ops::Deref for SMP9_R {
    type Target = crate::FieldReader<u8, SMP9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP9` writer - Channel 9 sampling time selection"]
pub struct SMP9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn cycles3(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES3)
    }
    #[doc = "15 cycles"]
    #[inline(always)]
    pub fn cycles15(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES15)
    }
    #[doc = "28 cycles"]
    #[inline(always)]
    pub fn cycles28(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES28)
    }
    #[doc = "56 cycles"]
    #[inline(always)]
    pub fn cycles56(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES56)
    }
    #[doc = "84 cycles"]
    #[inline(always)]
    pub fn cycles84(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES84)
    }
    #[doc = "112 cycles"]
    #[inline(always)]
    pub fn cycles112(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES112)
    }
    #[doc = "144 cycles"]
    #[inline(always)]
    pub fn cycles144(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES144)
    }
    #[doc = "480 cycles"]
    #[inline(always)]
    pub fn cycles480(self) -> &'a mut W {
        self.variant(SMP9_A::CYCLES480)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Field `SMP8` reader - Channel 8 sampling time selection"]
pub struct SMP8_R(crate::FieldReader<u8, u8>);
impl SMP8_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP8` writer - Channel 8 sampling time selection"]
pub struct SMP8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `SMP7` reader - Channel 7 sampling time selection"]
pub struct SMP7_R(crate::FieldReader<u8, u8>);
impl SMP7_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP7` writer - Channel 7 sampling time selection"]
pub struct SMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SMP6` reader - Channel 6 sampling time selection"]
pub struct SMP6_R(crate::FieldReader<u8, u8>);
impl SMP6_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP6` writer - Channel 6 sampling time selection"]
pub struct SMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `SMP5` reader - Channel 5 sampling time selection"]
pub struct SMP5_R(crate::FieldReader<u8, u8>);
impl SMP5_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP5` writer - Channel 5 sampling time selection"]
pub struct SMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SMP4` reader - Channel 4 sampling time selection"]
pub struct SMP4_R(crate::FieldReader<u8, u8>);
impl SMP4_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP4` writer - Channel 4 sampling time selection"]
pub struct SMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SMP3` reader - Channel 3 sampling time selection"]
pub struct SMP3_R(crate::FieldReader<u8, u8>);
impl SMP3_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP3` writer - Channel 3 sampling time selection"]
pub struct SMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `SMP2` reader - Channel 2 sampling time selection"]
pub struct SMP2_R(crate::FieldReader<u8, u8>);
impl SMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP2` writer - Channel 2 sampling time selection"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `SMP1` reader - Channel 1 sampling time selection"]
pub struct SMP1_R(crate::FieldReader<u8, u8>);
impl SMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP1` writer - Channel 1 sampling time selection"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `SMP0` reader - Channel 0 sampling time selection"]
pub struct SMP0_R(crate::FieldReader<u8, u8>);
impl SMP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP0` writer - Channel 0 sampling time selection"]
pub struct SMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W {
        SMP9_W { w: self }
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W {
        SMP8_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W {
        SMP7_W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W {
        SMP6_W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W {
        SMP5_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W {
        SMP4_W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W {
        SMP3_W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP0_W {
        SMP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
