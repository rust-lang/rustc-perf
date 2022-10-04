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
#[doc = "SD I/O enable functions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOEN_A {
    #[doc = "0: SDIO operations disabled"]
    DISABLED = 0,
    #[doc = "1: SDIO operations enabled"]
    ENABLED = 1,
}
impl From<SDIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOEN` reader - SD I/O enable functions"]
pub struct SDIOEN_R(crate::FieldReader<bool, SDIOEN_A>);
impl SDIOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOEN_A {
        match self.bits {
            false => SDIOEN_A::DISABLED,
            true => SDIOEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SDIOEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SDIOEN_A::ENABLED
    }
}
impl core::ops::Deref for SDIOEN_R {
    type Target = crate::FieldReader<bool, SDIOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOEN` writer - SD I/O enable functions"]
pub struct SDIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SDIO operations disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIOEN_A::DISABLED)
    }
    #[doc = "SDIO operations enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIOEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Read wait mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWMOD_A {
    #[doc = "0: Read wait control stopping using SDIO_D2"]
    D2 = 0,
    #[doc = "1: Read wait control using SDIO_CK"]
    CK = 1,
}
impl From<RWMOD_A> for bool {
    #[inline(always)]
    fn from(variant: RWMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWMOD` reader - Read wait mode"]
pub struct RWMOD_R(crate::FieldReader<bool, RWMOD_A>);
impl RWMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWMOD_A {
        match self.bits {
            false => RWMOD_A::D2,
            true => RWMOD_A::CK,
        }
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        **self == RWMOD_A::D2
    }
    #[doc = "Checks if the value of the field is `CK`"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        **self == RWMOD_A::CK
    }
}
impl core::ops::Deref for RWMOD_R {
    type Target = crate::FieldReader<bool, RWMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWMOD` writer - Read wait mode"]
pub struct RWMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RWMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read wait control stopping using SDIO_D2"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(RWMOD_A::D2)
    }
    #[doc = "Read wait control using SDIO_CK"]
    #[inline(always)]
    pub fn ck(self) -> &'a mut W {
        self.variant(RWMOD_A::CK)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Read wait stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSTOP_A {
    #[doc = "0: Read wait in progress if RWSTART is enabled"]
    DISABLED = 0,
    #[doc = "1: Enable for read wait stop if RWSTART is enabled"]
    ENABLED = 1,
}
impl From<RWSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWSTOP` reader - Read wait stop"]
pub struct RWSTOP_R(crate::FieldReader<bool, RWSTOP_A>);
impl RWSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWSTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWSTOP_A {
        match self.bits {
            false => RWSTOP_A::DISABLED,
            true => RWSTOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RWSTOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RWSTOP_A::ENABLED
    }
}
impl core::ops::Deref for RWSTOP_R {
    type Target = crate::FieldReader<bool, RWSTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWSTOP` writer - Read wait stop"]
pub struct RWSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RWSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWSTOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read wait in progress if RWSTART is enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWSTOP_A::DISABLED)
    }
    #[doc = "Enable for read wait stop if RWSTART is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWSTOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Read wait start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWSTART_A {
    #[doc = "0: Don't start read wait operation"]
    DISABLED = 0,
    #[doc = "1: Read wait operation starts"]
    ENABLED = 1,
}
impl From<RWSTART_A> for bool {
    #[inline(always)]
    fn from(variant: RWSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWSTART` reader - Read wait start"]
pub struct RWSTART_R(crate::FieldReader<bool, RWSTART_A>);
impl RWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWSTART_A {
        match self.bits {
            false => RWSTART_A::DISABLED,
            true => RWSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RWSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RWSTART_A::ENABLED
    }
}
impl core::ops::Deref for RWSTART_R {
    type Target = crate::FieldReader<bool, RWSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWSTART` writer - Read wait start"]
pub struct RWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Don't start read wait operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RWSTART_A::DISABLED)
    }
    #[doc = "Read wait operation starts"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RWSTART_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DBLOCKSIZE` reader - Data block size"]
pub struct DBLOCKSIZE_R(crate::FieldReader<u8, u8>);
impl DBLOCKSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBLOCKSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBLOCKSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBLOCKSIZE` writer - Data block size"]
pub struct DBLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLOCKSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "DMA enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Dma disabled"]
    DISABLED = 0,
    #[doc = "1: Dma enabled"]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Dma disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "Dma enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
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
#[doc = "Data transfer mode selection 1: Stream or SDIO multibyte data transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTMODE_A {
    #[doc = "0: Bloack data transfer"]
    BLOCKMODE = 0,
    #[doc = "1: Stream or SDIO multibyte data transfer"]
    STREAMMODE = 1,
}
impl From<DTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTMODE` reader - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
pub struct DTMODE_R(crate::FieldReader<bool, DTMODE_A>);
impl DTMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTMODE_A {
        match self.bits {
            false => DTMODE_A::BLOCKMODE,
            true => DTMODE_A::STREAMMODE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKMODE`"]
    #[inline(always)]
    pub fn is_block_mode(&self) -> bool {
        **self == DTMODE_A::BLOCKMODE
    }
    #[doc = "Checks if the value of the field is `STREAMMODE`"]
    #[inline(always)]
    pub fn is_stream_mode(&self) -> bool {
        **self == DTMODE_A::STREAMMODE
    }
}
impl core::ops::Deref for DTMODE_R {
    type Target = crate::FieldReader<bool, DTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTMODE` writer - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
pub struct DTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bloack data transfer"]
    #[inline(always)]
    pub fn block_mode(self) -> &'a mut W {
        self.variant(DTMODE_A::BLOCKMODE)
    }
    #[doc = "Stream or SDIO multibyte data transfer"]
    #[inline(always)]
    pub fn stream_mode(self) -> &'a mut W {
        self.variant(DTMODE_A::STREAMMODE)
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
#[doc = "Data transfer direction selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDIR_A {
    #[doc = "0: From controller to card"]
    CONTROLLERTOCARD = 0,
    #[doc = "1: From card to controller"]
    CARDTOCONTROLLER = 1,
}
impl From<DTDIR_A> for bool {
    #[inline(always)]
    fn from(variant: DTDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDIR` reader - Data transfer direction selection"]
pub struct DTDIR_R(crate::FieldReader<bool, DTDIR_A>);
impl DTDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDIR_A {
        match self.bits {
            false => DTDIR_A::CONTROLLERTOCARD,
            true => DTDIR_A::CARDTOCONTROLLER,
        }
    }
    #[doc = "Checks if the value of the field is `CONTROLLERTOCARD`"]
    #[inline(always)]
    pub fn is_controller_to_card(&self) -> bool {
        **self == DTDIR_A::CONTROLLERTOCARD
    }
    #[doc = "Checks if the value of the field is `CARDTOCONTROLLER`"]
    #[inline(always)]
    pub fn is_card_to_controller(&self) -> bool {
        **self == DTDIR_A::CARDTOCONTROLLER
    }
}
impl core::ops::Deref for DTDIR_R {
    type Target = crate::FieldReader<bool, DTDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTDIR` writer - Data transfer direction selection"]
pub struct DTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "From controller to card"]
    #[inline(always)]
    pub fn controller_to_card(self) -> &'a mut W {
        self.variant(DTDIR_A::CONTROLLERTOCARD)
    }
    #[doc = "From card to controller"]
    #[inline(always)]
    pub fn card_to_controller(self) -> &'a mut W {
        self.variant(DTDIR_A::CARDTOCONTROLLER)
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
#[doc = "DTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Start transfer"]
    ENABLED = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN` reader - DTEN"]
pub struct DTEN_R(crate::FieldReader<bool, DTEN_A>);
impl DTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::DISABLED,
            true => DTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DTEN_A::ENABLED
    }
}
impl core::ops::Deref for DTEN_R {
    type Target = crate::FieldReader<bool, DTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN` writer - DTEN"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN_A::DISABLED)
    }
    #[doc = "Start transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 11 - SD I/O enable functions"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read wait mode"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read wait start"]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SD I/O enable functions"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W { w: self }
    }
    #[doc = "Bit 10 - Read wait mode"]
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W {
        RWMOD_W { w: self }
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RWSTOP_W {
        RWSTOP_W { w: self }
    }
    #[doc = "Bit 8 - Read wait start"]
    #[inline(always)]
    pub fn rwstart(&mut self) -> RWSTART_W {
        RWSTART_W { w: self }
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W {
        DBLOCKSIZE_W { w: self }
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W {
        DTMODE_W { w: self }
    }
    #[doc = "Bit 1 - Data transfer direction selection"]
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W {
        DTDIR_W { w: self }
    }
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](index.html) module"]
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
