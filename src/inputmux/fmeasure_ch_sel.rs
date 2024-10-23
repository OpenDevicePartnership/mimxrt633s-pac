#[doc = "Register `FMEASURE_CH_SEL%s` reader"]
pub type R = crate::R<FmeasureChSelSpec>;
#[doc = "Register `FMEASURE_CH_SEL%s` writer"]
pub type W = crate::W<FmeasureChSelSpec>;
#[doc = "Frequency Measure Channel n Selection 7:1 Mux Select. . .\n\nValue on reset: 31"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FmeasureSel {
    #[doc = "0: XTALIN"]
    Xtalin = 0,
    #[doc = "1: SFRO"]
    Sfro = 1,
    #[doc = "2: FFRO"]
    Ffro = 2,
    #[doc = "3: Low Power Oscillator Clock (LPOSC)"]
    Lposc = 3,
    #[doc = "4: RTC 32KHz OSC"]
    Rtc32khzOsc = 4,
    #[doc = "5: MAIN_SYS_CLOCK"]
    MainSysClock = 5,
    #[doc = "6: FREQME_GPIO_CLK"]
    FreqmeGpioClk = 6,
}
impl From<FmeasureSel> for u8 {
    #[inline(always)]
    fn from(variant: FmeasureSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FmeasureSel {
    type Ux = u8;
}
impl crate::IsEnum for FmeasureSel {}
#[doc = "Field `FMEASURE_SEL` reader - Frequency Measure Channel n Selection 7:1 Mux Select. . ."]
pub type FmeasureSelR = crate::FieldReader<FmeasureSel>;
impl FmeasureSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FmeasureSel> {
        match self.bits {
            0 => Some(FmeasureSel::Xtalin),
            1 => Some(FmeasureSel::Sfro),
            2 => Some(FmeasureSel::Ffro),
            3 => Some(FmeasureSel::Lposc),
            4 => Some(FmeasureSel::Rtc32khzOsc),
            5 => Some(FmeasureSel::MainSysClock),
            6 => Some(FmeasureSel::FreqmeGpioClk),
            _ => None,
        }
    }
    #[doc = "XTALIN"]
    #[inline(always)]
    pub fn is_xtalin(&self) -> bool {
        *self == FmeasureSel::Xtalin
    }
    #[doc = "SFRO"]
    #[inline(always)]
    pub fn is_sfro(&self) -> bool {
        *self == FmeasureSel::Sfro
    }
    #[doc = "FFRO"]
    #[inline(always)]
    pub fn is_ffro(&self) -> bool {
        *self == FmeasureSel::Ffro
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == FmeasureSel::Lposc
    }
    #[doc = "RTC 32KHz OSC"]
    #[inline(always)]
    pub fn is_rtc_32khz_osc(&self) -> bool {
        *self == FmeasureSel::Rtc32khzOsc
    }
    #[doc = "MAIN_SYS_CLOCK"]
    #[inline(always)]
    pub fn is_main_sys_clock(&self) -> bool {
        *self == FmeasureSel::MainSysClock
    }
    #[doc = "FREQME_GPIO_CLK"]
    #[inline(always)]
    pub fn is_freqme_gpio_clk(&self) -> bool {
        *self == FmeasureSel::FreqmeGpioClk
    }
}
#[doc = "Field `FMEASURE_SEL` writer - Frequency Measure Channel n Selection 7:1 Mux Select. . ."]
pub type FmeasureSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, FmeasureSel>;
impl<'a, REG> FmeasureSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XTALIN"]
    #[inline(always)]
    pub fn xtalin(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::Xtalin)
    }
    #[doc = "SFRO"]
    #[inline(always)]
    pub fn sfro(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::Sfro)
    }
    #[doc = "FFRO"]
    #[inline(always)]
    pub fn ffro(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::Ffro)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)"]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::Lposc)
    }
    #[doc = "RTC 32KHz OSC"]
    #[inline(always)]
    pub fn rtc_32khz_osc(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::Rtc32khzOsc)
    }
    #[doc = "MAIN_SYS_CLOCK"]
    #[inline(always)]
    pub fn main_sys_clock(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::MainSysClock)
    }
    #[doc = "FREQME_GPIO_CLK"]
    #[inline(always)]
    pub fn freqme_gpio_clk(self) -> &'a mut crate::W<REG> {
        self.variant(FmeasureSel::FreqmeGpioClk)
    }
}
impl R {
    #[doc = "Bits 0:4 - Frequency Measure Channel n Selection 7:1 Mux Select. . ."]
    #[inline(always)]
    pub fn fmeasure_sel(&self) -> FmeasureSelR {
        FmeasureSelR::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMEASURE_CH_SEL")
            .field("fmeasure_sel", &self.fmeasure_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Frequency Measure Channel n Selection 7:1 Mux Select. . ."]
    #[inline(always)]
    #[must_use]
    pub fn fmeasure_sel(&mut self) -> FmeasureSelW<FmeasureChSelSpec> {
        FmeasureSelW::new(self, 0)
    }
}
#[doc = "Frequency Measurement Input Channel Multiplexers\n\nYou can [`read`](crate::Reg::read) this register and get [`fmeasure_ch_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmeasure_ch_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmeasureChSelSpec;
impl crate::RegisterSpec for FmeasureChSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmeasure_ch_sel::R`](R) reader structure"]
impl crate::Readable for FmeasureChSelSpec {}
#[doc = "`write(|w| ..)` method takes [`fmeasure_ch_sel::W`](W) writer structure"]
impl crate::Writable for FmeasureChSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMEASURE_CH_SEL%s to value 0x1f"]
impl crate::Resettable for FmeasureChSelSpec {
    const RESET_VALUE: u32 = 0x1f;
}
