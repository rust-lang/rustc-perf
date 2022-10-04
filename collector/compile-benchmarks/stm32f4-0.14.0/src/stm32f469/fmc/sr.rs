#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FEMPT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEMPT_A {
    #[doc = "0: FIFO not empty"]
    NOTEMPTY = 0,
    #[doc = "1: FIFO empty"]
    EMPTY = 1,
}
impl From<FEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: FEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEMPT` reader - FEMPT"]
pub struct FEMPT_R(crate::FieldReader<bool, FEMPT_A>);
impl FEMPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEMPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEMPT_A {
        match self.bits {
            false => FEMPT_A::NOTEMPTY,
            true => FEMPT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == FEMPT_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == FEMPT_A::EMPTY
    }
}
impl core::ops::Deref for FEMPT_R {
    type Target = crate::FieldReader<bool, FEMPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "IFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFEN_A {
    #[doc = "0: Interrupt falling edge detection request disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt falling edge detection request enabled"]
    ENABLED = 1,
}
impl From<IFEN_A> for bool {
    #[inline(always)]
    fn from(variant: IFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFEN` reader - IFEN"]
pub struct IFEN_R(crate::FieldReader<bool, IFEN_A>);
impl IFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFEN_A {
        match self.bits {
            false => IFEN_A::DISABLED,
            true => IFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IFEN_A::ENABLED
    }
}
impl core::ops::Deref for IFEN_R {
    type Target = crate::FieldReader<bool, IFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFEN` writer - IFEN"]
pub struct IFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt falling edge detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IFEN_A::DISABLED)
    }
    #[doc = "Interrupt falling edge detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IFEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "ILEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILEN_A {
    #[doc = "0: Interrupt high-level detection request disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt high-level detection request enabled"]
    ENABLED = 1,
}
impl From<ILEN_A> for bool {
    #[inline(always)]
    fn from(variant: ILEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILEN` reader - ILEN"]
pub struct ILEN_R(crate::FieldReader<bool, ILEN_A>);
impl ILEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILEN_A {
        match self.bits {
            false => ILEN_A::DISABLED,
            true => ILEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ILEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ILEN_A::ENABLED
    }
}
impl core::ops::Deref for ILEN_R {
    type Target = crate::FieldReader<bool, ILEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILEN` writer - ILEN"]
pub struct ILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ILEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt high-level detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ILEN_A::DISABLED)
    }
    #[doc = "Interrupt high-level detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ILEN_A::ENABLED)
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
#[doc = "IREN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: Interrupt rising edge detection request disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt rising edge detection request enabled"]
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IREN"]
pub struct IREN_R(crate::FieldReader<bool, IREN_A>);
impl IREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IREN_A::ENABLED
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, IREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREN` writer - IREN"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt rising edge detection request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    #[doc = "Interrupt rising edge detection request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
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
#[doc = "IFS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFS_A {
    #[doc = "0: Interrupt falling edge did not occur"]
    DIDNOTOCCUR = 0,
    #[doc = "1: Interrupt falling edge occurred"]
    OCCURRED = 1,
}
impl From<IFS_A> for bool {
    #[inline(always)]
    fn from(variant: IFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFS` reader - IFS"]
pub struct IFS_R(crate::FieldReader<bool, IFS_A>);
impl IFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFS_A {
        match self.bits {
            false => IFS_A::DIDNOTOCCUR,
            true => IFS_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `DIDNOTOCCUR`"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        **self == IFS_A::DIDNOTOCCUR
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == IFS_A::OCCURRED
    }
}
impl core::ops::Deref for IFS_R {
    type Target = crate::FieldReader<bool, IFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFS` writer - IFS"]
pub struct IFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt falling edge did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut W {
        self.variant(IFS_A::DIDNOTOCCUR)
    }
    #[doc = "Interrupt falling edge occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(IFS_A::OCCURRED)
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
#[doc = "ILS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILS_A {
    #[doc = "0: Interrupt high-level did not occur"]
    DIDNOTOCCUR = 0,
    #[doc = "1: Interrupt high-level occurred"]
    OCCURRED = 1,
}
impl From<ILS_A> for bool {
    #[inline(always)]
    fn from(variant: ILS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILS` reader - ILS"]
pub struct ILS_R(crate::FieldReader<bool, ILS_A>);
impl ILS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILS_A {
        match self.bits {
            false => ILS_A::DIDNOTOCCUR,
            true => ILS_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `DIDNOTOCCUR`"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        **self == ILS_A::DIDNOTOCCUR
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == ILS_A::OCCURRED
    }
}
impl core::ops::Deref for ILS_R {
    type Target = crate::FieldReader<bool, ILS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILS` writer - ILS"]
pub struct ILS_W<'a> {
    w: &'a mut W,
}
impl<'a> ILS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt high-level did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut W {
        self.variant(ILS_A::DIDNOTOCCUR)
    }
    #[doc = "Interrupt high-level occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(ILS_A::OCCURRED)
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
#[doc = "IRS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRS_A {
    #[doc = "0: Interrupt rising edge did not occur"]
    DIDNOTOCCUR = 0,
    #[doc = "1: Interrupt rising edge occurred"]
    OCCURRED = 1,
}
impl From<IRS_A> for bool {
    #[inline(always)]
    fn from(variant: IRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRS` reader - IRS"]
pub struct IRS_R(crate::FieldReader<bool, IRS_A>);
impl IRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRS_A {
        match self.bits {
            false => IRS_A::DIDNOTOCCUR,
            true => IRS_A::OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `DIDNOTOCCUR`"]
    #[inline(always)]
    pub fn is_did_not_occur(&self) -> bool {
        **self == IRS_A::DIDNOTOCCUR
    }
    #[doc = "Checks if the value of the field is `OCCURRED`"]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        **self == IRS_A::OCCURRED
    }
}
impl core::ops::Deref for IRS_R {
    type Target = crate::FieldReader<bool, IRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRS` writer - IRS"]
pub struct IRS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt rising edge did not occur"]
    #[inline(always)]
    pub fn did_not_occur(self) -> &'a mut W {
        self.variant(IRS_A::DIDNOTOCCUR)
    }
    #[doc = "Interrupt rising edge occurred"]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut W {
        self.variant(IRS_A::OCCURRED)
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
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W {
        IFEN_W { w: self }
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W {
        ILEN_W { w: self }
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W {
        IFS_W { w: self }
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W {
        ILS_W { w: self }
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W {
        IRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO status and interrupt register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
