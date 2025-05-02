#[doc = "Register `PUCR` reader"]
pub struct R(crate::R<PUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCR` writer"]
pub struct W(crate::W<PUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_SPEC>;
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
impl From<crate::W<PUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEDL` reader - ULPS Exit on Data Lane"]
pub type UEDL_R = crate::BitReader<bool>;
#[doc = "Field `UEDL` writer - ULPS Exit on Data Lane"]
pub type UEDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_SPEC, bool, O>;
#[doc = "Field `URDL` reader - ULPS Request on Data Lane"]
pub type URDL_R = crate::BitReader<bool>;
#[doc = "Field `URDL` writer - ULPS Request on Data Lane"]
pub type URDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_SPEC, bool, O>;
#[doc = "Field `UECL` reader - ULPS Exit on Clock Lane"]
pub type UECL_R = crate::BitReader<bool>;
#[doc = "Field `UECL` writer - ULPS Exit on Clock Lane"]
pub type UECL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_SPEC, bool, O>;
#[doc = "Field `URCL` reader - ULPS Request on Clock Lane"]
pub type URCL_R = crate::BitReader<bool>;
#[doc = "Field `URCL` writer - ULPS Request on Clock Lane"]
pub type URCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - ULPS Exit on Data Lane"]
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - ULPS Request on Data Lane"]
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - ULPS Exit on Clock Lane"]
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - ULPS Request on Clock Lane"]
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ULPS Exit on Data Lane"]
    #[inline(always)]
    pub fn uedl(&mut self) -> UEDL_W<3> {
        UEDL_W::new(self)
    }
    #[doc = "Bit 2 - ULPS Request on Data Lane"]
    #[inline(always)]
    pub fn urdl(&mut self) -> URDL_W<2> {
        URDL_W::new(self)
    }
    #[doc = "Bit 1 - ULPS Exit on Clock Lane"]
    #[inline(always)]
    pub fn uecl(&mut self) -> UECL_W<1> {
        UECL_W::new(self)
    }
    #[doc = "Bit 0 - ULPS Request on Clock Lane"]
    #[inline(always)]
    pub fn urcl(&mut self) -> URCL_W<0> {
        URCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY ULPS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr](index.html) module"]
pub struct PUCR_SPEC;
impl crate::RegisterSpec for PUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr::R](R) reader structure"]
impl crate::Readable for PUCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr::W](W) writer structure"]
impl crate::Writable for PUCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCR to value 0"]
impl crate::Resettable for PUCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
