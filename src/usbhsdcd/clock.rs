#[doc = "Register `CLOCK` reader"]
pub type R = crate::R<ClockSpec>;
#[doc = "Register `CLOCK` writer"]
pub type W = crate::W<ClockSpec>;
#[doc = "Unit of Measurement Encoding for Clock Speed\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockUnit {
    #[doc = "0: kHz Speed (between 1 kHz and 1023 kHz)"]
    KhzClk = 0,
    #[doc = "1: MHz Speed (between 1 MHz and 1023 MHz)"]
    MhzClk = 1,
}
impl From<ClockUnit> for bool {
    #[inline(always)]
    fn from(variant: ClockUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLOCK_UNIT` reader - Unit of Measurement Encoding for Clock Speed"]
pub type ClockUnitR = crate::BitReader<ClockUnit>;
impl ClockUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClockUnit {
        match self.bits {
            false => ClockUnit::KhzClk,
            true => ClockUnit::MhzClk,
        }
    }
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    #[inline(always)]
    pub fn is_khz_clk(&self) -> bool {
        *self == ClockUnit::KhzClk
    }
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    #[inline(always)]
    pub fn is_mhz_clk(&self) -> bool {
        *self == ClockUnit::MhzClk
    }
}
#[doc = "Field `CLOCK_UNIT` writer - Unit of Measurement Encoding for Clock Speed"]
pub type ClockUnitW<'a, REG> = crate::BitWriter<'a, REG, ClockUnit>;
impl<'a, REG> ClockUnitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    #[inline(always)]
    pub fn khz_clk(self) -> &'a mut crate::W<REG> {
        self.variant(ClockUnit::KhzClk)
    }
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    #[inline(always)]
    pub fn mhz_clk(self) -> &'a mut crate::W<REG> {
        self.variant(ClockUnit::MhzClk)
    }
}
#[doc = "Field `CLOCK_SPEED` reader - Numerical Value of Clock Speed in Binary"]
pub type ClockSpeedR = crate::FieldReader<u16>;
#[doc = "Field `CLOCK_SPEED` writer - Numerical Value of Clock Speed in Binary"]
pub type ClockSpeedW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub fn clock_unit(&self) -> ClockUnitR {
        ClockUnitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    pub fn clock_speed(&self) -> ClockSpeedR {
        ClockSpeedR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK")
            .field("clock_unit", &self.clock_unit())
            .field("clock_speed", &self.clock_speed())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    #[must_use]
    pub fn clock_unit(&mut self) -> ClockUnitW<ClockSpec> {
        ClockUnitW::new(self, 0)
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    #[must_use]
    pub fn clock_speed(&mut self) -> ClockSpeedW<ClockSpec> {
        ClockSpeedW::new(self, 2)
    }
}
#[doc = "Clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockSpec;
impl crate::RegisterSpec for ClockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock::R`](R) reader structure"]
impl crate::Readable for ClockSpec {}
#[doc = "`write(|w| ..)` method takes [`clock::W`](W) writer structure"]
impl crate::Writable for ClockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCK to value 0xc1"]
impl crate::Resettable for ClockSpec {
    const RESET_VALUE: u32 = 0xc1;
}
