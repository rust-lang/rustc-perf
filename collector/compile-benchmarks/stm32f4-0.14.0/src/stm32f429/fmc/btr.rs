#[doc = "Register `BTR%s` reader"]
pub struct R(crate::R<BTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR%s` writer"]
pub struct W(crate::W<BTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR_SPEC>;
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
impl From<crate::W<BTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACCMOD\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACCMOD_A {
    #[doc = "0: Access mode A"]
    A = 0,
    #[doc = "1: Access mode B"]
    B = 1,
    #[doc = "2: Access mode C"]
    C = 2,
    #[doc = "3: Access mode D"]
    D = 3,
}
impl From<ACCMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACCMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACCMOD` reader - ACCMOD"]
pub struct ACCMOD_R(crate::FieldReader<u8, ACCMOD_A>);
impl ACCMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACCMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCMOD_A {
        match self.bits {
            0 => ACCMOD_A::A,
            1 => ACCMOD_A::B,
            2 => ACCMOD_A::C,
            3 => ACCMOD_A::D,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        **self == ACCMOD_A::A
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        **self == ACCMOD_A::B
    }
    #[doc = "Checks if the value of the field is `C`"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        **self == ACCMOD_A::C
    }
    #[doc = "Checks if the value of the field is `D`"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        **self == ACCMOD_A::D
    }
}
impl core::ops::Deref for ACCMOD_R {
    type Target = crate::FieldReader<u8, ACCMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCMOD` writer - ACCMOD"]
pub struct ACCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Access mode A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(ACCMOD_A::A)
    }
    #[doc = "Access mode B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(ACCMOD_A::B)
    }
    #[doc = "Access mode C"]
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(ACCMOD_A::C)
    }
    #[doc = "Access mode D"]
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(ACCMOD_A::D)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `DATLAT` reader - DATLAT"]
pub struct DATLAT_R(crate::FieldReader<u8, u8>);
impl DATLAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLAT` writer - DATLAT"]
pub struct DATLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `BUSTURN` reader - BUSTURN"]
pub struct BUSTURN_R(crate::FieldReader<u8, u8>);
impl BUSTURN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUSTURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSTURN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTURN` writer - BUSTURN"]
pub struct BUSTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTURN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DATAST` reader - DATAST"]
pub struct DATAST_R(crate::FieldReader<u8, u8>);
impl DATAST_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAST` writer - DATAST"]
pub struct DATAST_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ADDHLD` reader - ADDHLD"]
pub struct ADDHLD_R(crate::FieldReader<u8, u8>);
impl ADDHLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDHLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDHLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDHLD` writer - ADDHLD"]
pub struct ADDHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ADDSET` reader - ADDSET"]
pub struct ADDSET_R(crate::FieldReader<u8, u8>);
impl ADDSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDSET` writer - ADDSET"]
pub struct ADDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W {
        ACCMOD_W { w: self }
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W {
        DATLAT_W { w: self }
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W {
        BUSTURN_W { w: self }
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W {
        DATAST_W { w: self }
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W {
        ADDHLD_W { w: self }
    }
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W {
        ADDSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR-Flash chip-select timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](index.html) module"]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr::R](R) reader structure"]
impl crate::Readable for BTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr::W](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTR%s to value 0xffff_ffff"]
impl crate::Resettable for BTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
