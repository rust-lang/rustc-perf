#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Packet error checking byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECBYTE_A {
    #[doc = "0: No PEC transfer"]
    NOPEC = 0,
    #[doc = "1: PEC transmission/reception is requested"]
    PEC = 1,
}
impl From<PECBYTE_A> for bool {
    #[inline(always)]
    fn from(variant: PECBYTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECBYTE` reader - Packet error checking byte"]
pub struct PECBYTE_R(crate::FieldReader<bool, PECBYTE_A>);
impl PECBYTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECBYTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECBYTE_A {
        match self.bits {
            false => PECBYTE_A::NOPEC,
            true => PECBYTE_A::PEC,
        }
    }
    #[doc = "Checks if the value of the field is `NOPEC`"]
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        **self == PECBYTE_A::NOPEC
    }
    #[doc = "Checks if the value of the field is `PEC`"]
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        **self == PECBYTE_A::PEC
    }
}
impl core::ops::Deref for PECBYTE_R {
    type Target = crate::FieldReader<bool, PECBYTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECBYTE` writer - Packet error checking byte"]
pub struct PECBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PECBYTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PECBYTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn no_pec(self) -> &'a mut W {
        self.variant(PECBYTE_A::NOPEC)
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn pec(self) -> &'a mut W {
        self.variant(PECBYTE_A::PEC)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Automatic end mode (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOEND_A {
    #[doc = "0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    SOFTWARE = 0,
    #[doc = "1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    AUTOMATIC = 1,
}
impl From<AUTOEND_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode)"]
pub struct AUTOEND_R(crate::FieldReader<bool, AUTOEND_A>);
impl AUTOEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOEND_A {
        match self.bits {
            false => AUTOEND_A::SOFTWARE,
            true => AUTOEND_A::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == AUTOEND_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        **self == AUTOEND_A::AUTOMATIC
    }
}
impl core::ops::Deref for AUTOEND_R {
    type Target = crate::FieldReader<bool, AUTOEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode)"]
pub struct AUTOEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(AUTOEND_A::SOFTWARE)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTOEND_A::AUTOMATIC)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "NBYTES reload mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOAD_A {
    #[doc = "0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    COMPLETED = 0,
    #[doc = "1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NOTCOMPLETED = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - NBYTES reload mode"]
pub struct RELOAD_R(crate::FieldReader<bool, RELOAD_A>);
impl RELOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RELOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::COMPLETED,
            true => RELOAD_A::NOTCOMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        **self == RELOAD_A::COMPLETED
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        **self == RELOAD_A::NOTCOMPLETED
    }
}
impl core::ops::Deref for RELOAD_R {
    type Target = crate::FieldReader<bool, RELOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD` writer - NBYTES reload mode"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(RELOAD_A::COMPLETED)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(RELOAD_A::NOTCOMPLETED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub struct NBYTES_R(crate::FieldReader<u8, u8>);
impl NBYTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBYTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "NACK generation (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: an ACK is sent after current received byte"]
    ACK = 0,
    #[doc = "1: a NACK is sent after current received byte"]
    NACK = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - NACK generation (slave mode)"]
pub struct NACK_R(crate::FieldReader<bool, NACK_A>);
impl NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::ACK,
            true => NACK_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        **self == NACK_A::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        **self == NACK_A::NACK
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, NACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK` writer - NACK generation (slave mode)"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "an ACK is sent after current received byte"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(NACK_A::ACK)
    }
    #[doc = "a NACK is sent after current received byte"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(NACK_A::NACK)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Stop generation (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: No Stop generation"]
    NOSTOP = 0,
    #[doc = "1: Stop generation after current byte transfer"]
    STOP = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop generation (master mode)"]
pub struct STOP_R(crate::FieldReader<bool, STOP_A>);
impl STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NOSTOP,
            true => STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        **self == STOP_A::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == STOP_A::STOP
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Stop generation (master mode)"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOP_A::NOSTOP)
    }
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_A::STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: No Start generation"]
    NOSTART = 0,
    #[doc = "1: Restart/Start generation"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start generation"]
pub struct START_R(crate::FieldReader<bool, START_A>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NOSTART,
            true => START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        **self == START_A::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == START_A::START
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start generation"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NOSTART)
    }
    #[doc = "Restart/Start generation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "10-bit address header only read direction (master receiver mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEAD10R_A {
    #[doc = "0: The master sends the complete 10 bit slave address read sequence"]
    COMPLETE = 0,
    #[doc = "1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    PARTIAL = 1,
}
impl From<HEAD10R_A> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)"]
pub struct HEAD10R_R(crate::FieldReader<bool, HEAD10R_A>);
impl HEAD10R_R {
    pub(crate) fn new(bits: bool) -> Self {
        HEAD10R_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEAD10R_A {
        match self.bits {
            false => HEAD10R_A::COMPLETE,
            true => HEAD10R_A::PARTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == HEAD10R_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        **self == HEAD10R_A::PARTIAL
    }
}
impl core::ops::Deref for HEAD10R_R {
    type Target = crate::FieldReader<bool, HEAD10R_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)"]
pub struct HEAD10R_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD10R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HEAD10R_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(HEAD10R_A::COMPLETE)
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(HEAD10R_A::PARTIAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "10-bit addressing mode (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD10_A {
    #[doc = "0: The master operates in 7-bit addressing mode"]
    BIT7 = 0,
    #[doc = "1: The master operates in 10-bit addressing mode"]
    BIT10 = 1,
}
impl From<ADD10_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode)"]
pub struct ADD10_R(crate::FieldReader<bool, ADD10_A>);
impl ADD10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD10_A {
        match self.bits {
            false => ADD10_A::BIT7,
            true => ADD10_A::BIT10,
        }
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        **self == ADD10_A::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        **self == ADD10_A::BIT10
    }
}
impl core::ops::Deref for ADD10_R {
    type Target = crate::FieldReader<bool, ADD10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode)"]
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADD10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADD10_A::BIT7)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(ADD10_A::BIT10)
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
#[doc = "Transfer direction (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_WRN_A {
    #[doc = "0: Master requests a write transfer"]
    WRITE = 0,
    #[doc = "1: Master requests a read transfer"]
    READ = 1,
}
impl From<RD_WRN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode)"]
pub struct RD_WRN_R(crate::FieldReader<bool, RD_WRN_A>);
impl RD_WRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_WRN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_WRN_A {
        match self.bits {
            false => RD_WRN_A::WRITE,
            true => RD_WRN_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == RD_WRN_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == RD_WRN_A::READ
    }
}
impl core::ops::Deref for RD_WRN_R {
    type Target = crate::FieldReader<bool, RD_WRN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode)"]
pub struct RD_WRN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WRN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RD_WRN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(RD_WRN_A::WRITE)
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RD_WRN_A::READ)
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
#[doc = "Field `SADD` reader - Slave address bit (master mode)"]
pub struct SADD_R(crate::FieldReader<u16, u16>);
impl SADD_R {
    pub(crate) fn new(bits: u16) -> Self {
        SADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADD` writer - Slave address bit (master mode)"]
pub struct SADD_W<'a> {
    w: &'a mut W,
}
impl<'a> SADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - Slave address bit (master mode)"]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W {
        PECBYTE_W { w: self }
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W {
        AUTOEND_W { w: self }
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W {
        HEAD10R_W { w: self }
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W {
        RD_WRN_W { w: self }
    }
    #[doc = "Bits 0:9 - Slave address bit (master mode)"]
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W {
        SADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
