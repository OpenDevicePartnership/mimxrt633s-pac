#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: The transmitter/receiver for channel pair is currently idle."]
    Idle = 0,
    #[doc = "1: The transmitter/receiver for channel pair is currently processing data."]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Busy,
        }
    }
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
#[doc = "Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvfrmerr {
    #[doc = "0: No error has been recorded."]
    NoError = 0,
    #[doc = "1: An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    Error = 1,
}
impl From<Slvfrmerr> for bool {
    #[inline(always)]
    fn from(variant: Slvfrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVFRMERR` writer - Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
pub type SlvfrmerrW<'a, REG> = crate::BitWriter<'a, REG, Slvfrmerr>;
impl<'a, REG> SlvfrmerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error has been recorded."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Slvfrmerr::NoError)
    }
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Slvfrmerr::Error)
    }
}
#[doc = "Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lr {
    #[doc = "0: Left channel."]
    LeftChannel = 0,
    #[doc = "1: Right channel."]
    RightChannel = 1,
}
impl From<Lr> for bool {
    #[inline(always)]
    fn from(variant: Lr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LR` reader - Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
pub type LrR = crate::BitReader<Lr>;
impl LrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lr {
        match self.bits {
            false => Lr::LeftChannel,
            true => Lr::RightChannel,
        }
    }
    #[doc = "Left channel."]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        *self == Lr::LeftChannel
    }
    #[doc = "Right channel."]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        *self == Lr::RightChannel
    }
}
#[doc = "Data Paused status flag. Applies to all I2S channels\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datapaused {
    #[doc = "0: Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description of the DATAPAUSE control bit in the CFG1 register."]
    NotPaused = 0,
    #[doc = "1: A data pause has been requested and is now in force."]
    Paused = 1,
}
impl From<Datapaused> for bool {
    #[inline(always)]
    fn from(variant: Datapaused) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAPAUSED` reader - Data Paused status flag. Applies to all I2S channels"]
pub type DatapausedR = crate::BitReader<Datapaused>;
impl DatapausedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datapaused {
        match self.bits {
            false => Datapaused::NotPaused,
            true => Datapaused::Paused,
        }
    }
    #[doc = "Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description of the DATAPAUSE control bit in the CFG1 register."]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        *self == Datapaused::NotPaused
    }
    #[doc = "A data pause has been requested and is now in force."]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        *self == Datapaused::Paused
    }
}
impl R {
    #[doc = "Bit 0 - Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[inline(always)]
    pub fn lr(&self) -> LrR {
        LrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Paused status flag. Applies to all I2S channels"]
    #[inline(always)]
    pub fn datapaused(&self) -> DatapausedR {
        DatapausedR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("busy", &self.busy())
            .field("lr", &self.lr())
            .field("datapaused", &self.datapaused())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[inline(always)]
    #[must_use]
    pub fn slvfrmerr(&mut self) -> SlvfrmerrW<StatSpec> {
        SlvfrmerrW::new(self, 1)
    }
}
#[doc = "Status register for the primary channel pair.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
