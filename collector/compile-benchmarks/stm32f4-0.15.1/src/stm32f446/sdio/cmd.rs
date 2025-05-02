#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDINDEX` reader - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMD_SPEC, u8, u8, 6, O>;
#[doc = "Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAITRESP_A {
    #[doc = "0: No response"]
    NoResponse = 0,
    #[doc = "1: Short response"]
    ShortResponse = 1,
    #[doc = "2: No reponse"]
    NoResponse2 = 2,
    #[doc = "3: Long reponse"]
    LongResponse = 3,
}
impl From<WAITRESP_A> for u8 {
    #[inline(always)]
    fn from(variant: WAITRESP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAITRESP` reader - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
pub type WAITRESP_R = crate::FieldReader<u8, WAITRESP_A>;
impl WAITRESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITRESP_A {
        match self.bits {
            0 => WAITRESP_A::NoResponse,
            1 => WAITRESP_A::ShortResponse,
            2 => WAITRESP_A::NoResponse2,
            3 => WAITRESP_A::LongResponse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoResponse`"]
    #[inline(always)]
    pub fn is_no_response(&self) -> bool {
        *self == WAITRESP_A::NoResponse
    }
    #[doc = "Checks if the value of the field is `ShortResponse`"]
    #[inline(always)]
    pub fn is_short_response(&self) -> bool {
        *self == WAITRESP_A::ShortResponse
    }
    #[doc = "Checks if the value of the field is `NoResponse2`"]
    #[inline(always)]
    pub fn is_no_response2(&self) -> bool {
        *self == WAITRESP_A::NoResponse2
    }
    #[doc = "Checks if the value of the field is `LongResponse`"]
    #[inline(always)]
    pub fn is_long_response(&self) -> bool {
        *self == WAITRESP_A::LongResponse
    }
}
#[doc = "Field `WAITRESP` writer - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
pub type WAITRESP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CMD_SPEC, u8, WAITRESP_A, 2, O>;
impl<'a, const O: u8> WAITRESP_W<'a, O> {
    #[doc = "No response"]
    #[inline(always)]
    pub fn no_response(self) -> &'a mut W {
        self.variant(WAITRESP_A::NoResponse)
    }
    #[doc = "Short response"]
    #[inline(always)]
    pub fn short_response(self) -> &'a mut W {
        self.variant(WAITRESP_A::ShortResponse)
    }
    #[doc = "No reponse"]
    #[inline(always)]
    pub fn no_response2(self) -> &'a mut W {
        self.variant(WAITRESP_A::NoResponse2)
    }
    #[doc = "Long reponse"]
    #[inline(always)]
    pub fn long_response(self) -> &'a mut W {
        self.variant(WAITRESP_A::LongResponse)
    }
}
#[doc = "CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITINT_A {
    #[doc = "0: Don't wait for interrupt request"]
    Disabled = 0,
    #[doc = "1: Wait for interrupt request"]
    Enabled = 1,
}
impl From<WAITINT_A> for bool {
    #[inline(always)]
    fn from(variant: WAITINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
pub type WAITINT_R = crate::BitReader<WAITINT_A>;
impl WAITINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITINT_A {
        match self.bits {
            false => WAITINT_A::Disabled,
            true => WAITINT_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITINT_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITINT_A::Enabled
    }
}
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
pub type WAITINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, WAITINT_A, O>;
impl<'a, const O: u8> WAITINT_W<'a, O> {
    #[doc = "Don't wait for interrupt request"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITINT_A::Disabled)
    }
    #[doc = "Wait for interrupt request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITINT_A::Enabled)
    }
}
#[doc = "PSM Waits for ends of data transfer (CmdPend internal signal). If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command. This feature is available only with Stream data transfer mode SDIO_DCTRL\\[2\\]
= 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITPEND_A {
    #[doc = "0: Don't wait for data end"]
    Disabled = 0,
    #[doc = "1: Wait for end of data transfer signal before sending command"]
    Enabled = 1,
}
impl From<WAITPEND_A> for bool {
    #[inline(always)]
    fn from(variant: WAITPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITPEND` reader - PSM Waits for ends of data transfer (CmdPend internal signal). If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command. This feature is available only with Stream data transfer mode SDIO_DCTRL\\[2\\]
= 1."]
pub type WAITPEND_R = crate::BitReader<WAITPEND_A>;
impl WAITPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITPEND_A {
        match self.bits {
            false => WAITPEND_A::Disabled,
            true => WAITPEND_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITPEND_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITPEND_A::Enabled
    }
}
#[doc = "Field `WAITPEND` writer - PSM Waits for ends of data transfer (CmdPend internal signal). If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command. This feature is available only with Stream data transfer mode SDIO_DCTRL\\[2\\]
= 1."]
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, WAITPEND_A, O>;
impl<'a, const O: u8> WAITPEND_W<'a, O> {
    #[doc = "Don't wait for data end"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITPEND_A::Disabled)
    }
    #[doc = "Wait for end of data transfer signal before sending command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITPEND_A::Enabled)
    }
}
#[doc = "Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPSMEN_A {
    #[doc = "0: Command path state machine disabled"]
    Disabled = 0,
    #[doc = "1: Command path state machine enabled"]
    Enabled = 1,
}
impl From<CPSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPSMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
pub type CPSMEN_R = crate::BitReader<CPSMEN_A>;
impl CPSMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPSMEN_A {
        match self.bits {
            false => CPSMEN_A::Disabled,
            true => CPSMEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPSMEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPSMEN_A::Enabled
    }
}
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
pub type CPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, CPSMEN_A, O>;
impl<'a, const O: u8> CPSMEN_W<'a, O> {
    #[doc = "Command path state machine disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPSMEN_A::Disabled)
    }
    #[doc = "Command path state machine enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPSMEN_A::Enabled)
    }
}
#[doc = "SD I/O suspend command. If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIOSUSPEND_A {
    #[doc = "0: Next command is not a SDIO suspend command"]
    Disabled = 0,
    #[doc = "1: Next command send is a SDIO suspend command"]
    Enabled = 1,
}
impl From<SDIOSUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SDIOSUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOSuspend` reader - SD I/O suspend command. If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card)"]
pub type SDIOSUSPEND_R = crate::BitReader<SDIOSUSPEND_A>;
impl SDIOSUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIOSUSPEND_A {
        match self.bits {
            false => SDIOSUSPEND_A::Disabled,
            true => SDIOSUSPEND_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIOSUSPEND_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIOSUSPEND_A::Enabled
    }
}
#[doc = "Field `SDIOSuspend` writer - SD I/O suspend command. If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card)"]
pub type SDIOSUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, SDIOSUSPEND_A, O>;
impl<'a, const O: u8> SDIOSUSPEND_W<'a, O> {
    #[doc = "Next command is not a SDIO suspend command"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIOSUSPEND_A::Disabled)
    }
    #[doc = "Next command send is a SDIO suspend command"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIOSUSPEND_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:5 - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PSM Waits for ends of data transfer (CmdPend internal signal). If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command. This feature is available only with Stream data transfer mode SDIO_DCTRL\\[2\\]
= 1."]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command. If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card)"]
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). The command index is sent to the card as part of a command message."]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Bits 6:7 - Wait for response bits. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). They are used to configure whether the CPSM is to wait for a response, and if yes, which kind of response."]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W<6> {
        WAITRESP_W::new(self)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request. If this bit is set, the CPSM disables command timeout and waits for an card interrupt request (Response). If this bit is cleared in the CPSM Wait state, will cause the abort of the interrupt mode."]
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W<8> {
        WAITINT_W::new(self)
    }
    #[doc = "Bit 9 - PSM Waits for ends of data transfer (CmdPend internal signal). If this bit is set, the CPSM waits for the end of data transfer before it starts sending a command. This feature is available only with Stream data transfer mode SDIO_DCTRL\\[2\\]
= 1."]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W<9> {
        WAITPEND_W::new(self)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle state. If this bit is set, the CPSM is enabled. When DTEN = 1, no command will be transfered nor boot procedure will be started. CPSMEN is cleared to 0."]
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W<10> {
        CPSMEN_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O suspend command. If this bit is set, the command to be sent is a suspend command (to be used only with SDIO card)"]
    #[inline(always)]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<11> {
        SDIOSUSPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
