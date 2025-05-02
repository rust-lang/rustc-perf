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
    Normal = 0,
    #[doc = "1: Clock Configuration Enable"]
    ClockConfigurationEnable = 1,
    #[doc = "2: PALL (All Bank Precharge) command"]
    Pall = 2,
    #[doc = "3: Auto-refresh command"]
    AutoRefreshCommand = 3,
    #[doc = "4: Load Mode Resgier"]
    LoadModeRegister = 4,
    #[doc = "5: Self-refresh command"]
    SelfRefreshCommand = 5,
    #[doc = "6: Power-down command"]
    PowerDownCommand = 6,
}
impl From<MODE_AW> for u8 {
    #[inline(always)]
    fn from(variant: MODE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` writer - Command mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCMR_SPEC, u8, MODE_AW, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_AW::Normal)
    }
    #[doc = "Clock Configuration Enable"]
    #[inline(always)]
    pub fn clock_configuration_enable(self) -> &'a mut W {
        self.variant(MODE_AW::ClockConfigurationEnable)
    }
    #[doc = "PALL (All Bank Precharge) command"]
    #[inline(always)]
    pub fn pall(self) -> &'a mut W {
        self.variant(MODE_AW::Pall)
    }
    #[doc = "Auto-refresh command"]
    #[inline(always)]
    pub fn auto_refresh_command(self) -> &'a mut W {
        self.variant(MODE_AW::AutoRefreshCommand)
    }
    #[doc = "Load Mode Resgier"]
    #[inline(always)]
    pub fn load_mode_register(self) -> &'a mut W {
        self.variant(MODE_AW::LoadModeRegister)
    }
    #[doc = "Self-refresh command"]
    #[inline(always)]
    pub fn self_refresh_command(self) -> &'a mut W {
        self.variant(MODE_AW::SelfRefreshCommand)
    }
    #[doc = "Power-down command"]
    #[inline(always)]
    pub fn power_down_command(self) -> &'a mut W {
        self.variant(MODE_AW::PowerDownCommand)
    }
}
#[doc = "Command target bank 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTB2_AW {
    #[doc = "0: Command not issued to SDRAM Bank 1"]
    NotIssued = 0,
    #[doc = "1: Command issued to SDRAM Bank 1"]
    Issued = 1,
}
impl From<CTB2_AW> for bool {
    #[inline(always)]
    fn from(variant: CTB2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTB2` writer - Command target bank 2"]
pub type CTB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDCMR_SPEC, CTB2_AW, O>;
impl<'a, const O: u8> CTB2_W<'a, O> {
    #[doc = "Command not issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn not_issued(self) -> &'a mut W {
        self.variant(CTB2_AW::NotIssued)
    }
    #[doc = "Command issued to SDRAM Bank 1"]
    #[inline(always)]
    pub fn issued(self) -> &'a mut W {
        self.variant(CTB2_AW::Issued)
    }
}
#[doc = "Command target bank 1"]
pub use CTB2_AW as CTB1_AW;
#[doc = "Field `CTB1` writer - Command target bank 1"]
pub use CTB2_W as CTB1_W;
#[doc = "Field `NRFS` reader - Number of Auto-refresh"]
pub type NRFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRFS` writer - Number of Auto-refresh"]
pub type NRFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDCMR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MRD` reader - Mode Register definition"]
pub type MRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MRD` writer - Mode Register definition"]
pub type MRD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDCMR_SPEC, u16, u16, 13, O>;
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
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Command target bank 2"]
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W<3> {
        CTB2_W::new(self)
    }
    #[doc = "Bit 4 - Command target bank 1"]
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W<4> {
        CTB1_W::new(self)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W<5> {
        NRFS_W::new(self)
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W<9> {
        MRD_W::new(self)
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
