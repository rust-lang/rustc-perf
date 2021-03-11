#[doc = "Reader of register MACDBGR"]
pub type R = crate::R<u32, super::MACDBGR>;
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSR`"]
pub type CSR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROR`"]
pub type ROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCF`"]
pub type MCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCP`"]
pub type MCP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCFHP`"]
pub type MCFHP_R = crate::R<bool, bool>;
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
