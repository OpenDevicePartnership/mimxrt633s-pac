#[doc = "Register `SLVCTL` reader"]
pub type R = crate::R<SlvctlSpec>;
#[doc = "Register `SLVCTL` writer"]
pub type W = crate::W<SlvctlSpec>;
#[doc = "Slave Continue.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvcontinue {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    Continue = 1,
}
impl From<Slvcontinue> for bool {
    #[inline(always)]
    fn from(variant: Slvcontinue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVCONTINUE` reader - Slave Continue."]
pub type SlvcontinueR = crate::BitReader<Slvcontinue>;
impl SlvcontinueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvcontinue {
        match self.bits {
            false => Slvcontinue::NoEffect,
            true => Slvcontinue::Continue,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Slvcontinue::NoEffect
    }
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == Slvcontinue::Continue
    }
}
#[doc = "Field `SLVCONTINUE` writer - Slave Continue."]
pub type SlvcontinueW<'a, REG> = crate::BitWriter<'a, REG, Slvcontinue>;
impl<'a, REG> SlvcontinueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Slvcontinue::NoEffect)
    }
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Slvcontinue::Continue)
    }
}
#[doc = "Slave NACK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvnack {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    Nack = 1,
}
impl From<Slvnack> for bool {
    #[inline(always)]
    fn from(variant: Slvnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVNACK` reader - Slave NACK."]
pub type SlvnackR = crate::BitReader<Slvnack>;
impl SlvnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvnack {
        match self.bits {
            false => Slvnack::NoEffect,
            true => Slvnack::Nack,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Slvnack::NoEffect
    }
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == Slvnack::Nack
    }
}
#[doc = "Field `SLVNACK` writer - Slave NACK."]
pub type SlvnackW<'a, REG> = crate::BitWriter<'a, REG, Slvnack>;
impl<'a, REG> SlvnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnack::NoEffect)
    }
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnack::Nack)
    }
}
#[doc = "Slave DMA enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvdma {
    #[doc = "0: Disabled. No DMA requests are issued for Slave mode operation."]
    Disabled = 0,
    #[doc = "1: Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    Enabled = 1,
}
impl From<Slvdma> for bool {
    #[inline(always)]
    fn from(variant: Slvdma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVDMA` reader - Slave DMA enable."]
pub type SlvdmaR = crate::BitReader<Slvdma>;
impl SlvdmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvdma {
        match self.bits {
            false => Slvdma::Disabled,
            true => Slvdma::Enabled,
        }
    }
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slvdma::Disabled
    }
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slvdma::Enabled
    }
}
#[doc = "Field `SLVDMA` writer - Slave DMA enable."]
pub type SlvdmaW<'a, REG> = crate::BitWriter<'a, REG, Slvdma>;
impl<'a, REG> SlvdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvdma::Disabled)
    }
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slvdma::Enabled)
    }
}
#[doc = "Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoack {
    #[doc = "0: Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    Normal = 0,
    #[doc = "1: A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    AutomaticAck = 1,
}
impl From<Autoack> for bool {
    #[inline(always)]
    fn from(variant: Autoack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOACK` reader - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
pub type AutoackR = crate::BitReader<Autoack>;
impl AutoackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoack {
        match self.bits {
            false => Autoack::Normal,
            true => Autoack::AutomaticAck,
        }
    }
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Autoack::Normal
    }
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    #[inline(always)]
    pub fn is_automatic_ack(&self) -> bool {
        *self == Autoack::AutomaticAck
    }
}
#[doc = "Field `AUTOACK` writer - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
pub type AutoackW<'a, REG> = crate::BitWriter<'a, REG, Autoack>;
impl<'a, REG> AutoackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Autoack::Normal)
    }
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    #[inline(always)]
    pub fn automatic_ack(self) -> &'a mut crate::W<REG> {
        self.variant(Autoack::AutomaticAck)
    }
}
#[doc = "When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Automatchread {
    #[doc = "0: The expected next operation in Automatic Mode is an I2C write."]
    I2cWrite = 0,
    #[doc = "1: The expected next operation in Automatic Mode is an I2C read."]
    I2cRead = 1,
}
impl From<Automatchread> for bool {
    #[inline(always)]
    fn from(variant: Automatchread) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOMATCHREAD` reader - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
pub type AutomatchreadR = crate::BitReader<Automatchread>;
impl AutomatchreadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Automatchread {
        match self.bits {
            false => Automatchread::I2cWrite,
            true => Automatchread::I2cRead,
        }
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    #[inline(always)]
    pub fn is_i2c_write(&self) -> bool {
        *self == Automatchread::I2cWrite
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    #[inline(always)]
    pub fn is_i2c_read(&self) -> bool {
        *self == Automatchread::I2cRead
    }
}
#[doc = "Field `AUTOMATCHREAD` writer - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
pub type AutomatchreadW<'a, REG> = crate::BitWriter<'a, REG, Automatchread>;
impl<'a, REG> AutomatchreadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    #[inline(always)]
    pub fn i2c_write(self) -> &'a mut crate::W<REG> {
        self.variant(Automatchread::I2cWrite)
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    #[inline(always)]
    pub fn i2c_read(self) -> &'a mut crate::W<REG> {
        self.variant(Automatchread::I2cRead)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&self) -> SlvcontinueR {
        SlvcontinueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&self) -> SlvnackR {
        SlvnackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline(always)]
    pub fn slvdma(&self) -> SlvdmaR {
        SlvdmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub fn autoack(&self) -> AutoackR {
        AutoackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub fn automatchread(&self) -> AutomatchreadR {
        AutomatchreadR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVCTL")
            .field("slvcontinue", &self.slvcontinue())
            .field("slvnack", &self.slvnack())
            .field("slvdma", &self.slvdma())
            .field("autoack", &self.autoack())
            .field("automatchread", &self.automatchread())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    #[must_use]
    pub fn slvcontinue(&mut self) -> SlvcontinueW<SlvctlSpec> {
        SlvcontinueW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    #[must_use]
    pub fn slvnack(&mut self) -> SlvnackW<SlvctlSpec> {
        SlvnackW::new(self, 1)
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvdma(&mut self) -> SlvdmaW<SlvctlSpec> {
        SlvdmaW::new(self, 3)
    }
    #[doc = "Bit 8 - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn autoack(&mut self) -> AutoackW<SlvctlSpec> {
        AutoackW::new(self, 8)
    }
    #[doc = "Bit 9 - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    #[must_use]
    pub fn automatchread(&mut self) -> AutomatchreadW<SlvctlSpec> {
        AutomatchreadW::new(self, 9)
    }
}
#[doc = "Slave control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvctlSpec;
impl crate::RegisterSpec for SlvctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvctl::R`](R) reader structure"]
impl crate::Readable for SlvctlSpec {}
#[doc = "`write(|w| ..)` method takes [`slvctl::W`](W) writer structure"]
impl crate::Writable for SlvctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLVCTL to value 0"]
impl crate::Resettable for SlvctlSpec {
    const RESET_VALUE: u32 = 0;
}
