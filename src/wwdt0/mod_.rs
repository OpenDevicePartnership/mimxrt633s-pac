#[doc = "Register `MOD` reader"]
pub type R = crate::R<ModSpec>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<ModSpec>;
#[doc = "Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wden {
    #[doc = "0: Stop. The watchdog timer is stopped."]
    Stop = 0,
    #[doc = "1: Run. The watchdog timer is running."]
    Run = 1,
}
impl From<Wden> for bool {
    #[inline(always)]
    fn from(variant: Wden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDEN` reader - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
pub type WdenR = crate::BitReader<Wden>;
impl WdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wden {
        match self.bits {
            false => Wden::Stop,
            true => Wden::Run,
        }
    }
    #[doc = "Stop. The watchdog timer is stopped."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Wden::Stop
    }
    #[doc = "Run. The watchdog timer is running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Wden::Run
    }
}
#[doc = "Field `WDEN` writer - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
pub type WdenW<'a, REG> = crate::BitWriter<'a, REG, Wden>;
impl<'a, REG> WdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop. The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Wden::Stop)
    }
    #[doc = "Run. The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Wden::Run)
    }
}
#[doc = "Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdreset {
    #[doc = "0: Interrupt. A watchdog time-out will not cause a chip reset."]
    Interrupt = 0,
    #[doc = "1: Reset. A watchdog time-out will cause a chip reset."]
    Reset = 1,
}
impl From<Wdreset> for bool {
    #[inline(always)]
    fn from(variant: Wdreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDRESET` reader - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
pub type WdresetR = crate::BitReader<Wdreset>;
impl WdresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdreset {
        match self.bits {
            false => Wdreset::Interrupt,
            true => Wdreset::Reset,
        }
    }
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Wdreset::Interrupt
    }
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Wdreset::Reset
    }
}
#[doc = "Field `WDRESET` writer - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
pub type WdresetW<'a, REG> = crate::BitWriter<'a, REG, Wdreset>;
impl<'a, REG> WdresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Wdreset::Interrupt)
    }
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Wdreset::Reset)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
pub type WdtofR = crate::BitReader;
#[doc = "Field `WDTOF` writer - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
pub type WdtofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDINT` reader - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
pub type WdintR = crate::BitReader;
#[doc = "Field `WDINT` writer - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
pub type WdintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog update mode. This bit can be set once by software and is only cleared by a reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdprotect {
    #[doc = "0: Flexible. The watchdog time-out value (TC) can be changed at any time."]
    Flexible = 0,
    #[doc = "1: Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    Threshold = 1,
}
impl From<Wdprotect> for bool {
    #[inline(always)]
    fn from(variant: Wdprotect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDPROTECT` reader - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
pub type WdprotectR = crate::BitReader<Wdprotect>;
impl WdprotectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdprotect {
        match self.bits {
            false => Wdprotect::Flexible,
            true => Wdprotect::Threshold,
        }
    }
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        *self == Wdprotect::Flexible
    }
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    #[inline(always)]
    pub fn is_threshold(&self) -> bool {
        *self == Wdprotect::Threshold
    }
}
#[doc = "Field `WDPROTECT` writer - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
pub type WdprotectW<'a, REG> = crate::BitWriter<'a, REG, Wdprotect>;
impl<'a, REG> WdprotectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    #[inline(always)]
    pub fn flexible(self) -> &'a mut crate::W<REG> {
        self.variant(Wdprotect::Flexible)
    }
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    #[inline(always)]
    pub fn threshold(self) -> &'a mut crate::W<REG> {
        self.variant(Wdprotect::Threshold)
    }
}
#[doc = "Field `LOCK` reader - Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub fn wden(&self) -> WdenR {
        WdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub fn wdreset(&self) -> WdresetR {
        WdresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&self) -> WdtofR {
        WdtofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub fn wdint(&self) -> WdintR {
        WdintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WdprotectR {
        WdprotectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOD")
            .field("wden", &self.wden())
            .field("wdreset", &self.wdreset())
            .field("wdtof", &self.wdtof())
            .field("wdint", &self.wdint())
            .field("wdprotect", &self.wdprotect())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub fn wden(&mut self) -> WdenW<ModSpec> {
        WdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub fn wdreset(&mut self) -> WdresetW<ModSpec> {
        WdresetW::new(self, 1)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&mut self) -> WdtofW<ModSpec> {
        WdtofW::new(self, 2)
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub fn wdint(&mut self) -> WdintW<ModSpec> {
        WdintW::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&mut self) -> WdprotectW<ModSpec> {
        WdprotectW::new(self, 4)
    }
    #[doc = "Bit 5 - Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<ModSpec> {
        LockW::new(self, 5)
    }
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModSpec;
impl crate::RegisterSpec for ModSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_::R`](R) reader structure"]
impl crate::Readable for ModSpec {}
#[doc = "`write(|w| ..)` method takes [`mod_::W`](W) writer structure"]
impl crate::Writable for ModSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for ModSpec {
    const RESET_VALUE: u32 = 0;
}
