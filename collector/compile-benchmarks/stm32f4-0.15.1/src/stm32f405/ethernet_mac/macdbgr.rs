#[doc = "Register `MACDBGR` reader"]
pub struct R(crate::R<MACDBGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACDBGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACDBGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACDBGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CR` reader - CR"]
pub type CR_R = crate::BitReader<bool>;
#[doc = "Field `CSR` reader - CSR"]
pub type CSR_R = crate::BitReader<bool>;
#[doc = "Field `ROR` reader - ROR"]
pub type ROR_R = crate::BitReader<bool>;
#[doc = "Field `MCF` reader - MCF"]
pub type MCF_R = crate::BitReader<bool>;
#[doc = "Field `MCP` reader - MCP"]
pub type MCP_R = crate::BitReader<bool>;
#[doc = "Field `MCFHP` reader - MCFHP"]
pub type MCFHP_R = crate::BitReader<bool>;
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
#[doc = "Ethernet MAC debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macdbgr](index.html) module"]
pub struct MACDBGR_SPEC;
impl crate::RegisterSpec for MACDBGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macdbgr::R](R) reader structure"]
impl crate::Readable for MACDBGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACDBGR to value 0"]
impl crate::Resettable for MACDBGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
