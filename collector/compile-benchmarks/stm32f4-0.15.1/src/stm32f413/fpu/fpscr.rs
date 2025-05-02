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
pub type IOC_R = crate::BitReader<bool>;
#[doc = "Field `IOC` writer - Invalid operation cumulative exception bit"]
pub type IOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `DZC` reader - Division by zero cumulative exception bit."]
pub type DZC_R = crate::BitReader<bool>;
#[doc = "Field `DZC` writer - Division by zero cumulative exception bit."]
pub type DZC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `OFC` reader - Overflow cumulative exception bit"]
pub type OFC_R = crate::BitReader<bool>;
#[doc = "Field `OFC` writer - Overflow cumulative exception bit"]
pub type OFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `UFC` reader - Underflow cumulative exception bit"]
pub type UFC_R = crate::BitReader<bool>;
#[doc = "Field `UFC` writer - Underflow cumulative exception bit"]
pub type UFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `IXC` reader - Inexact cumulative exception bit"]
pub type IXC_R = crate::BitReader<bool>;
#[doc = "Field `IXC` writer - Inexact cumulative exception bit"]
pub type IXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `IDC` reader - Input denormal cumulative exception bit."]
pub type IDC_R = crate::BitReader<bool>;
#[doc = "Field `IDC` writer - Input denormal cumulative exception bit."]
pub type IDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `RMode` reader - Rounding Mode control field"]
pub type RMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMode` writer - Rounding Mode control field"]
pub type RMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FZ` reader - Flush-to-zero mode control bit:"]
pub type FZ_R = crate::BitReader<bool>;
#[doc = "Field `FZ` writer - Flush-to-zero mode control bit:"]
pub type FZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `DN` reader - Default NaN mode control bit"]
pub type DN_R = crate::BitReader<bool>;
#[doc = "Field `DN` writer - Default NaN mode control bit"]
pub type DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `AHP` reader - Alternative half-precision control bit"]
pub type AHP_R = crate::BitReader<bool>;
#[doc = "Field `AHP` writer - Alternative half-precision control bit"]
pub type AHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `V` reader - Overflow condition code flag"]
pub type V_R = crate::BitReader<bool>;
#[doc = "Field `V` writer - Overflow condition code flag"]
pub type V_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `C` reader - Carry condition code flag"]
pub type C_R = crate::BitReader<bool>;
#[doc = "Field `C` writer - Carry condition code flag"]
pub type C_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `Z` reader - Zero condition code flag"]
pub type Z_R = crate::BitReader<bool>;
#[doc = "Field `Z` writer - Zero condition code flag"]
pub type Z_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
#[doc = "Field `N` reader - Negative condition code flag"]
pub type N_R = crate::BitReader<bool>;
#[doc = "Field `N` writer - Negative condition code flag"]
pub type N_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Invalid operation cumulative exception bit"]
    #[inline(always)]
    pub fn ioc(&self) -> IOC_R {
        IOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Division by zero cumulative exception bit."]
    #[inline(always)]
    pub fn dzc(&self) -> DZC_R {
        DZC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow cumulative exception bit"]
    #[inline(always)]
    pub fn ofc(&self) -> OFC_R {
        OFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underflow cumulative exception bit"]
    #[inline(always)]
    pub fn ufc(&self) -> UFC_R {
        UFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Inexact cumulative exception bit"]
    #[inline(always)]
    pub fn ixc(&self) -> IXC_R {
        IXC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Input denormal cumulative exception bit."]
    #[inline(always)]
    pub fn idc(&self) -> IDC_R {
        IDC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Rounding Mode control field"]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Flush-to-zero mode control bit:"]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Default NaN mode control bit"]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Alternative half-precision control bit"]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Overflow condition code flag"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Carry condition code flag"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Zero condition code flag"]
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Negative condition code flag"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid operation cumulative exception bit"]
    #[inline(always)]
    pub fn ioc(&mut self) -> IOC_W<0> {
        IOC_W::new(self)
    }
    #[doc = "Bit 1 - Division by zero cumulative exception bit."]
    #[inline(always)]
    pub fn dzc(&mut self) -> DZC_W<1> {
        DZC_W::new(self)
    }
    #[doc = "Bit 2 - Overflow cumulative exception bit"]
    #[inline(always)]
    pub fn ofc(&mut self) -> OFC_W<2> {
        OFC_W::new(self)
    }
    #[doc = "Bit 3 - Underflow cumulative exception bit"]
    #[inline(always)]
    pub fn ufc(&mut self) -> UFC_W<3> {
        UFC_W::new(self)
    }
    #[doc = "Bit 4 - Inexact cumulative exception bit"]
    #[inline(always)]
    pub fn ixc(&mut self) -> IXC_W<4> {
        IXC_W::new(self)
    }
    #[doc = "Bit 7 - Input denormal cumulative exception bit."]
    #[inline(always)]
    pub fn idc(&mut self) -> IDC_W<7> {
        IDC_W::new(self)
    }
    #[doc = "Bits 22:23 - Rounding Mode control field"]
    #[inline(always)]
    pub fn rmode(&mut self) -> RMODE_W<22> {
        RMODE_W::new(self)
    }
    #[doc = "Bit 24 - Flush-to-zero mode control bit:"]
    #[inline(always)]
    pub fn fz(&mut self) -> FZ_W<24> {
        FZ_W::new(self)
    }
    #[doc = "Bit 25 - Default NaN mode control bit"]
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W<25> {
        DN_W::new(self)
    }
    #[doc = "Bit 26 - Alternative half-precision control bit"]
    #[inline(always)]
    pub fn ahp(&mut self) -> AHP_W<26> {
        AHP_W::new(self)
    }
    #[doc = "Bit 28 - Overflow condition code flag"]
    #[inline(always)]
    pub fn v(&mut self) -> V_W<28> {
        V_W::new(self)
    }
    #[doc = "Bit 29 - Carry condition code flag"]
    #[inline(always)]
    pub fn c(&mut self) -> C_W<29> {
        C_W::new(self)
    }
    #[doc = "Bit 30 - Zero condition code flag"]
    #[inline(always)]
    pub fn z(&mut self) -> Z_W<30> {
        Z_W::new(self)
    }
    #[doc = "Bit 31 - Negative condition code flag"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W<31> {
        N_W::new(self)
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
