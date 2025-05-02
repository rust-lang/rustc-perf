#[doc = "Register `DCTRL` reader"]
pub struct R(crate::R<DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTRL` writer"]
pub struct W(crate::W<DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTRL_SPEC>;
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
impl From<crate::W<DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN_A {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Start transfer"]
    Enabled = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN` reader - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
pub type DTEN_R = crate::BitReader<DTEN_A>;
impl DTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::Disabled,
            true => DTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN_A::Enabled
    }
}
#[doc = "Field `DTEN` writer - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DTEN_A, O>;
impl<'a, const O: u8> DTEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN_A::Disabled)
    }
    #[doc = "Start transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN_A::Enabled)
    }
}
#[doc = "Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDIR_A {
    #[doc = "0: From controller to card"]
    ControllerToCard = 0,
    #[doc = "1: From card to controller"]
    CardToController = 1,
}
impl From<DTDIR_A> for bool {
    #[inline(always)]
    fn from(variant: DTDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDIR` reader - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTDIR_R = crate::BitReader<DTDIR_A>;
impl DTDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDIR_A {
        match self.bits {
            false => DTDIR_A::ControllerToCard,
            true => DTDIR_A::CardToController,
        }
    }
    #[doc = "Checks if the value of the field is `ControllerToCard`"]
    #[inline(always)]
    pub fn is_controller_to_card(&self) -> bool {
        *self == DTDIR_A::ControllerToCard
    }
    #[doc = "Checks if the value of the field is `CardToController`"]
    #[inline(always)]
    pub fn is_card_to_controller(&self) -> bool {
        *self == DTDIR_A::CardToController
    }
}
#[doc = "Field `DTDIR` writer - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DTDIR_A, O>;
impl<'a, const O: u8> DTDIR_W<'a, O> {
    #[doc = "From controller to card"]
    #[inline(always)]
    pub fn controller_to_card(self) -> &'a mut W {
        self.variant(DTDIR_A::ControllerToCard)
    }
    #[doc = "From card to controller"]
    #[inline(always)]
    pub fn card_to_controller(self) -> &'a mut W {
        self.variant(DTDIR_A::CardToController)
    }
}
#[doc = "Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTMODE_A {
    #[doc = "0: Bloack data transfer"]
    BlockMode = 0,
    #[doc = "1: Stream or SDIO multibyte data transfer"]
    StreamMode = 1,
}
impl From<DTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTMODE` reader - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTMODE_R = crate::BitReader<DTMODE_A>;
impl DTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTMODE_A {
        match self.bits {
            false => DTMODE_A::BlockMode,
            true => DTMODE_A::StreamMode,
        }
    }
    #[doc = "Checks if the value of the field is `BlockMode`"]
    #[inline(always)]
    pub fn is_block_mode(&self) -> bool {
        *self == DTMODE_A::BlockMode
    }
    #[doc = "Checks if the value of the field is `StreamMode`"]
    #[inline(always)]
    pub fn is_stream_mode(&self) -> bool {
        *self == DTMODE_A::StreamMode
    }
}
#[doc = "Field `DTMODE` writer - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DTMODE_A, O>;
impl<'a, const O: u8> DTMODE_W<'a, O> {
    #[doc = "Bloack data transfer"]
    #[inline(always)]
    pub fn block_mode(self) -> &'a mut W {
        self.variant(DTMODE_A::BlockMode)
    }
    #[doc = "Stream or SDIO multibyte data transfer"]
    #[inline(always)]
    pub fn stream_mode(self) -> &'a mut W {
        self.variant(DTMODE_A::StreamMode)
    }
}
#[doc = "Field `DBLOCKSIZE` reader - Data block size. Define the data block length when the block data transfer mode is selected, block length = 2^(DBLOCKSIZE) bytes"]
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLOCKSIZE` writer - Data block size. Define the data block length when the block data transfer mode is selected, block length = 2^(DBLOCKSIZE) bytes"]
pub type DBLOCKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Read wait start. If this bit is set, read wait operation starts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSTART_A {
    #[doc = "0: Don't start read wait operation"]
    Disabled = 0,
    #[doc = "1: Read wait operation starts"]
    Enabled = 1,
}
impl From<RWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWSTART` reader - Read wait start. If this bit is set, read wait operation starts."]
pub type RWSTART_R = crate::BitReader<RWSTART_A>;
impl RWSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWSTART_A {
        match self.bits {
            false => RWSTART_A::Disabled,
            true => RWSTART_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTART_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTART_A::Enabled
    }
}
#[doc = "Field `RWSTART` writer - Read wait start. If this bit is set, read wait operation starts."]
pub type RWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, RWSTART_A, O>;
impl<'a, const O: u8> RWSTART_W<'a, O> {
    #[doc = "Don't start read wait operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWSTART_A::Disabled)
    }
    #[doc = "Read wait operation starts"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWSTART_A::Enabled)
    }
}
#[doc = "Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSTOP_A {
    #[doc = "0: Read wait in progress if RWSTART is enabled"]
    Disabled = 0,
    #[doc = "1: Enable for read wait stop if RWSTART is enabled"]
    Enabled = 1,
}
impl From<RWSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWSTOP` reader - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
pub type RWSTOP_R = crate::BitReader<RWSTOP_A>;
impl RWSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWSTOP_A {
        match self.bits {
            false => RWSTOP_A::Disabled,
            true => RWSTOP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RWSTOP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RWSTOP_A::Enabled
    }
}
#[doc = "Field `RWSTOP` writer - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
pub type RWSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, RWSTOP_A, O>;
impl<'a, const O: u8> RWSTOP_W<'a, O> {
    #[doc = "Read wait in progress if RWSTART is enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWSTOP_A::Disabled)
    }
    #[doc = "Enable for read wait stop if RWSTART is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWSTOP_A::Enabled)
    }
}
#[doc = "Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWMOD_A {
    #[doc = "0: Read wait control stopping using SDIO_D2"]
    D2 = 0,
    #[doc = "1: Read wait control using SDIO_CK"]
    Ck = 1,
}
impl From<RWMOD_A> for bool {
    #[inline(always)]
    fn from(variant: RWMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWMOD` reader - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type RWMOD_R = crate::BitReader<RWMOD_A>;
impl RWMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWMOD_A {
        match self.bits {
            false => RWMOD_A::D2,
            true => RWMOD_A::Ck,
        }
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == RWMOD_A::D2
    }
    #[doc = "Checks if the value of the field is `Ck`"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == RWMOD_A::Ck
    }
}
#[doc = "Field `RWMOD` writer - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type RWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, RWMOD_A, O>;
impl<'a, const O: u8> RWMOD_W<'a, O> {
    #[doc = "Read wait control stopping using SDIO_D2"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(RWMOD_A::D2)
    }
    #[doc = "Read wait control using SDIO_CK"]
    #[inline(always)]
    pub fn ck(self) -> &'a mut W {
        self.variant(RWMOD_A::Ck)
    }
}
#[doc = "SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOEN_A {
    #[doc = "0: SDIO operations disabled"]
    Disabled = 0,
    #[doc = "1: SDIO operations enabled"]
    Enabled = 1,
}
impl From<SDIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOEN` reader - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
pub type SDIOEN_R = crate::BitReader<SDIOEN_A>;
impl SDIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOEN_A {
        match self.bits {
            false => SDIOEN_A::Disabled,
            true => SDIOEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOEN_A::Enabled
    }
}
#[doc = "Field `SDIOEN` writer - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, SDIOEN_A, O>;
impl<'a, const O: u8> SDIOEN_W<'a, O> {
    #[doc = "SDIO operations disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIOEN_A::Disabled)
    }
    #[doc = "SDIO operations enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIOEN_A::Enabled)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Dma disabled"]
    Disabled = 0,
    #[doc = "1: Dma enabled"]
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "Dma disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "Dma enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Data block size. Define the data block length when the block data transfer mode is selected, block length = 2^(DBLOCKSIZE) bytes"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait start. If this bit is set, read wait operation starts."]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W<1> {
        DTDIR_W::new(self)
    }
    #[doc = "Bit 2 - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W<2> {
        DTMODE_W::new(self)
    }
    #[doc = "Bits 4:7 - Data block size. Define the data block length when the block data transfer mode is selected, block length = 2^(DBLOCKSIZE) bytes"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<4> {
        DBLOCKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Read wait start. If this bit is set, read wait operation starts."]
    #[inline(always)]
    pub fn rwstart(&mut self) -> RWSTART_W<8> {
        RWSTART_W::new(self)
    }
    #[doc = "Bit 9 - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W<9> {
        RWSTOP_W::new(self)
    }
    #[doc = "Bit 10 - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W<10> {
        RWMOD_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](index.html) module"]
pub struct DCTRL_SPEC;
impl crate::RegisterSpec for DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctrl::R](R) reader structure"]
impl crate::Readable for DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctrl::W](W) writer structure"]
impl crate::Writable for DCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
