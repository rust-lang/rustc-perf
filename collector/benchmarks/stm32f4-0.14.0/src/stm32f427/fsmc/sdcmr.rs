#[doc = "Register `SDCMR` reader"]
pub struct R(crate::R<SDCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCMR` writer"]
pub struct W(crate::W<SDCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCMR_SPEC>;
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
impl From<crate::W<SDCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_AW {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Clock Configuration Enable"]
    CLOCKCONFIGURATIONENABLE = 1,
    #[doc = "2: PALL (All Bank Precharge) command"]
    PALL = 2,
    #[doc = "3: Auto-refresh command"]
    AUTOREFRESHCOMMAND = 3,
    #[doc = "4: Load Mode Resgier"]
    LOADMODEREGISTER = 4,
    #[doc = "5: Self-refresh command"]
    SELFREFRESHCOMMAND = 5,
    #[doc = "6: Power-down command"]
    POWERDOWNCOMMAND = 6,
}
impl From<MODE_AW> for u8 {
    #[inline(always)]
    fn from(variant: MODE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` writer - Command mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_AW::NORMAL)
    }
    #[doc = "Clock Configuration Enable"]
    #[inline(always)]
    pub fn clock_configuration_enable(self) -> &'a mut W {
        self.variant(MODE_AW::CLOCKCONFIGURATIONENABLE)
    }
    #[doc = "PALL (All Bank Precharge) command"]
    #[inline(always)]
    pub fn pall(self) -> &'a mut W {
        self.variant(MODE_AW::PALL)
    }
    #[doc = "Auto-refresh command"]
    #[inline(always)]
    pub fn auto_refresh_command(self) -> &'a mut W {
        self.variant(MODE_AW::AUTOREFRESHCOMMAND)
    }
    #[doc = "Load Mode Resgier"]
    #[inline(always)]
    pub fn load_mode_register(self) -> &'a mut W {
        self.variant(MODE_AW::LOADMODEREGISTER)
    }
    #[doc = "Self-refresh command"]
    #[inline(always)]
    pub fn self_refresh_command(self) -> &'a mut W {
        self.variant(MODE_AW::SELFREFRESHCOMMAND)
    }
    #[doc = "Power-down command"]
    #[inline(always)]
    pub fn power_down_command(self) -> &'a mut W {
        self.variant(MODE_AW::POWERDOWNCOMMAND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Command target bank 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTB2_AW {
    #[doc = "0: Command not issued to SDRAM Bank 1"]
    NOTISSUED = 0,
    #[doc = "1: Command issued to SDRAM Bank 1"]
    ISSUED = 1,
}
impl From<CTB2_AW> for bool {
    #[inline(always)]
    fn from(variant: CTB2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTB2` writer - Command target bank 2"]
pub struct CTB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTB2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Command not issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut W {
        self.variant(CTB2_AW::NOTISSUED)
    }
    #[doc = "Command issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn issued(self) -> &'a mut W {
        self.variant(CTB2_AW::ISSUED)
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
#[doc = "Command target bank 1"]
pub type CTB1_AW = CTB2_AW;
#[doc = "Field `CTB1` writer - Command target bank 1"]
pub struct CTB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTB1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Command not issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut W {
        self.variant(CTB1_AW::NOTISSUED)
    }
    #[doc = "Command issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn issued(self) -> &'a mut W {
        self.variant(CTB1_AW::ISSUED)
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
#[doc = "Field `NRFS` reader - Number of Auto-refresh"]
pub struct NRFS_R(crate::FieldReader<u8, u8>);
impl NRFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NRFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRFS` writer - Number of Auto-refresh"]
pub struct NRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `MRD` reader - Mode Register definition"]
pub struct MRD_R(crate::FieldReader<u16, u16>);
impl MRD_R {
    pub(crate) fn new(bits: u16) -> Self {
        MRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRD` writer - Mode Register definition"]
pub struct MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 9)) | ((value as u32 & 0x1fff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Command target bank 2"]
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W {
        CTB2_W { w: self }
    }
    #[doc = "Bit 4 - Command target bank 1"]
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W {
        CTB1_W { w: self }
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W {
        NRFS_W { w: self }
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W {
        MRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Command Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcmr](index.html) module"]
pub struct SDCMR_SPEC;
impl crate::RegisterSpec for SDCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcmr::R](R) reader structure"]
impl crate::Readable for SDCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcmr::W](W) writer structure"]
impl crate::Writable for SDCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCMR to value 0"]
impl crate::Resettable for SDCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
