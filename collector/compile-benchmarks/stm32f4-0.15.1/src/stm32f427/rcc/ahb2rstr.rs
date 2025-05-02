#[doc = "Register `AHB2RSTR` reader"]
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2RSTR` writer"]
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG FS module reset"]
pub use DCMIRST_A as OTGFSRST_A;
#[doc = "Random number generator module reset"]
pub use DCMIRST_A as RNGRST_A;
#[doc = "Hash module reset"]
pub use DCMIRST_A as HSAHRST_A;
#[doc = "Cryptographic module reset"]
pub use DCMIRST_A as CRYPRST_A;
#[doc = "Field `OTGFSRST` reader - USB OTG FS module reset"]
pub use DCMIRST_R as OTGFSRST_R;
#[doc = "Field `RNGRST` reader - Random number generator module reset"]
pub use DCMIRST_R as RNGRST_R;
#[doc = "Field `HSAHRST` reader - Hash module reset"]
pub use DCMIRST_R as HSAHRST_R;
#[doc = "Field `CRYPRST` reader - Cryptographic module reset"]
pub use DCMIRST_R as CRYPRST_R;
#[doc = "Field `OTGFSRST` writer - USB OTG FS module reset"]
pub use DCMIRST_W as OTGFSRST_W;
#[doc = "Field `RNGRST` writer - Random number generator module reset"]
pub use DCMIRST_W as RNGRST_W;
#[doc = "Field `HSAHRST` writer - Hash module reset"]
pub use DCMIRST_W as HSAHRST_W;
#[doc = "Field `CRYPRST` writer - Cryptographic module reset"]
pub use DCMIRST_W as CRYPRST_W;
#[doc = "Camera interface reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMIRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DCMIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMIRST` reader - Camera interface reset"]
pub type DCMIRST_R = crate::BitReader<DCMIRST_A>;
impl DCMIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCMIRST_A> {
        match self.bits {
            true => Some(DCMIRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMIRST_A::Reset
    }
}
#[doc = "Field `DCMIRST` writer - Camera interface reset"]
pub type DCMIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, DCMIRST_A, O>;
impl<'a, const O: u8> DCMIRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCMIRST_A::Reset)
    }
}
impl R {
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash module reset"]
    #[inline(always)]
    pub fn hsahrst(&self) -> HSAHRST_R {
        HSAHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptographic module reset"]
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<7> {
        OTGFSRST_W::new(self)
    }
    #[doc = "Bit 6 - Random number generator module reset"]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W<6> {
        RNGRST_W::new(self)
    }
    #[doc = "Bit 5 - Hash module reset"]
    #[inline(always)]
    pub fn hsahrst(&mut self) -> HSAHRST_W<5> {
        HSAHRST_W::new(self)
    }
    #[doc = "Bit 4 - Cryptographic module reset"]
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W<4> {
        CRYPRST_W::new(self)
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W<0> {
        DCMIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2rstr](index.html) module"]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2rstr::R](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
