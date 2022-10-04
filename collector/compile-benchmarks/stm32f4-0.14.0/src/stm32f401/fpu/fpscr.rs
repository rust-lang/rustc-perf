#[doc = "Register `FPSCR` reader"]
pub struct R(crate::R<FPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPSCR` writer"]
pub struct W(crate::W<FPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPSCR_SPEC>;
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
impl From<crate::W<FPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOC` reader - Invalid operation cumulative exception bit"]
pub struct IOC_R(crate::FieldReader<bool, bool>);
impl IOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOC` writer - Invalid operation cumulative exception bit"]
pub struct IOC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOC_W<'a> {
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
#[doc = "Field `DZC` reader - Division by zero cumulative exception bit."]
pub struct DZC_R(crate::FieldReader<bool, bool>);
impl DZC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DZC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DZC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DZC` writer - Division by zero cumulative exception bit."]
pub struct DZC_W<'a> {
    w: &'a mut W,
}
impl<'a> DZC_W<'a> {
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
#[doc = "Field `OFC` reader - Overflow cumulative exception bit"]
pub struct OFC_R(crate::FieldReader<bool, bool>);
impl OFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFC` writer - Overflow cumulative exception bit"]
pub struct OFC_W<'a> {
    w: &'a mut W,
}
impl<'a> OFC_W<'a> {
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
#[doc = "Field `UFC` reader - Underflow cumulative exception bit"]
pub struct UFC_R(crate::FieldReader<bool, bool>);
impl UFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        UFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFC` writer - Underflow cumulative exception bit"]
pub struct UFC_W<'a> {
    w: &'a mut W,
}
impl<'a> UFC_W<'a> {
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
#[doc = "Field `IXC` reader - Inexact cumulative exception bit"]
pub struct IXC_R(crate::FieldReader<bool, bool>);
impl IXC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IXC` writer - Inexact cumulative exception bit"]
pub struct IXC_W<'a> {
    w: &'a mut W,
}
impl<'a> IXC_W<'a> {
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
#[doc = "Field `IDC` reader - Input denormal cumulative exception bit."]
pub struct IDC_R(crate::FieldReader<bool, bool>);
impl IDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDC` writer - Input denormal cumulative exception bit."]
pub struct IDC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RMode` reader - Rounding Mode control field"]
pub struct RMODE_R(crate::FieldReader<u8, u8>);
impl RMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMode` writer - Rounding Mode control field"]
pub struct RMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `FZ` reader - Flush-to-zero mode control bit:"]
pub struct FZ_R(crate::FieldReader<bool, bool>);
impl FZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FZ` writer - Flush-to-zero mode control bit:"]
pub struct FZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `DN` reader - Default NaN mode control bit"]
pub struct DN_R(crate::FieldReader<bool, bool>);
impl DN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DN` writer - Default NaN mode control bit"]
pub struct DN_W<'a> {
    w: &'a mut W,
}
impl<'a> DN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `AHP` reader - Alternative half-precision control bit"]
pub struct AHP_R(crate::FieldReader<bool, bool>);
impl AHP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHP` writer - Alternative half-precision control bit"]
pub struct AHP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `V` reader - Overflow condition code flag"]
pub struct V_R(crate::FieldReader<bool, bool>);
impl V_R {
    pub(crate) fn new(bits: bool) -> Self {
        V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V` writer - Overflow condition code flag"]
pub struct V_W<'a> {
    w: &'a mut W,
}
impl<'a> V_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `C` reader - Carry condition code flag"]
pub struct C_R(crate::FieldReader<bool, bool>);
impl C_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C` writer - Carry condition code flag"]
pub struct C_W<'a> {
    w: &'a mut W,
}
impl<'a> C_W<'a> {
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
#[doc = "Field `Z` reader - Zero condition code flag"]
pub struct Z_R(crate::FieldReader<bool, bool>);
impl Z_R {
    pub(crate) fn new(bits: bool) -> Self {
        Z_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Z_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Z` writer - Zero condition code flag"]
pub struct Z_W<'a> {
    w: &'a mut W,
}
impl<'a> Z_W<'a> {
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
#[doc = "Field `N` reader - Negative condition code flag"]
pub struct N_R(crate::FieldReader<bool, bool>);
impl N_R {
    pub(crate) fn new(bits: bool) -> Self {
        N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `N` writer - Negative condition code flag"]
pub struct N_W<'a> {
    w: &'a mut W,
}
impl<'a> N_W<'a> {
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
    #[doc = "Bit 0 - Invalid operation cumulative exception bit"]
    #[inline(always)]
    pub fn ioc(&self) -> IOC_R {
        IOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Division by zero cumulative exception bit."]
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overflow cumulative exception bit"]
    #[inline(always)]
    pub fn ofc(&self) -> OFC_R {
        OFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Underflow cumulative exception bit"]
    #[inline(always)]
    pub fn ufc(&self) -> UFC_R {
        UFC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Inexact cumulative exception bit"]
    #[inline(always)]
    pub fn ixc(&self) -> IXC_R {
        IXC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input denormal cumulative exception bit."]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Rounding Mode control field"]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Flush-to-zero mode control bit:"]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Default NaN mode control bit"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Alternative half-precision control bit"]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Overflow condition code flag"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Carry condition code flag"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Zero condition code flag"]
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Negative condition code flag"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid operation cumulative exception bit"]
    #[inline(always)]
    pub fn ioc(&mut self) -> IOC_W {
        IOC_W { w: self }
    }
    #[doc = "Bit 1 - Division by zero cumulative exception bit."]
    #[inline(always)]
    pub fn dzc(&mut self) -> DZC_W {
        DZC_W { w: self }
    }
    #[doc = "Bit 2 - Overflow cumulative exception bit"]
    #[inline(always)]
    pub fn ofc(&mut self) -> OFC_W {
        OFC_W { w: self }
    }
    #[doc = "Bit 3 - Underflow cumulative exception bit"]
    #[inline(always)]
    pub fn ufc(&mut self) -> UFC_W {
        UFC_W { w: self }
    }
    #[doc = "Bit 4 - Inexact cumulative exception bit"]
    #[inline(always)]
    pub fn ixc(&mut self) -> IXC_W {
        IXC_W { w: self }
    }
    #[doc = "Bit 7 - Input denormal cumulative exception bit."]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W {
        IDC_W { w: self }
    }
    #[doc = "Bits 22:23 - Rounding Mode control field"]
    #[inline(always)]
    pub fn rmode(&mut self) -> RMODE_W {
        RMODE_W { w: self }
    }
    #[doc = "Bit 24 - Flush-to-zero mode control bit:"]
    #[inline(always)]
    pub fn fz(&mut self) -> FZ_W {
        FZ_W { w: self }
    }
    #[doc = "Bit 25 - Default NaN mode control bit"]
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W {
        DN_W { w: self }
    }
    #[doc = "Bit 26 - Alternative half-precision control bit"]
    #[inline(always)]
    pub fn ahp(&mut self) -> AHP_W {
        AHP_W { w: self }
    }
    #[doc = "Bit 28 - Overflow condition code flag"]
    #[inline(always)]
    pub fn v(&mut self) -> V_W {
        V_W { w: self }
    }
    #[doc = "Bit 29 - Carry condition code flag"]
    #[inline(always)]
    pub fn c(&mut self) -> C_W {
        C_W { w: self }
    }
    #[doc = "Bit 30 - Zero condition code flag"]
    #[inline(always)]
    pub fn z(&mut self) -> Z_W {
        Z_W { w: self }
    }
    #[doc = "Bit 31 - Negative condition code flag"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W {
        N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating-point status control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpscr](index.html) module"]
pub struct FPSCR_SPEC;
impl crate::RegisterSpec for FPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpscr::R](R) reader structure"]
impl crate::Readable for FPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpscr::W](W) writer structure"]
impl crate::Writable for FPSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPSCR to value 0"]
impl crate::Resettable for FPSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
