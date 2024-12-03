#[doc = "Register `MSTCTL` reader"]
pub type R = crate::R<MstctlSpec>;
#[doc = "Register `MSTCTL` writer"]
pub type W = crate::W<MstctlSpec>;
#[doc = "Master Continue. This bit is write-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstcontinue {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    Continue = 1,
}
impl From<Mstcontinue> for bool {
    #[inline(always)]
    fn from(variant: Mstcontinue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTCONTINUE` writer - Master Continue. This bit is write-only."]
pub type MstcontinueW<'a, REG> = crate::BitWriter<'a, REG, Mstcontinue>;
impl<'a, REG> MstcontinueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mstcontinue::NoEffect)
    }
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(Mstcontinue::Continue)
    }
}
#[doc = "Master Start control. This bit is write-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mststart {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Start. A Start will be generated on the I2C bus at the next allowed time."]
    Start = 1,
}
impl From<Mststart> for bool {
    #[inline(always)]
    fn from(variant: Mststart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSTART` reader - Master Start control. This bit is write-only."]
pub type MststartR = crate::BitReader<Mststart>;
impl MststartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mststart {
        match self.bits {
            false => Mststart::NoEffect,
            true => Mststart::Start,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Mststart::NoEffect
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Mststart::Start
    }
}
#[doc = "Field `MSTSTART` writer - Master Start control. This bit is write-only."]
pub type MststartW<'a, REG> = crate::BitWriter<'a, REG, Mststart>;
impl<'a, REG> MststartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mststart::NoEffect)
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Mststart::Start)
    }
}
#[doc = "Master Stop control. This bit is write-only.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mststop {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    Stop = 1,
}
impl From<Mststop> for bool {
    #[inline(always)]
    fn from(variant: Mststop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSTOP` reader - Master Stop control. This bit is write-only."]
pub type MststopR = crate::BitReader<Mststop>;
impl MststopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mststop {
        match self.bits {
            false => Mststop::NoEffect,
            true => Mststop::Stop,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Mststop::NoEffect
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Mststop::Stop
    }
}
#[doc = "Field `MSTSTOP` writer - Master Stop control. This bit is write-only."]
pub type MststopW<'a, REG> = crate::BitWriter<'a, REG, Mststop>;
impl<'a, REG> MststopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mststop::NoEffect)
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Mststop::Stop)
    }
}
#[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstdma {
    #[doc = "0: Disable. No DMA requests are generated for master operation."]
    Disabled = 0,
    #[doc = "1: Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    Enabled = 1,
}
impl From<Mstdma> for bool {
    #[inline(always)]
    fn from(variant: Mstdma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTDMA` reader - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
pub type MstdmaR = crate::BitReader<Mstdma>;
impl MstdmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstdma {
        match self.bits {
            false => Mstdma::Disabled,
            true => Mstdma::Enabled,
        }
    }
    #[doc = "Disable. No DMA requests are generated for master operation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mstdma::Disabled
    }
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mstdma::Enabled
    }
}
#[doc = "Field `MSTDMA` writer - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
pub type MstdmaW<'a, REG> = crate::BitWriter<'a, REG, Mstdma>;
impl<'a, REG> MstdmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable. No DMA requests are generated for master operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstdma::Disabled)
    }
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mstdma::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&self) -> MststartR {
        MststartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&self) -> MststopR {
        MststopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&self) -> MstdmaR {
        MstdmaR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTCTL")
            .field("mststart", &self.mststart())
            .field("mststop", &self.mststop())
            .field("mstdma", &self.mstdma())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline(always)]
    pub fn mstcontinue(&mut self) -> MstcontinueW<MstctlSpec> {
        MstcontinueW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&mut self) -> MststartW<MstctlSpec> {
        MststartW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&mut self) -> MststopW<MstctlSpec> {
        MststopW::new(self, 2)
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&mut self) -> MstdmaW<MstctlSpec> {
        MstdmaW::new(self, 3)
    }
}
#[doc = "Master control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstctlSpec;
impl crate::RegisterSpec for MstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstctl::R`](R) reader structure"]
impl crate::Readable for MstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mstctl::W`](W) writer structure"]
impl crate::Writable for MstctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTCTL to value 0"]
impl crate::Resettable for MstctlSpec {
    const RESET_VALUE: u32 = 0;
}
