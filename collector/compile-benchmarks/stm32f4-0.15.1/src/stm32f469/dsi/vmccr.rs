#[doc = "Register `VMCCR` reader"]
pub struct R(crate::R<VMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMCCR` writer"]
pub struct W(crate::W<VMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMCCR_SPEC>;
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
impl From<crate::W<VMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCE` reader - Low-Power Command Enable"]
pub type LPCE_R = crate::BitReader<bool>;
#[doc = "Field `LPCE` writer - Low-Power Command Enable"]
pub type LPCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `FBTAAE` reader - Frame BTA Acknowledge Enable"]
pub type FBTAAE_R = crate::BitReader<bool>;
#[doc = "Field `FBTAAE` writer - Frame BTA Acknowledge Enable"]
pub type FBTAAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPHFE` reader - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFE_R = crate::BitReader<bool>;
#[doc = "Field `LPHFE` writer - Low-Power Horizontal Front-Porch Enable"]
pub type LPHFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPHBPE` reader - Low-power Horizontal Back-Porch Enable"]
pub type LPHBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPHBPE` writer - Low-power Horizontal Back-Porch Enable"]
pub type LPHBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LVAE` reader - Low-Power Vertical Active Enable"]
pub type LVAE_R = crate::BitReader<bool>;
#[doc = "Field `LVAE` writer - Low-Power Vertical Active Enable"]
pub type LVAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPVFPE` reader - Low-power Vertical Front-Porch Enable"]
pub type LPVFPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVFPE` writer - Low-power Vertical Front-Porch Enable"]
pub type LPVFPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPVBPE` reader - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_R = crate::BitReader<bool>;
#[doc = "Field `LPVBPE` writer - Low-power Vertical Back-Porch Enable"]
pub type LPVBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `LPVSAE` reader - Low-Power Vertical Sync time Enable"]
pub type LPVSAE_R = crate::BitReader<bool>;
#[doc = "Field `LPVSAE` writer - Low-Power Vertical Sync time Enable"]
pub type LPVSAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCCR_SPEC, bool, O>;
#[doc = "Field `VMT` reader - Video mode Type"]
pub type VMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VMT` writer - Video mode Type"]
pub type VMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VMCCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&self) -> LPHFE_R {
        LPHFE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&self) -> LVAE_R {
        LVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Low-Power Command Enable"]
    #[inline(always)]
    pub fn lpce(&mut self) -> LPCE_W<15> {
        LPCE_W::new(self)
    }
    #[doc = "Bit 14 - Frame BTA Acknowledge Enable"]
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FBTAAE_W<14> {
        FBTAAE_W::new(self)
    }
    #[doc = "Bit 13 - Low-Power Horizontal Front-Porch Enable"]
    #[inline(always)]
    pub fn lphfe(&mut self) -> LPHFE_W<13> {
        LPHFE_W::new(self)
    }
    #[doc = "Bit 12 - Low-power Horizontal Back-Porch Enable"]
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LPHBPE_W<12> {
        LPHBPE_W::new(self)
    }
    #[doc = "Bit 11 - Low-Power Vertical Active Enable"]
    #[inline(always)]
    pub fn lvae(&mut self) -> LVAE_W<11> {
        LVAE_W::new(self)
    }
    #[doc = "Bit 10 - Low-power Vertical Front-Porch Enable"]
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<10> {
        LPVFPE_W::new(self)
    }
    #[doc = "Bit 9 - Low-power Vertical Back-Porch Enable"]
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<9> {
        LPVBPE_W::new(self)
    }
    #[doc = "Bit 8 - Low-Power Vertical Sync time Enable"]
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LPVSAE_W<8> {
        LPVSAE_W::new(self)
    }
    #[doc = "Bits 0:1 - Video mode Type"]
    #[inline(always)]
    pub fn vmt(&mut self) -> VMT_W<0> {
        VMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video mode Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](index.html) module"]
pub struct VMCCR_SPEC;
impl crate::RegisterSpec for VMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmccr::R](R) reader structure"]
impl crate::Readable for VMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmccr::W](W) writer structure"]
impl crate::Writable for VMCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VMCCR to value 0"]
impl crate::Resettable for VMCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
