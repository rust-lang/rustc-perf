#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRCFAILIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<CCRCFAILIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub type CCRCFAILIE_R = crate::BitReader<CCRCFAILIE_A>;
impl CCRCFAILIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCRCFAILIE_A {
        match self.bits {
            false => CCRCFAILIE_A::Disabled,
            true => CCRCFAILIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCRCFAILIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCRCFAILIE_A::Enabled
    }
}
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub type CCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, CCRCFAILIE_A, O>;
impl<'a, const O: u8> CCRCFAILIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CCRCFAILIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CCRCFAILIE_A::Enabled)
    }
}
#[doc = "Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub use CCRCFAILIE_A as DCRCFAILIE_A;
#[doc = "Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub use CCRCFAILIE_A as CTIMEOUTIE_A;
#[doc = "Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub use CCRCFAILIE_A as DTIMEOUTIE_A;
#[doc = "Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub use CCRCFAILIE_A as TXUNDERRIE_A;
#[doc = "Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub use CCRCFAILIE_A as RXOVERRIE_A;
#[doc = "Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub use CCRCFAILIE_A as CMDRENDIE_A;
#[doc = "Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub use CCRCFAILIE_A as CMDSENTIE_A;
#[doc = "Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub use CCRCFAILIE_A as DATAENDIE_A;
#[doc = "Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub use CCRCFAILIE_A as DBCKENDIE_A;
#[doc = "Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub use CCRCFAILIE_A as CMDACTIE_A;
#[doc = "Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub use CCRCFAILIE_A as TXFIFOHEIE_A;
#[doc = "Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub use CCRCFAILIE_A as RXFIFOHFIE_A;
#[doc = "Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub use CCRCFAILIE_A as RXFIFOFIE_A;
#[doc = "Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub use CCRCFAILIE_A as TXFIFOEIE_A;
#[doc = "BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub use CCRCFAILIE_A as RXDAVLIE_A;
#[doc = "SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub use CCRCFAILIE_A as SDIOITIE_A;
#[doc = "Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
pub use CCRCFAILIE_A as TXDAVLIE_A;
#[doc = "Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
pub use CCRCFAILIE_A as RXFIFOEIE_A;
#[doc = "Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
pub use CCRCFAILIE_A as TXFIFOFIE_A;
#[doc = "Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
pub use CCRCFAILIE_A as RXACTIE_A;
#[doc = "Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
pub use CCRCFAILIE_A as TXACTIE_A;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub use CCRCFAILIE_R as DCRCFAILIE_R;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub use CCRCFAILIE_R as CTIMEOUTIE_R;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub use CCRCFAILIE_R as DTIMEOUTIE_R;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub use CCRCFAILIE_R as TXUNDERRIE_R;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub use CCRCFAILIE_R as RXOVERRIE_R;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub use CCRCFAILIE_R as CMDRENDIE_R;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub use CCRCFAILIE_R as CMDSENTIE_R;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub use CCRCFAILIE_R as DATAENDIE_R;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub use CCRCFAILIE_R as DBCKENDIE_R;
#[doc = "Field `CMDACTIE` reader - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub use CCRCFAILIE_R as CMDACTIE_R;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub use CCRCFAILIE_R as TXFIFOHEIE_R;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub use CCRCFAILIE_R as RXFIFOHFIE_R;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub use CCRCFAILIE_R as RXFIFOFIE_R;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub use CCRCFAILIE_R as TXFIFOEIE_R;
#[doc = "Field `RXDAVLIE` reader - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub use CCRCFAILIE_R as RXDAVLIE_R;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub use CCRCFAILIE_R as SDIOITIE_R;
#[doc = "Field `TXDAVLIE` reader - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
pub use CCRCFAILIE_R as TXDAVLIE_R;
#[doc = "Field `RXFIFOEIE` reader - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
pub use CCRCFAILIE_R as RXFIFOEIE_R;
#[doc = "Field `TXFIFOFIE` reader - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
pub use CCRCFAILIE_R as TXFIFOFIE_R;
#[doc = "Field `RXACTIE` reader - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
pub use CCRCFAILIE_R as RXACTIE_R;
#[doc = "Field `TXACTIE` reader - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
pub use CCRCFAILIE_R as TXACTIE_R;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub use CCRCFAILIE_W as DCRCFAILIE_W;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub use CCRCFAILIE_W as CTIMEOUTIE_W;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub use CCRCFAILIE_W as DTIMEOUTIE_W;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub use CCRCFAILIE_W as TXUNDERRIE_W;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub use CCRCFAILIE_W as RXOVERRIE_W;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub use CCRCFAILIE_W as CMDRENDIE_W;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub use CCRCFAILIE_W as CMDSENTIE_W;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub use CCRCFAILIE_W as DATAENDIE_W;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub use CCRCFAILIE_W as DBCKENDIE_W;
#[doc = "Field `CMDACTIE` writer - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub use CCRCFAILIE_W as CMDACTIE_W;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub use CCRCFAILIE_W as TXFIFOHEIE_W;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub use CCRCFAILIE_W as RXFIFOHFIE_W;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub use CCRCFAILIE_W as RXFIFOFIE_W;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub use CCRCFAILIE_W as TXFIFOEIE_W;
#[doc = "Field `RXDAVLIE` writer - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub use CCRCFAILIE_W as RXDAVLIE_W;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub use CCRCFAILIE_W as SDIOITIE_W;
#[doc = "Field `TXDAVLIE` writer - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
pub use CCRCFAILIE_W as TXDAVLIE_W;
#[doc = "Field `RXFIFOEIE` writer - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
pub use CCRCFAILIE_W as RXFIFOEIE_W;
#[doc = "Field `TXFIFOFIE` writer - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
pub use CCRCFAILIE_W as TXFIFOFIE_W;
#[doc = "Field `RXACTIE` writer - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
pub use CCRCFAILIE_W as RXACTIE_W;
#[doc = "Field `TXACTIE` writer - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
pub use CCRCFAILIE_W as TXACTIE_W;
impl R {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<0> {
        CCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<1> {
        DCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<2> {
        CTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<3> {
        DTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<4> {
        TXUNDERRIE_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<5> {
        RXOVERRIE_W::new(self)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<6> {
        CMDRENDIE_W::new(self)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<7> {
        CMDSENTIE_W::new(self)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W<8> {
        DATAENDIE_W::new(self)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<10> {
        DBCKENDIE_W::new(self)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<11> {
        CMDACTIE_W::new(self)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<14> {
        TXFIFOHEIE_W::new(self)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<15> {
        RXFIFOHFIE_W::new(self)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<17> {
        RXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<18> {
        TXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<21> {
        RXDAVLIE_W::new(self)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<22> {
        SDIOITIE_W::new(self)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable. Set and cleared by software to enable/disable the interrupt generated by the presence of data available in Tx FIFO"]
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<20> {
        TXDAVLIE_W::new(self)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable. Set and cleared by software to enable/disable interrupt caused by Rx FIFO empty"]
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<19> {
        RXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable. Set and cleared by software to enable/disable interrupt caused by Tx FIFO full"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<16> {
        TXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being received (data receive acting)"]
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W<13> {
        RXACTIE_W::new(self)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable. Set and cleared by software to enable/disable interrupt caused by data being transferred (data transmit acting)"]
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W<12> {
        TXACTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
