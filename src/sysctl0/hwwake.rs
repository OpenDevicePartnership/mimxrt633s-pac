#[doc = "Register `HWWAKE` reader"]
pub type R = crate::R<HwwakeSpec>;
#[doc = "Register `HWWAKE` writer"]
pub type W = crate::W<HwwakeSpec>;
#[doc = "Field `FORCEWAKE` reader - Force peripheral clocking to stay on during deep-sleep mode. When 1, clocking to peripherals is prevented from being shut down when the CPU enters deep-sleep mode. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
pub type ForcewakeR = crate::BitReader;
#[doc = "Field `FORCEWAKE` writer - Force peripheral clocking to stay on during deep-sleep mode. When 1, clocking to peripherals is prevented from being shut down when the CPU enters deep-sleep mode. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
pub type ForcewakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCWAKE` reader - Wake for Flexcomm Interfaces. When 1, any Flexcomm Interface FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub type FcwakeR = crate::BitReader;
#[doc = "Field `FCWAKE` writer - Wake for Flexcomm Interfaces. When 1, any Flexcomm Interface FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub type FcwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMICWAKE` reader - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub type DmicwakeR = crate::BitReader;
#[doc = "Field `DMICWAKE` writer - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub type DmicwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC0WAKE` reader - Wake for DMAC0. When 1, DMAC0 being busy will cause peripheral clocking to remain running until DMAC0 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC0 has completed its related activity."]
pub type Dmac0wakeR = crate::BitReader;
#[doc = "Field `DMAC0WAKE` writer - Wake for DMAC0. When 1, DMAC0 being busy will cause peripheral clocking to remain running until DMAC0 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC0 has completed its related activity."]
pub type Dmac0wakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC1WAKE` reader - Wake for DMAC1. When 1, DMAC1 being busy will cause peripheral clocking to remain running until DMAC1 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC1 has completed its related activity."]
pub type Dmac1wakeR = crate::BitReader;
#[doc = "Field `DMAC1WAKE` writer - Wake for DMAC1. When 1, DMAC1 being busy will cause peripheral clocking to remain running until DMAC1 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC1 has completed its related activity."]
pub type Dmac1wakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPIWAKE` reader - Wake for ESPI. When 1, ESPI being busy will cause peripheral clocking to remain running until ESPI completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before ESPI has completed its related activity."]
pub type EspiwakeR = crate::BitReader;
#[doc = "Field `ESPIWAKE` writer - Wake for ESPI. When 1, ESPI being busy will cause peripheral clocking to remain running until ESPI completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before ESPI has completed its related activity."]
pub type EspiwakeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during deep-sleep mode. When 1, clocking to peripherals is prevented from being shut down when the CPU enters deep-sleep mode. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&self) -> ForcewakeR {
        ForcewakeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake for Flexcomm Interfaces. When 1, any Flexcomm Interface FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&self) -> FcwakeR {
        FcwakeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn dmicwake(&self) -> DmicwakeR {
        DmicwakeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake for DMAC0. When 1, DMAC0 being busy will cause peripheral clocking to remain running until DMAC0 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC0 has completed its related activity."]
    #[inline(always)]
    pub fn dmac0wake(&self) -> Dmac0wakeR {
        Dmac0wakeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake for DMAC1. When 1, DMAC1 being busy will cause peripheral clocking to remain running until DMAC1 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC1 has completed its related activity."]
    #[inline(always)]
    pub fn dmac1wake(&self) -> Dmac1wakeR {
        Dmac1wakeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake for ESPI. When 1, ESPI being busy will cause peripheral clocking to remain running until ESPI completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before ESPI has completed its related activity."]
    #[inline(always)]
    pub fn espiwake(&self) -> EspiwakeR {
        EspiwakeR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWWAKE")
            .field("forcewake", &self.forcewake())
            .field("fcwake", &self.fcwake())
            .field("dmicwake", &self.dmicwake())
            .field("dmac0wake", &self.dmac0wake())
            .field("dmac1wake", &self.dmac1wake())
            .field("espiwake", &self.espiwake())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during deep-sleep mode. When 1, clocking to peripherals is prevented from being shut down when the CPU enters deep-sleep mode. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    #[must_use]
    pub fn forcewake(&mut self) -> ForcewakeW<HwwakeSpec> {
        ForcewakeW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake for Flexcomm Interfaces. When 1, any Flexcomm Interface FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn fcwake(&mut self) -> FcwakeW<HwwakeSpec> {
        FcwakeW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn dmicwake(&mut self) -> DmicwakeW<HwwakeSpec> {
        DmicwakeW::new(self, 2)
    }
    #[doc = "Bit 3 - Wake for DMAC0. When 1, DMAC0 being busy will cause peripheral clocking to remain running until DMAC0 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC0 has completed its related activity."]
    #[inline(always)]
    #[must_use]
    pub fn dmac0wake(&mut self) -> Dmac0wakeW<HwwakeSpec> {
        Dmac0wakeW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake for DMAC1. When 1, DMAC1 being busy will cause peripheral clocking to remain running until DMAC1 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC1 has completed its related activity."]
    #[inline(always)]
    #[must_use]
    pub fn dmac1wake(&mut self) -> Dmac1wakeW<HwwakeSpec> {
        Dmac1wakeW::new(self, 4)
    }
    #[doc = "Bit 5 - Wake for ESPI. When 1, ESPI being busy will cause peripheral clocking to remain running until ESPI completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before ESPI has completed its related activity."]
    #[inline(always)]
    #[must_use]
    pub fn espiwake(&mut self) -> EspiwakeW<HwwakeSpec> {
        EspiwakeW::new(self, 5)
    }
}
#[doc = "Hardware Wake-up control\n\nYou can [`read`](crate::Reg::read) this register and get [`hwwake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwwake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwwakeSpec;
impl crate::RegisterSpec for HwwakeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwwake::R`](R) reader structure"]
impl crate::Readable for HwwakeSpec {}
#[doc = "`write(|w| ..)` method takes [`hwwake::W`](W) writer structure"]
impl crate::Writable for HwwakeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWWAKE to value 0"]
impl crate::Resettable for HwwakeSpec {
    const RESET_VALUE: u32 = 0;
}
