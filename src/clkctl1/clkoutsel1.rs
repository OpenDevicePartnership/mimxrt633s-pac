#[doc = "Register `CLKOUTSEL1` reader"]
pub type R = crate::R<Clkoutsel1Spec>;
#[doc = "Register `CLKOUTSEL1` writer"]
pub type W = crate::W<Clkoutsel1Spec>;
#[doc = "Clock out clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: CLKOUTSEL0 Multiplexed Output."]
    Clkoutsel0Output = 0,
    #[doc = "1: Main System PLL Clock."]
    MainPllClk = 1,
    #[doc = "2: SYSPLL0 AUX0_PLL_Clock."]
    Syspll0Aux0PllClk = 2,
    #[doc = "3: DSP PLL clock."]
    DspPllClk = 3,
    #[doc = "4: SYSPLL0 AUX1_PLL_Clock."]
    Syspll0Aux1PllClk = 4,
    #[doc = "5: AUDIO PLL Clock."]
    AudioPllClk = 5,
    #[doc = "6: 32KHz RTC Clock."]
    RtcClk32khz = 6,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    None = 7,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - Clock out clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::Clkoutsel0Output,
            1 => Sel::MainPllClk,
            2 => Sel::Syspll0Aux0PllClk,
            3 => Sel::DspPllClk,
            4 => Sel::Syspll0Aux1PllClk,
            5 => Sel::AudioPllClk,
            6 => Sel::RtcClk32khz,
            7 => Sel::None,
            _ => unreachable!(),
        }
    }
    #[doc = "CLKOUTSEL0 Multiplexed Output."]
    #[inline(always)]
    pub fn is_clkoutsel0_output(&self) -> bool {
        *self == Sel::Clkoutsel0Output
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn is_main_pll_clk(&self) -> bool {
        *self == Sel::MainPllClk
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux0_pll_clk(&self) -> bool {
        *self == Sel::Syspll0Aux0PllClk
    }
    #[doc = "DSP PLL clock."]
    #[inline(always)]
    pub fn is_dsp_pll_clk(&self) -> bool {
        *self == Sel::DspPllClk
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux1_pll_clk(&self) -> bool {
        *self == Sel::Syspll0Aux1PllClk
    }
    #[doc = "AUDIO PLL Clock."]
    #[inline(always)]
    pub fn is_audio_pll_clk(&self) -> bool {
        *self == Sel::AudioPllClk
    }
    #[doc = "32KHz RTC Clock."]
    #[inline(always)]
    pub fn is_rtc_clk_32khz(&self) -> bool {
        *self == Sel::RtcClk32khz
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - Clock out clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKOUTSEL0 Multiplexed Output."]
    #[inline(always)]
    pub fn clkoutsel0_output(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Clkoutsel0Output)
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn main_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainPllClk)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux0_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux0PllClk)
    }
    #[doc = "DSP PLL clock."]
    #[inline(always)]
    pub fn dsp_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::DspPllClk)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux1_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux1PllClk)
    }
    #[doc = "AUDIO PLL Clock."]
    #[inline(always)]
    pub fn audio_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::AudioPllClk)
    }
    #[doc = "32KHz RTC Clock."]
    #[inline(always)]
    pub fn rtc_clk_32khz(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::RtcClk32khz)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock out clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKOUTSEL1")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock out clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Clkoutsel1Spec> {
        SelW::new(self, 0)
    }
}
#[doc = "clock out selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkoutsel1Spec;
impl crate::RegisterSpec for Clkoutsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutsel1::R`](R) reader structure"]
impl crate::Readable for Clkoutsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkoutsel1::W`](W) writer structure"]
impl crate::Writable for Clkoutsel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUTSEL1 to value 0x07"]
impl crate::Resettable for Clkoutsel1Spec {
    const RESET_VALUE: u32 = 0x07;
}
