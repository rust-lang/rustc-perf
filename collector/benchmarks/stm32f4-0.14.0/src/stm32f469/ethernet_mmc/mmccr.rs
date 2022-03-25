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
#[doc = "Counter reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CR_A {
    #[doc = "1: Reset all counters. Cleared automatically"]
    RESET = 1,
}
impl From<CR_A> for bool {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CR` reader - Counter reset"]
pub struct CR_R(crate::FieldReader<bool, CR_A>);
impl CR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CR_A> {
        match self.bits {
            true => Some(CR_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == CR_A::RESET
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<bool, CR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - Counter reset"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset all counters. Cleared automatically"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CR_A::RESET)
    }
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
#[doc = "Counter stop rollover\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR_A {
    #[doc = "0: Counters roll over to zero after reaching the maximum value"]
    DISABLED = 0,
    #[doc = "1: Counters do not roll over to zero after reaching the maximum value"]
    ENABLED = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub struct CSR_R(crate::FieldReader<bool, CSR_A>);
impl CSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::DISABLED,
            true => CSR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CSR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CSR_A::ENABLED
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<bool, CSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counters roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSR_A::DISABLED)
    }
    #[doc = "Counters do not roll over to zero after reaching the maximum value"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSR_A::ENABLED)
    }
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
#[doc = "Reset on read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROR_A {
    #[doc = "0: MMC counters do not reset on read"]
    DISABLED = 0,
    #[doc = "1: MMC counters reset to zero after read"]
    ENABLED = 1,
}
impl From<ROR_A> for bool {
    #[inline(always)]
    fn from(variant: ROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROR` reader - Reset on read"]
pub struct ROR_R(crate::FieldReader<bool, ROR_A>);
impl ROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROR_A {
        match self.bits {
            false => ROR_A::DISABLED,
            true => ROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ROR_A::ENABLED
    }
}
impl core::ops::Deref for ROR_R {
    type Target = crate::FieldReader<bool, ROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROR` writer - Reset on read"]
pub struct ROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MMC counters do not reset on read"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROR_A::DISABLED)
    }
    #[doc = "MMC counters reset to zero after read"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROR_A::ENABLED)
    }
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
#[doc = "MMC counter freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCF_A {
    #[doc = "0: All MMC counters update normally"]
    UNFROZEN = 0,
    #[doc = "1: All MMC counters frozen to their current value"]
    FROZEN = 1,
}
impl From<MCF_A> for bool {
    #[inline(always)]
    fn from(variant: MCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub struct MCF_R(crate::FieldReader<bool, MCF_A>);
impl MCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCF_A {
        match self.bits {
            false => MCF_A::UNFROZEN,
            true => MCF_A::FROZEN,
        }
    }
    #[doc = "Checks if the value of the field is `UNFROZEN`"]
    #[inline(always)]
    pub fn is_unfrozen(&self) -> bool {
        **self == MCF_A::UNFROZEN
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == MCF_A::FROZEN
    }
}
impl core::ops::Deref for MCF_R {
    type Target = crate::FieldReader<bool, MCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub struct MCF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All MMC counters update normally"]
    #[inline(always)]
    pub fn unfrozen(self) -> &'a mut W {
        self.variant(MCF_A::UNFROZEN)
    }
    #[doc = "All MMC counters frozen to their current value"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(MCF_A::FROZEN)
    }
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
#[doc = "MMC counter preset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCP_A {
    #[doc = "1: MMC counters will be preset to almost full or almost half. Cleared automatically"]
    PRESET = 1,
}
impl From<MCP_A> for bool {
    #[inline(always)]
    fn from(variant: MCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCP` reader - MMC counter preset"]
pub struct MCP_R(crate::FieldReader<bool, MCP_A>);
impl MCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCP_A> {
        match self.bits {
            true => Some(MCP_A::PRESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESET`"]
    #[inline(always)]
    pub fn is_preset(&self) -> bool {
        **self == MCP_A::PRESET
    }
}
impl core::ops::Deref for MCP_R {
    type Target = crate::FieldReader<bool, MCP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCP` writer - MMC counter preset"]
pub struct MCP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MMC counters will be preset to almost full or almost half. Cleared automatically"]
    #[inline(always)]
    pub fn preset(self) -> &'a mut W {
        self.variant(MCP_A::PRESET)
    }
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
#[doc = "MMC counter Full-Half preset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCFHP_A {
    #[doc = "0: When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    ALMOSTHALF = 0,
    #[doc = "1: When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    ALMOSTFULL = 1,
}
impl From<MCFHP_A> for bool {
    #[inline(always)]
    fn from(variant: MCFHP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCFHP` reader - MMC counter Full-Half preset"]
pub struct MCFHP_R(crate::FieldReader<bool, MCFHP_A>);
impl MCFHP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCFHP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCFHP_A {
        match self.bits {
            false => MCFHP_A::ALMOSTHALF,
            true => MCFHP_A::ALMOSTFULL,
        }
    }
    #[doc = "Checks if the value of the field is `ALMOSTHALF`"]
    #[inline(always)]
    pub fn is_almost_half(&self) -> bool {
        **self == MCFHP_A::ALMOSTHALF
    }
    #[doc = "Checks if the value of the field is `ALMOSTFULL`"]
    #[inline(always)]
    pub fn is_almost_full(&self) -> bool {
        **self == MCFHP_A::ALMOSTFULL
    }
}
impl core::ops::Deref for MCFHP_R {
    type Target = crate::FieldReader<bool, MCFHP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCFHP` writer - MMC counter Full-Half preset"]
pub struct MCFHP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCFHP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCFHP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-half value 0x7FFF_FFF0"]
    #[inline(always)]
    pub fn almost_half(self) -> &'a mut W {
        self.variant(MCFHP_A::ALMOSTHALF)
    }
    #[doc = "When MCP is set, MMC counters are preset to almost-full value 0xFFFF_FFF0"]
    #[inline(always)]
    pub fn almost_full(self) -> &'a mut W {
        self.variant(MCFHP_A::ALMOSTFULL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC counter preset"]
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC counter Full-Half preset"]
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W {
        ROR_W { w: self }
    }
    #[doc = "Bit 3 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W {
        MCF_W { w: self }
    }
    #[doc = "Bit 4 - MMC counter preset"]
    #[inline(always)]
    pub fn mcp(&mut self) -> MCP_W {
        MCP_W { w: self }
    }
    #[doc = "Bit 5 - MMC counter Full-Half preset"]
    #[inline(always)]
    pub fn mcfhp(&mut self) -> MCFHP_W {
        MCFHP_W { w: self }
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
