#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCRCFAILC_A {
    #[doc = "1: Clear flag"]
    CLEAR = 1,
}
impl From<CCRCFAILC_A> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCRCFAILC`"]
pub type CCRCFAILC_R = crate::R<bool, CCRCFAILC_A>;
impl CCRCFAILC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CCRCFAILC_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CCRCFAILC_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCRCFAILC_A::CLEAR
    }
}
#[doc = "Write proxy for field `CCRCFAILC`"]
pub struct CCRCFAILC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCFAILC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCRCFAILC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
pub type DCRCFAILC_A = CCRCFAILC_A;
#[doc = "Reader of field `DCRCFAILC`"]
pub type DCRCFAILC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `DCRCFAILC`"]
pub struct DCRCFAILC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCFAILC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCRCFAILC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
pub type CTIMEOUTC_A = CCRCFAILC_A;
#[doc = "Reader of field `CTIMEOUTC`"]
pub type CTIMEOUTC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `CTIMEOUTC`"]
pub struct CTIMEOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMEOUTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMEOUTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
pub type DTIMEOUTC_A = CCRCFAILC_A;
#[doc = "Reader of field `DTIMEOUTC`"]
pub type DTIMEOUTC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `DTIMEOUTC`"]
pub struct DTIMEOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIMEOUTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTIMEOUTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
pub type TXUNDERRC_A = CCRCFAILC_A;
#[doc = "Reader of field `TXUNDERRC`"]
pub type TXUNDERRC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `TXUNDERRC`"]
pub struct TXUNDERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXUNDERRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
#[doc = "RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
pub type RXOVERRC_A = CCRCFAILC_A;
#[doc = "Reader of field `RXOVERRC`"]
pub type RXOVERRC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `RXOVERRC`"]
pub struct RXOVERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXOVERRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "CMDREND flag clear bit Set by software to clear the CMDREND flag."]
pub type CMDRENDC_A = CCRCFAILC_A;
#[doc = "Reader of field `CMDRENDC`"]
pub type CMDRENDC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `CMDRENDC`"]
pub struct CMDRENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRENDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDRENDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
pub type CMDSENTC_A = CCRCFAILC_A;
#[doc = "Reader of field `CMDSENTC`"]
pub type CMDSENTC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `CMDSENTC`"]
pub struct CMDSENTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDSENTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "DATAEND flag clear bit Set by software to clear the DATAEND flag."]
pub type DATAENDC_A = CCRCFAILC_A;
#[doc = "Reader of field `DATAENDC`"]
pub type DATAENDC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `DATAENDC`"]
pub struct DATAENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAENDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
pub type DBCKENDC_A = CCRCFAILC_A;
#[doc = "Reader of field `DBCKENDC`"]
pub type DBCKENDC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `DBCKENDC`"]
pub struct DBCKENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCKENDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBCKENDC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
pub type SDIOITC_A = CCRCFAILC_A;
#[doc = "Reader of field `SDIOITC`"]
pub type SDIOITC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `SDIOITC`"]
pub struct SDIOITC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOITC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIOITC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
pub type IDMATEC_A = CCRCFAILC_A;
#[doc = "Reader of field `IDMATEC`"]
pub type IDMATEC_R = crate::R<bool, CCRCFAILC_A>;
#[doc = "Write proxy for field `IDMATEC`"]
pub struct IDMATEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMATEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDMATEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCRCFAILC_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag."]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag."]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W {
        CCRCFAILC_W { w: self }
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W {
        DCRCFAILC_W { w: self }
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W {
        CTIMEOUTC_W { w: self }
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W {
        DTIMEOUTC_W { w: self }
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W {
        TXUNDERRC_W { w: self }
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W {
        RXOVERRC_W { w: self }
    }
    #[doc = "Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag."]
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W {
        CMDRENDC_W { w: self }
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W {
        CMDSENTC_W { w: self }
    }
    #[doc = "Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag."]
    #[inline(always)]
    pub fn dataendc(&mut self) -> DATAENDC_W {
        DATAENDC_W { w: self }
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DBCKENDC_W {
        DBCKENDC_W { w: self }
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SDIOITC_W {
        SDIOITC_W { w: self }
    }
    #[doc = "Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
    #[inline(always)]
    pub fn idmatec(&mut self) -> IDMATEC_W {
        IDMATEC_W { w: self }
    }
}
