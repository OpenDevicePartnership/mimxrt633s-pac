#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msten {
    #[doc = "0: Disabled. The I2C Master function is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The I2C Master function is enabled."]
    Enabled = 1,
}
impl From<Msten> for bool {
    #[inline(always)]
    fn from(variant: Msten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTEN` reader - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
pub type MstenR = crate::BitReader<Msten>;
impl MstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msten {
        match self.bits {
            false => Msten::Disabled,
            true => Msten::Enabled,
        }
    }
    #[doc = "Disabled. The I2C Master function is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msten::Disabled
    }
    #[doc = "Enabled. The I2C Master function is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msten::Enabled
    }
}
#[doc = "Field `MSTEN` writer - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
pub type MstenW<'a, REG> = crate::BitWriter<'a, REG, Msten>;
impl<'a, REG> MstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The I2C Master function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msten::Disabled)
    }
    #[doc = "Enabled. The I2C Master function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msten::Enabled)
    }
}
#[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slven {
    #[doc = "0: Disabled. The I2C slave function is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The I2C slave function is enabled."]
    Enabled = 1,
}
impl From<Slven> for bool {
    #[inline(always)]
    fn from(variant: Slven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVEN` reader - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
pub type SlvenR = crate::BitReader<Slven>;
impl SlvenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slven {
        match self.bits {
            false => Slven::Disabled,
            true => Slven::Enabled,
        }
    }
    #[doc = "Disabled. The I2C slave function is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Slven::Disabled
    }
    #[doc = "Enabled. The I2C slave function is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Slven::Enabled
    }
}
#[doc = "Field `SLVEN` writer - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
pub type SlvenW<'a, REG> = crate::BitWriter<'a, REG, Slven>;
impl<'a, REG> SlvenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The I2C slave function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slven::Disabled)
    }
    #[doc = "Enabled. The I2C slave function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Slven::Enabled)
    }
}
#[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monen {
    #[doc = "0: Disabled. The I2C Monitor function is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. The I2C Monitor function is enabled."]
    Enabled = 1,
}
impl From<Monen> for bool {
    #[inline(always)]
    fn from(variant: Monen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONEN` reader - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
pub type MonenR = crate::BitReader<Monen>;
impl MonenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monen {
        match self.bits {
            false => Monen::Disabled,
            true => Monen::Enabled,
        }
    }
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Monen::Disabled
    }
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Monen::Enabled
    }
}
#[doc = "Field `MONEN` writer - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
pub type MonenW<'a, REG> = crate::BitWriter<'a, REG, Monen>;
impl<'a, REG> MonenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monen::Disabled)
    }
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monen::Enabled)
    }
}
#[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeouten {
    #[doc = "0: Disabled. Time-out function is disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    Enabled = 1,
}
impl From<Timeouten> for bool {
    #[inline(always)]
    fn from(variant: Timeouten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTEN` reader - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
pub type TimeoutenR = crate::BitReader<Timeouten>;
impl TimeoutenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeouten {
        match self.bits {
            false => Timeouten::Disabled,
            true => Timeouten::Enabled,
        }
    }
    #[doc = "Disabled. Time-out function is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Timeouten::Disabled
    }
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Timeouten::Enabled
    }
}
#[doc = "Field `TIMEOUTEN` writer - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
pub type TimeoutenW<'a, REG> = crate::BitWriter<'a, REG, Timeouten>;
impl<'a, REG> TimeoutenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Time-out function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouten::Disabled)
    }
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Timeouten::Enabled)
    }
}
#[doc = "Monitor function Clock Stretching.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monclkstr {
    #[doc = "0: Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    Disabled = 0,
    #[doc = "1: Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    Enabled = 1,
}
impl From<Monclkstr> for bool {
    #[inline(always)]
    fn from(variant: Monclkstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONCLKSTR` reader - Monitor function Clock Stretching."]
pub type MonclkstrR = crate::BitReader<Monclkstr>;
impl MonclkstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monclkstr {
        match self.bits {
            false => Monclkstr::Disabled,
            true => Monclkstr::Enabled,
        }
    }
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Monclkstr::Disabled
    }
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Monclkstr::Enabled
    }
}
#[doc = "Field `MONCLKSTR` writer - Monitor function Clock Stretching."]
pub type MonclkstrW<'a, REG> = crate::BitWriter<'a, REG, Monclkstr>;
impl<'a, REG> MonclkstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monclkstr::Disabled)
    }
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Monclkstr::Enabled)
    }
}
#[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hscapable {
    #[doc = "0: Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    FastModePlus = 0,
    #[doc = "1: High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HighSpeed = 1,
}
impl From<Hscapable> for bool {
    #[inline(always)]
    fn from(variant: Hscapable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCAPABLE` reader - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
pub type HscapableR = crate::BitReader<Hscapable>;
impl HscapableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hscapable {
        match self.bits {
            false => Hscapable::FastModePlus,
            true => Hscapable::HighSpeed,
        }
    }
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == Hscapable::FastModePlus
    }
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == Hscapable::HighSpeed
    }
}
#[doc = "Field `HSCAPABLE` writer - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
pub type HscapableW<'a, REG> = crate::BitWriter<'a, REG, Hscapable>;
impl<'a, REG> HscapableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut crate::W<REG> {
        self.variant(Hscapable::FastModePlus)
    }
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(Hscapable::HighSpeed)
    }
}
impl R {
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&self) -> MstenR {
        MstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&self) -> SlvenR {
        SlvenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&self) -> MonenR {
        MonenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&self) -> TimeoutenR {
        TimeoutenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&self) -> MonclkstrR {
        MonclkstrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&self) -> HscapableR {
        HscapableR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("msten", &self.msten())
            .field("slven", &self.slven())
            .field("monen", &self.monen())
            .field("timeouten", &self.timeouten())
            .field("monclkstr", &self.monclkstr())
            .field("hscapable", &self.hscapable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&mut self) -> MstenW<CfgSpec> {
        MstenW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&mut self) -> SlvenW<CfgSpec> {
        SlvenW::new(self, 1)
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&mut self) -> MonenW<CfgSpec> {
        MonenW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&mut self) -> TimeoutenW<CfgSpec> {
        TimeoutenW::new(self, 3)
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&mut self) -> MonclkstrW<CfgSpec> {
        MonclkstrW::new(self, 4)
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&mut self) -> HscapableW<CfgSpec> {
        HscapableW::new(self, 5)
    }
}
#[doc = "Configuration for shared functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
