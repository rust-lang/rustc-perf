#[doc = "Register `CLKCR` reader"]
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCR` writer"]
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \\[CLKDIV + 2\\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \\[CLKDIV + 2\\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLKCR_SPEC, u8, u8, 8, O>;
#[doc = "Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSAV_A {
    #[doc = "0: SDIO_CK clock is always enabled"]
    Enabled = 0,
    #[doc = "1: SDIO_CK is only enabled when the bus is active"]
    Disabled = 1,
}
impl From<PWRSAV_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSAV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSAV` reader - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV"]
pub type PWRSAV_R = crate::BitReader<PWRSAV_A>;
impl PWRSAV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSAV_A {
        match self.bits {
            false => PWRSAV_A::Enabled,
            true => PWRSAV_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSAV_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSAV_A::Disabled
    }
}
#[doc = "Field `PWRSAV` writer - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV"]
pub type PWRSAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, PWRSAV_A, O>;
impl<'a, const O: u8> PWRSAV_W<'a, O> {
    #[doc = "SDIO_CK clock is always enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWRSAV_A::Enabled)
    }
    #[doc = "SDIO_CK is only enabled when the bus is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWRSAV_A::Disabled)
    }
}
#[doc = "Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDBUS_A {
    #[doc = "0: 1 lane wide bus"]
    BusWidth1 = 0,
    #[doc = "1: 4 lane wide bus"]
    BusWidth4 = 1,
    #[doc = "2: 8 lane wide bus"]
    BusWidth8 = 2,
}
impl From<WIDBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type WIDBUS_R = crate::FieldReader<u8, WIDBUS_A>;
impl WIDBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WIDBUS_A> {
        match self.bits {
            0 => Some(WIDBUS_A::BusWidth1),
            1 => Some(WIDBUS_A::BusWidth4),
            2 => Some(WIDBUS_A::BusWidth8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BusWidth1`"]
    #[inline(always)]
    pub fn is_bus_width1(&self) -> bool {
        *self == WIDBUS_A::BusWidth1
    }
    #[doc = "Checks if the value of the field is `BusWidth4`"]
    #[inline(always)]
    pub fn is_bus_width4(&self) -> bool {
        *self == WIDBUS_A::BusWidth4
    }
    #[doc = "Checks if the value of the field is `BusWidth8`"]
    #[inline(always)]
    pub fn is_bus_width8(&self) -> bool {
        *self == WIDBUS_A::BusWidth8
    }
}
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
pub type WIDBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, WIDBUS_A, 2, O>;
impl<'a, const O: u8> WIDBUS_W<'a, O> {
    #[doc = "1 lane wide bus"]
    #[inline(always)]
    pub fn bus_width1(self) -> &'a mut W {
        self.variant(WIDBUS_A::BusWidth1)
    }
    #[doc = "4 lane wide bus"]
    #[inline(always)]
    pub fn bus_width4(self) -> &'a mut W {
        self.variant(WIDBUS_A::BusWidth4)
    }
    #[doc = "8 lane wide bus"]
    #[inline(always)]
    pub fn bus_width8(self) -> &'a mut W {
        self.variant(WIDBUS_A::BusWidth8)
    }
}
#[doc = "SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGEDGE_A {
    #[doc = "0: SDIO_CK generated on the rising edge"]
    Rising = 0,
    #[doc = "1: SDIO_CK generated on the falling edge"]
    Falling = 1,
}
impl From<NEGEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: NEGEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEGEDGE` reader - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value"]
pub type NEGEDGE_R = crate::BitReader<NEGEDGE_A>;
impl NEGEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEGEDGE_A {
        match self.bits {
            false => NEGEDGE_A::Rising,
            true => NEGEDGE_A::Falling,
        }
    }
    #[doc = "Checks if the value of the field is `Rising`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == NEGEDGE_A::Rising
    }
    #[doc = "Checks if the value of the field is `Falling`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == NEGEDGE_A::Falling
    }
}
#[doc = "Field `NEGEDGE` writer - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value"]
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, NEGEDGE_A, O>;
impl<'a, const O: u8> NEGEDGE_W<'a, O> {
    #[doc = "SDIO_CK generated on the rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(NEGEDGE_A::Rising)
    }
    #[doc = "SDIO_CK generated on the falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(NEGEDGE_A::Falling)
    }
}
#[doc = "HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWFC_EN_A {
    #[doc = "0: HW Flow Control is disabled"]
    Disabled = 0,
    #[doc = "1: HW Flow Control is enabled"]
    Enabled = 1,
}
impl From<HWFC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HWFC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWFC_EN` reader - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11"]
pub type HWFC_EN_R = crate::BitReader<HWFC_EN_A>;
impl HWFC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWFC_EN_A {
        match self.bits {
            false => HWFC_EN_A::Disabled,
            true => HWFC_EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_EN_A::Enabled
    }
}
#[doc = "Field `HWFC_EN` writer - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11"]
pub type HWFC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, HWFC_EN_A, O>;
impl<'a, const O: u8> HWFC_EN_W<'a, O> {
    #[doc = "HW Flow Control is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFC_EN_A::Disabled)
    }
    #[doc = "HW Flow Control is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFC_EN_A::Enabled)
    }
}
#[doc = "Clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "0: Disable clock"]
    Disabled = 0,
    #[doc = "1: Enable clock"]
    Enabled = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable bit"]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::Disabled,
            true => CLKEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::Enabled
    }
}
#[doc = "Field `CLKEN` writer - Clock enable bit"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Disabled)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Enabled)
    }
}
#[doc = "Clock divider bypass enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    Disabled = 0,
    #[doc = "1: SDIOCLK directly drives the SDIO_CK output signal"]
    Enabled = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Clock divider bypass enable bit"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::Disabled,
            true => BYPASS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::Enabled
    }
}
#[doc = "Field `BYPASS` writer - Clock divider bypass enable bit"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::Disabled)
    }
    #[doc = "SDIOCLK directly drives the SDIO_CK output signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \\[CLKDIV + 2\\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \\[CLKDIV + 2\\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 9 - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W<9> {
        PWRSAV_W::new(self)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)"]
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W<11> {
        WIDBUS_W::new(self)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value"]
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W<13> {
        NEGEDGE_W::new(self)
    }
    #[doc = "Bit 14 - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11"]
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<14> {
        HWFC_EN_W::new(self)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<8> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<10> {
        BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKCR register controls the SDIO_CK output clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](index.html) module"]
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcr::R](R) reader structure"]
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcr::W](W) writer structure"]
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
