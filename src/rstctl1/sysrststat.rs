#[doc = "Register `SYSRSTSTAT` reader"]
pub type R = crate::R<SysrststatSpec>;
#[doc = "Register `SYSRSTSTAT` writer"]
pub type W = crate::W<SysrststatSpec>;
#[doc = "VDD POR Event Detected:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VddPor {
    #[doc = "0: No event detected."]
    NoPorDetected = 0,
    #[doc = "1: VDD POR event detected. (Writing a 1 to this bit clears this status)."]
    PorDetected = 1,
}
impl From<VddPor> for bool {
    #[inline(always)]
    fn from(variant: VddPor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDD_POR` reader - VDD POR Event Detected:"]
pub type VddPorR = crate::BitReader<VddPor>;
impl VddPorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VddPor {
        match self.bits {
            false => VddPor::NoPorDetected,
            true => VddPor::PorDetected,
        }
    }
    #[doc = "No event detected."]
    #[inline(always)]
    pub fn is_no_por_detected(&self) -> bool {
        *self == VddPor::NoPorDetected
    }
    #[doc = "VDD POR event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn is_por_detected(&self) -> bool {
        *self == VddPor::PorDetected
    }
}
#[doc = "Field `VDD_POR` writer - VDD POR Event Detected:"]
pub type VddPorW<'a, REG> = crate::BitWriter<'a, REG, VddPor>;
impl<'a, REG> VddPorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event detected."]
    #[inline(always)]
    pub fn no_por_detected(self) -> &'a mut crate::W<REG> {
        self.variant(VddPor::NoPorDetected)
    }
    #[doc = "VDD POR event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn por_detected(self) -> &'a mut crate::W<REG> {
        self.variant(VddPor::PorDetected)
    }
}
#[doc = "PAD RESET Event Detected:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PadReset {
    #[doc = "0: No EVENT Detected."]
    NoEventDetected = 0,
    #[doc = "1: RESET Detected. (Write 1 to CLR),"]
    ResetDetected = 1,
}
impl From<PadReset> for bool {
    #[inline(always)]
    fn from(variant: PadReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD_RESET` reader - PAD RESET Event Detected:"]
pub type PadResetR = crate::BitReader<PadReset>;
impl PadResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PadReset {
        match self.bits {
            false => PadReset::NoEventDetected,
            true => PadReset::ResetDetected,
        }
    }
    #[doc = "No EVENT Detected."]
    #[inline(always)]
    pub fn is_no_event_detected(&self) -> bool {
        *self == PadReset::NoEventDetected
    }
    #[doc = "RESET Detected. (Write 1 to CLR),"]
    #[inline(always)]
    pub fn is_reset_detected(&self) -> bool {
        *self == PadReset::ResetDetected
    }
}
#[doc = "Field `PAD_RESET` writer - PAD RESET Event Detected:"]
pub type PadResetW<'a, REG> = crate::BitWriter<'a, REG, PadReset>;
impl<'a, REG> PadResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No EVENT Detected."]
    #[inline(always)]
    pub fn no_event_detected(self) -> &'a mut crate::W<REG> {
        self.variant(PadReset::NoEventDetected)
    }
    #[doc = "RESET Detected. (Write 1 to CLR),"]
    #[inline(always)]
    pub fn reset_detected(self) -> &'a mut crate::W<REG> {
        self.variant(PadReset::ResetDetected)
    }
}
#[doc = "ARM RESET Event Detected:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmApdReset {
    #[doc = "0: No event detected."]
    NoEventDetected = 0,
    #[doc = "1: ARM reset event detected. (Writing a 1 to this bit clears this status)."]
    ResetDetected = 1,
}
impl From<ArmApdReset> for bool {
    #[inline(always)]
    fn from(variant: ArmApdReset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARM_APD_RESET` reader - ARM RESET Event Detected:"]
pub type ArmApdResetR = crate::BitReader<ArmApdReset>;
impl ArmApdResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArmApdReset {
        match self.bits {
            false => ArmApdReset::NoEventDetected,
            true => ArmApdReset::ResetDetected,
        }
    }
    #[doc = "No event detected."]
    #[inline(always)]
    pub fn is_no_event_detected(&self) -> bool {
        *self == ArmApdReset::NoEventDetected
    }
    #[doc = "ARM reset event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn is_reset_detected(&self) -> bool {
        *self == ArmApdReset::ResetDetected
    }
}
#[doc = "Field `ARM_APD_RESET` writer - ARM RESET Event Detected:"]
pub type ArmApdResetW<'a, REG> = crate::BitWriter<'a, REG, ArmApdReset>;
impl<'a, REG> ArmApdResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event detected."]
    #[inline(always)]
    pub fn no_event_detected(self) -> &'a mut crate::W<REG> {
        self.variant(ArmApdReset::NoEventDetected)
    }
    #[doc = "ARM reset event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn reset_detected(self) -> &'a mut crate::W<REG> {
        self.variant(ArmApdReset::ResetDetected)
    }
}
#[doc = "WDT0 RESET Event Detected:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt0Reset {
    #[doc = "0: No EVENT Detected."]
    NoEventDetected = 0,
    #[doc = "1: WDT0 reset event detected. (Writing a 1 to this bit clears this status)."]
    Wdt0ResetDetected = 1,
}
impl From<Wdt0Reset> for bool {
    #[inline(always)]
    fn from(variant: Wdt0Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT0_RESET` reader - WDT0 RESET Event Detected:"]
pub type Wdt0ResetR = crate::BitReader<Wdt0Reset>;
impl Wdt0ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt0Reset {
        match self.bits {
            false => Wdt0Reset::NoEventDetected,
            true => Wdt0Reset::Wdt0ResetDetected,
        }
    }
    #[doc = "No EVENT Detected."]
    #[inline(always)]
    pub fn is_no_event_detected(&self) -> bool {
        *self == Wdt0Reset::NoEventDetected
    }
    #[doc = "WDT0 reset event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn is_wdt0_reset_detected(&self) -> bool {
        *self == Wdt0Reset::Wdt0ResetDetected
    }
}
#[doc = "Field `WDT0_RESET` writer - WDT0 RESET Event Detected:"]
pub type Wdt0ResetW<'a, REG> = crate::BitWriter<'a, REG, Wdt0Reset>;
impl<'a, REG> Wdt0ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No EVENT Detected."]
    #[inline(always)]
    pub fn no_event_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt0Reset::NoEventDetected)
    }
    #[doc = "WDT0 reset event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn wdt0_reset_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt0Reset::Wdt0ResetDetected)
    }
}
#[doc = "WDT1 RESET Event Detected:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt1Reset {
    #[doc = "0: No EVENT Detected."]
    NoEventDetected = 0,
    #[doc = "1: WDT1 reset event detected. (Writing a 1 to this bit clears this status)."]
    Wdt1ResetDetected = 1,
}
impl From<Wdt1Reset> for bool {
    #[inline(always)]
    fn from(variant: Wdt1Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT1_RESET` reader - WDT1 RESET Event Detected:"]
pub type Wdt1ResetR = crate::BitReader<Wdt1Reset>;
impl Wdt1ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt1Reset {
        match self.bits {
            false => Wdt1Reset::NoEventDetected,
            true => Wdt1Reset::Wdt1ResetDetected,
        }
    }
    #[doc = "No EVENT Detected."]
    #[inline(always)]
    pub fn is_no_event_detected(&self) -> bool {
        *self == Wdt1Reset::NoEventDetected
    }
    #[doc = "WDT1 reset event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn is_wdt1_reset_detected(&self) -> bool {
        *self == Wdt1Reset::Wdt1ResetDetected
    }
}
#[doc = "Field `WDT1_RESET` writer - WDT1 RESET Event Detected:"]
pub type Wdt1ResetW<'a, REG> = crate::BitWriter<'a, REG, Wdt1Reset>;
impl<'a, REG> Wdt1ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No EVENT Detected."]
    #[inline(always)]
    pub fn no_event_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt1Reset::NoEventDetected)
    }
    #[doc = "WDT1 reset event detected. (Writing a 1 to this bit clears this status)."]
    #[inline(always)]
    pub fn wdt1_reset_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt1Reset::Wdt1ResetDetected)
    }
}
impl R {
    #[doc = "Bit 0 - VDD POR Event Detected:"]
    #[inline(always)]
    pub fn vdd_por(&self) -> VddPorR {
        VddPorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PAD RESET Event Detected:"]
    #[inline(always)]
    pub fn pad_reset(&self) -> PadResetR {
        PadResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARM RESET Event Detected:"]
    #[inline(always)]
    pub fn arm_apd_reset(&self) -> ArmApdResetR {
        ArmApdResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDT0 RESET Event Detected:"]
    #[inline(always)]
    pub fn wdt0_reset(&self) -> Wdt0ResetR {
        Wdt0ResetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDT1 RESET Event Detected:"]
    #[inline(always)]
    pub fn wdt1_reset(&self) -> Wdt1ResetR {
        Wdt1ResetR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSRSTSTAT")
            .field("vdd_por", &self.vdd_por())
            .field("pad_reset", &self.pad_reset())
            .field("arm_apd_reset", &self.arm_apd_reset())
            .field("wdt0_reset", &self.wdt0_reset())
            .field("wdt1_reset", &self.wdt1_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VDD POR Event Detected:"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_por(&mut self) -> VddPorW<SysrststatSpec> {
        VddPorW::new(self, 0)
    }
    #[doc = "Bit 4 - PAD RESET Event Detected:"]
    #[inline(always)]
    #[must_use]
    pub fn pad_reset(&mut self) -> PadResetW<SysrststatSpec> {
        PadResetW::new(self, 4)
    }
    #[doc = "Bit 5 - ARM RESET Event Detected:"]
    #[inline(always)]
    #[must_use]
    pub fn arm_apd_reset(&mut self) -> ArmApdResetW<SysrststatSpec> {
        ArmApdResetW::new(self, 5)
    }
    #[doc = "Bit 6 - WDT0 RESET Event Detected:"]
    #[inline(always)]
    #[must_use]
    pub fn wdt0_reset(&mut self) -> Wdt0ResetW<SysrststatSpec> {
        Wdt0ResetW::new(self, 6)
    }
    #[doc = "Bit 7 - WDT1 RESET Event Detected:"]
    #[inline(always)]
    #[must_use]
    pub fn wdt1_reset(&mut self) -> Wdt1ResetW<SysrststatSpec> {
        Wdt1ResetW::new(self, 7)
    }
}
#[doc = "system reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrststat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrststat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrststatSpec;
impl crate::RegisterSpec for SysrststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysrststat::R`](R) reader structure"]
impl crate::Readable for SysrststatSpec {}
#[doc = "`write(|w| ..)` method takes [`sysrststat::W`](W) writer structure"]
impl crate::Writable for SysrststatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSRSTSTAT to value 0x01"]
impl crate::Resettable for SysrststatSpec {
    const RESET_VALUE: u32 = 0x01;
}
