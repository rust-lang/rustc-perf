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
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    #[doc = "0: TXE interrupt masked"]
    Masked = 0,
    #[doc = "1: TXE interrupt not masked"]
    NotMasked = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::Masked,
            true => TXEIE_A::NotMasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE_A::Masked
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE_A::NotMasked
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TXEIE_A, O>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::Masked)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NotMasked)
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    #[doc = "0: RXE interrupt masked"]
    Masked = 0,
    #[doc = "1: RXE interrupt not masked"]
    NotMasked = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::Masked,
            true => RXNEIE_A::NotMasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE_A::Masked
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE_A::NotMasked
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RXNEIE_A, O>;
impl<'a, const O: u8> RXNEIE_W<'a, O> {
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::Masked)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NotMasked)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt masked"]
    Masked = 0,
    #[doc = "1: Error interrupt not masked"]
    NotMasked = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Masked,
            true => ERRIE_A::NotMasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE_A::Masked
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE_A::NotMasked
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::Masked)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NotMasked)
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: SPI Motorola mode"]
    Motorola = 0,
    #[doc = "1: SPI TI mode"]
    Ti = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::BitReader<FRF_A>;
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::Motorola,
            true => FRF_A::Ti,
        }
    }
    #[doc = "Checks if the value of the field is `Motorola`"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF_A::Motorola
    }
    #[doc = "Checks if the value of the field is `Ti`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::Ti
    }
}
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, FRF_A, O>;
impl<'a, const O: u8> FRF_W<'a, O> {
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRF_A::Motorola)
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::Ti)
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode"]
    Disabled = 0,
    #[doc = "1: SS output is enabled in master mode"]
    Enabled = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SSOE_R = crate::BitReader<SSOE_A>;
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::Disabled,
            true => SSOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::Enabled
    }
}
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SSOE_A, O>;
impl<'a, const O: u8> SSOE_W<'a, O> {
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::Disabled)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::Enabled)
    }
}
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    Enabled = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::Disabled,
            true => TXDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TXDMAEN_A, O>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Disabled)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Enabled)
    }
}
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    Enabled = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::Disabled,
            true => RXDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RXDMAEN_A, O>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Disabled)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<6> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<2> {
        SSOE_W::new(self)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<1> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<0> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
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
