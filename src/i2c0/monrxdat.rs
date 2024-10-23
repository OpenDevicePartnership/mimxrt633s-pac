#[doc = "Register `MONRXDAT` reader"]
pub type R = crate::R<MonrxdatSpec>;
#[doc = "Field `MONRXDAT` reader - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
pub type MonrxdatR = crate::FieldReader;
#[doc = "Monitor Received Start.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monstart {
    #[doc = "0: No start detected. The Monitor function has not detected a Start event on the I2C bus."]
    NoStartDetected = 0,
    #[doc = "1: Start detected. The Monitor function has detected a Start event on the I2C bus."]
    StartDetected = 1,
}
impl From<Monstart> for bool {
    #[inline(always)]
    fn from(variant: Monstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONSTART` reader - Monitor Received Start."]
pub type MonstartR = crate::BitReader<Monstart>;
impl MonstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monstart {
        match self.bits {
            false => Monstart::NoStartDetected,
            true => Monstart::StartDetected,
        }
    }
    #[doc = "No start detected. The Monitor function has not detected a Start event on the I2C bus."]
    #[inline(always)]
    pub fn is_no_start_detected(&self) -> bool {
        *self == Monstart::NoStartDetected
    }
    #[doc = "Start detected. The Monitor function has detected a Start event on the I2C bus."]
    #[inline(always)]
    pub fn is_start_detected(&self) -> bool {
        *self == Monstart::StartDetected
    }
}
#[doc = "Monitor Received Repeated Start.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monrestart {
    #[doc = "0: No repeated start detected. The Monitor function has not detected a Repeated Start event on the I2C bus."]
    NotDetected = 0,
    #[doc = "1: Repeated start detected. The Monitor function has detected a Repeated Start event on the I2C bus."]
    Detected = 1,
}
impl From<Monrestart> for bool {
    #[inline(always)]
    fn from(variant: Monrestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONRESTART` reader - Monitor Received Repeated Start."]
pub type MonrestartR = crate::BitReader<Monrestart>;
impl MonrestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monrestart {
        match self.bits {
            false => Monrestart::NotDetected,
            true => Monrestart::Detected,
        }
    }
    #[doc = "No repeated start detected. The Monitor function has not detected a Repeated Start event on the I2C bus."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Monrestart::NotDetected
    }
    #[doc = "Repeated start detected. The Monitor function has detected a Repeated Start event on the I2C bus."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Monrestart::Detected
    }
}
#[doc = "Monitor Received NACK.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Monnack {
    #[doc = "0: Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    Acknowledged = 0,
    #[doc = "1: Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    NotAcknowledged = 1,
}
impl From<Monnack> for bool {
    #[inline(always)]
    fn from(variant: Monnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONNACK` reader - Monitor Received NACK."]
pub type MonnackR = crate::BitReader<Monnack>;
impl MonnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Monnack {
        match self.bits {
            false => Monnack::Acknowledged,
            true => Monnack::NotAcknowledged,
        }
    }
    #[doc = "Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    #[inline(always)]
    pub fn is_acknowledged(&self) -> bool {
        *self == Monnack::Acknowledged
    }
    #[doc = "Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    #[inline(always)]
    pub fn is_not_acknowledged(&self) -> bool {
        *self == Monnack::NotAcknowledged
    }
}
impl R {
    #[doc = "Bits 0:7 - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub fn monrxdat(&self) -> MonrxdatR {
        MonrxdatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Monitor Received Start."]
    #[inline(always)]
    pub fn monstart(&self) -> MonstartR {
        MonstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Monitor Received Repeated Start."]
    #[inline(always)]
    pub fn monrestart(&self) -> MonrestartR {
        MonrestartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Monitor Received NACK."]
    #[inline(always)]
    pub fn monnack(&self) -> MonnackR {
        MonnackR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONRXDAT")
            .field("monrxdat", &self.monrxdat())
            .field("monstart", &self.monstart())
            .field("monrestart", &self.monrestart())
            .field("monnack", &self.monnack())
            .finish()
    }
}
#[doc = "Monitor receiver data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`monrxdat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonrxdatSpec;
impl crate::RegisterSpec for MonrxdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monrxdat::R`](R) reader structure"]
impl crate::Readable for MonrxdatSpec {}
#[doc = "`reset()` method sets MONRXDAT to value 0"]
impl crate::Resettable for MonrxdatSpec {
    const RESET_VALUE: u32 = 0;
}
