#[doc = "Register `MMCCR` reader"]
pub struct R(crate::R<MMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCCR` writer"]
pub struct W(crate::W<MMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCCR_SPEC>;
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
impl From<crate::W<MMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - CR"]
pub type CR_R = crate::BitReader<bool>;
#[doc = "Field `CR` writer - CR"]
pub type CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
#[doc = "Field `CSR` reader - CSR"]
pub type CSR_R = crate::BitReader<bool>;
#[doc = "Field `CSR` writer - CSR"]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
#[doc = "Field `ROR` reader - ROR"]
pub type ROR_R = crate::BitReader<bool>;
#[doc = "Field `ROR` writer - ROR"]
pub type ROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
#[doc = "Field `MCF` reader - MCF"]
pub type MCF_R = crate::BitReader<bool>;
#[doc = "Field `MCF` writer - MCF"]
pub type MCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
#[doc = "Field `MCP` reader - MCP"]
pub type MCP_R = crate::BitReader<bool>;
#[doc = "Field `MCP` writer - MCP"]
pub type MCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
#[doc = "Field `MCFHP` reader - MCFHP"]
pub type MCFHP_R = crate::BitReader<bool>;
#[doc = "Field `MCFHP` writer - MCFHP"]
pub type MCFHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSR"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ROR"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCF"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MCP"]
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MCFHP"]
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    #[doc = "Bit 1 - CSR"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W<1> {
        CSR_W::new(self)
    }
    #[doc = "Bit 2 - ROR"]
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W<2> {
        ROR_W::new(self)
    }
    #[doc = "Bit 3 - MCF"]
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W<3> {
        MCF_W::new(self)
    }
    #[doc = "Bit 4 - MCP"]
    #[inline(always)]
    pub fn mcp(&mut self) -> MCP_W<4> {
        MCP_W::new(self)
    }
    #[doc = "Bit 5 - MCFHP"]
    #[inline(always)]
    pub fn mcfhp(&mut self) -> MCFHP_W<5> {
        MCFHP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmccr](index.html) module"]
pub struct MMCCR_SPEC;
impl crate::RegisterSpec for MMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmccr::R](R) reader structure"]
impl crate::Readable for MMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmccr::W](W) writer structure"]
impl crate::Writable for MMCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MMCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
