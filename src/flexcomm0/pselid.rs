#[doc = "Register `PSELID` reader"]
pub type R = crate::R<PselidSpec>;
#[doc = "Register `PSELID` writer"]
pub type W = crate::W<PselidSpec>;
#[doc = "Peripheral Select. This field is writable by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Persel {
    #[doc = "0: No peripheral selected."]
    NoPeriphSelected = 0,
    #[doc = "1: USART function selected."]
    Usart = 1,
    #[doc = "2: SPI function selected."]
    Spi = 2,
    #[doc = "3: I2C function selected."]
    I2c = 3,
    #[doc = "4: I2S transmit function selected."]
    I2sTransmit = 4,
    #[doc = "5: I2S receive function selected."]
    I2sReceive = 5,
}
impl From<Persel> for u8 {
    #[inline(always)]
    fn from(variant: Persel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Persel {
    type Ux = u8;
}
impl crate::IsEnum for Persel {}
#[doc = "Field `PERSEL` reader - Peripheral Select. This field is writable by software."]
pub type PerselR = crate::FieldReader<Persel>;
impl PerselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Persel> {
        match self.bits {
            0 => Some(Persel::NoPeriphSelected),
            1 => Some(Persel::Usart),
            2 => Some(Persel::Spi),
            3 => Some(Persel::I2c),
            4 => Some(Persel::I2sTransmit),
            5 => Some(Persel::I2sReceive),
            _ => None,
        }
    }
    #[doc = "No peripheral selected."]
    #[inline(always)]
    pub fn is_no_periph_selected(&self) -> bool {
        *self == Persel::NoPeriphSelected
    }
    #[doc = "USART function selected."]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == Persel::Usart
    }
    #[doc = "SPI function selected."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Persel::Spi
    }
    #[doc = "I2C function selected."]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == Persel::I2c
    }
    #[doc = "I2S transmit function selected."]
    #[inline(always)]
    pub fn is_i2s_transmit(&self) -> bool {
        *self == Persel::I2sTransmit
    }
    #[doc = "I2S receive function selected."]
    #[inline(always)]
    pub fn is_i2s_receive(&self) -> bool {
        *self == Persel::I2sReceive
    }
}
#[doc = "Field `PERSEL` writer - Peripheral Select. This field is writable by software."]
pub type PerselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Persel>;
impl<'a, REG> PerselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No peripheral selected."]
    #[inline(always)]
    pub fn no_periph_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::NoPeriphSelected)
    }
    #[doc = "USART function selected."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Usart)
    }
    #[doc = "SPI function selected."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::Spi)
    }
    #[doc = "I2C function selected."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::I2c)
    }
    #[doc = "I2S transmit function selected."]
    #[inline(always)]
    pub fn i2s_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::I2sTransmit)
    }
    #[doc = "I2S receive function selected."]
    #[inline(always)]
    pub fn i2s_receive(self) -> &'a mut crate::W<REG> {
        self.variant(Persel::I2sReceive)
    }
}
#[doc = "Lock the peripheral select. This field is writable by software.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Peripheral select can be changed by software."]
    Unlocked = 0,
    #[doc = "1: Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock the peripheral select. This field is writable by software."]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Peripheral select can be changed by software."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - Lock the peripheral select. This field is writable by software."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral select can be changed by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
#[doc = "USART present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usartpresent {
    #[doc = "0: This Flexcomm does not include the USART function."]
    NotPresent = 0,
    #[doc = "1: This Flexcomm includes the USART function."]
    Present = 1,
}
impl From<Usartpresent> for bool {
    #[inline(always)]
    fn from(variant: Usartpresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USARTPRESENT` reader - USART present indicator. This field is Read-only."]
pub type UsartpresentR = crate::BitReader<Usartpresent>;
impl UsartpresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usartpresent {
        match self.bits {
            false => Usartpresent::NotPresent,
            true => Usartpresent::Present,
        }
    }
    #[doc = "This Flexcomm does not include the USART function."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Usartpresent::NotPresent
    }
    #[doc = "This Flexcomm includes the USART function."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Usartpresent::Present
    }
}
#[doc = "SPI present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spipresent {
    #[doc = "0: This Flexcomm does not include the SPI function."]
    NotPresent = 0,
    #[doc = "1: This Flexcomm includes the SPI function."]
    Present = 1,
}
impl From<Spipresent> for bool {
    #[inline(always)]
    fn from(variant: Spipresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIPRESENT` reader - SPI present indicator. This field is Read-only."]
pub type SpipresentR = crate::BitReader<Spipresent>;
impl SpipresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spipresent {
        match self.bits {
            false => Spipresent::NotPresent,
            true => Spipresent::Present,
        }
    }
    #[doc = "This Flexcomm does not include the SPI function."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Spipresent::NotPresent
    }
    #[doc = "This Flexcomm includes the SPI function."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Spipresent::Present
    }
}
#[doc = "I2C present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cpresent {
    #[doc = "0: This Flexcomm does not include the I2C function."]
    NotPresent = 0,
    #[doc = "1: This Flexcomm includes the I2C function."]
    Present = 1,
}
impl From<I2cpresent> for bool {
    #[inline(always)]
    fn from(variant: I2cpresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CPRESENT` reader - I2C present indicator. This field is Read-only."]
pub type I2cpresentR = crate::BitReader<I2cpresent>;
impl I2cpresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cpresent {
        match self.bits {
            false => I2cpresent::NotPresent,
            true => I2cpresent::Present,
        }
    }
    #[doc = "This Flexcomm does not include the I2C function."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2cpresent::NotPresent
    }
    #[doc = "This Flexcomm includes the I2C function."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2cpresent::Present
    }
}
#[doc = "I 2S present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2spresent {
    #[doc = "0: This Flexcomm does not include the I2S function."]
    NotPresent = 0,
    #[doc = "1: This Flexcomm includes the I2S function."]
    Present = 1,
}
impl From<I2spresent> for bool {
    #[inline(always)]
    fn from(variant: I2spresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SPRESENT` reader - I 2S present indicator. This field is Read-only."]
pub type I2spresentR = crate::BitReader<I2spresent>;
impl I2spresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2spresent {
        match self.bits {
            false => I2spresent::NotPresent,
            true => I2spresent::Present,
        }
    }
    #[doc = "This Flexcomm does not include the I2S function."]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2spresent::NotPresent
    }
    #[doc = "This Flexcomm includes the I2S function."]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2spresent::Present
    }
}
#[doc = "Field `ID` reader - Flexcomm ID."]
pub type IdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&self) -> PerselR {
        PerselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn usartpresent(&self) -> UsartpresentR {
        UsartpresentR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn spipresent(&self) -> SpipresentR {
        SpipresentR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2cpresent(&self) -> I2cpresentR {
        I2cpresentR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2spresent(&self) -> I2spresentR {
        I2spresentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Flexcomm ID."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSELID")
            .field("persel", &self.persel())
            .field("lock", &self.lock())
            .field("usartpresent", &self.usartpresent())
            .field("spipresent", &self.spipresent())
            .field("i2cpresent", &self.i2cpresent())
            .field("i2spresent", &self.i2spresent())
            .field("id", &self.id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    #[must_use]
    pub fn persel(&mut self) -> PerselW<PselidSpec> {
        PerselW::new(self, 0)
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<PselidSpec> {
        LockW::new(self, 3)
    }
}
#[doc = "Peripheral Select and Flexcomm ID register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pselid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselidSpec;
impl crate::RegisterSpec for PselidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselid::R`](R) reader structure"]
impl crate::Readable for PselidSpec {}
#[doc = "`write(|w| ..)` method takes [`pselid::W`](W) writer structure"]
impl crate::Writable for PselidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSELID to value 0x0010_1000"]
impl crate::Resettable for PselidSpec {
    const RESET_VALUE: u32 = 0x0010_1000;
}
