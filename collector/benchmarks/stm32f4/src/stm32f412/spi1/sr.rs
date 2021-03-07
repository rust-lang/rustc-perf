#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "TI frame format error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRE_A {
    #[doc = "0: No frame format error"]
    NOERROR = 0,
    #[doc = "1: A frame format error occurred"]
    ERROR = 1,
}
impl From<FRE_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRE`"]
pub type FRE_R = crate::R<bool, FRE_A>;
impl FRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRE_A {
        match self.bits {
            false => FRE_A::NOERROR,
            true => FRE_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FRE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FRE_A::ERROR
    }
}
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    #[doc = "0: SPI not busy"]
    NOTBUSY = 0,
    #[doc = "1: SPI busy"]
    BUSY = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, BSY_A>;
impl BSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::NOTBUSY,
            true => BSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSY_A::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSY_A::BUSY
    }
}
#[doc = "Overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "0: No overrun occurred"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
#[doc = "Mode fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No mode fault occurred"]
    NOFAULT = 0,
    #[doc = "1: Mode fault occurred"]
    FAULT = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODF`"]
pub type MODF_R = crate::R<bool, MODF_A>;
impl MODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::NOFAULT,
            true => MODF_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODF_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODF_A::FAULT
    }
}
#[doc = "CRC error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    MATCH = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    NOMATCH = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, CRCERR_A>;
impl CRCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::MATCH,
            true => CRCERR_A::NOMATCH,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == CRCERR_A::MATCH
    }
    #[doc = "Checks if the value of the field is `NOMATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERR_A::NOMATCH
    }
}
#[doc = "Write proxy for field `CRCERR`"]
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(CRCERR_A::MATCH)
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(CRCERR_A::NOMATCH)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDR_A {
    #[doc = "0: No underrun occurred"]
    NOUNDERRUN = 0,
    #[doc = "1: Underrun occurred"]
    UNDERRUN = 1,
}
impl From<UDR_A> for bool {
    #[inline(always)]
    fn from(variant: UDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UDR`"]
pub type UDR_R = crate::R<bool, UDR_A>;
impl UDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDR_A {
        match self.bits {
            false => UDR_A::NOUNDERRUN,
            true => UDR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOUNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR_A::NOUNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR_A::UNDERRUN
    }
}
#[doc = "Channel side\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSIDE_A {
    #[doc = "0: Channel left has to be transmitted or has been received"]
    LEFT = 0,
    #[doc = "1: Channel right has to be transmitted or has been received"]
    RIGHT = 1,
}
impl From<CHSIDE_A> for bool {
    #[inline(always)]
    fn from(variant: CHSIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHSIDE`"]
pub type CHSIDE_R = crate::R<bool, CHSIDE_A>;
impl CHSIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSIDE_A {
        match self.bits {
            false => CHSIDE_A::LEFT,
            true => CHSIDE_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == CHSIDE_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == CHSIDE_A::RIGHT
    }
}
#[doc = "Transmit buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: Tx buffer not empty"]
    NOTEMPTY = 0,
    #[doc = "1: Tx buffer empty"]
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, TXE_A>;
impl TXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOTEMPTY,
            true => TXE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::EMPTY
    }
}
#[doc = "Receive buffer not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    #[doc = "0: Rx buffer empty"]
    EMPTY = 0,
    #[doc = "1: Rx buffer not empty"]
    NOTEMPTY = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXNE`"]
pub type RXNE_R = crate::R<bool, RXNE_A>;
impl RXNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::EMPTY,
            true => RXNE_A::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NOTEMPTY
    }
}
impl R {
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Underrun flag"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel side"]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
}
