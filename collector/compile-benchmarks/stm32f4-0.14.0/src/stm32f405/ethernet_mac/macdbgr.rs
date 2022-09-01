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
pub struct CR_R(crate::FieldReader<bool, bool>);
impl CR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR` reader - CSR"]
pub struct CSR_R(crate::FieldReader<bool, bool>);
impl CSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROR` reader - ROR"]
pub struct ROR_R(crate::FieldReader<bool, bool>);
impl ROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCF` reader - MCF"]
pub struct MCF_R(crate::FieldReader<bool, bool>);
impl MCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCP` reader - MCP"]
pub struct MCP_R(crate::FieldReader<bool, bool>);
impl MCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCFHP` reader - MCFHP"]
pub struct MCFHP_R(crate::FieldReader<bool, bool>);
impl MCFHP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCFHP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCFHP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSR"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ROR"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MCF"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MCP"]
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MCFHP"]
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 0x01) != 0)
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
