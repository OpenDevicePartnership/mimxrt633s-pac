#[doc = "Register `ADC0FCLKSEL1` reader"]
pub type R = crate::R<Adc0fclksel1Spec>;
#[doc = "Register `ADC0FCLKSEL1` writer"]
pub type W = crate::W<Adc0fclksel1Spec>;
#[doc = "ADC Functional Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: ADC0FCLKSEL0 Multiplexed Output."]
    Adc0fclksel0MuxOut = 0,
    #[doc = "1: SYSPLL0 MAIN_CLK (PFD0 Output)"]
    Syspll0MainClk = 1,
    #[doc = "3: SYSPLL0 AUX0_PLL_Clock."]
    Syspll0Aux0PllClock = 3,
    #[doc = "5: SYSPLL0 AUX1_PLL_Clock."]
    Syspll0Aux1PllClock = 5,
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
#[doc = "Field `SEL` reader - ADC Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Adc0fclksel0MuxOut),
            1 => Some(Sel::Syspll0MainClk),
            3 => Some(Sel::Syspll0Aux0PllClock),
            5 => Some(Sel::Syspll0Aux1PllClock),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "ADC0FCLKSEL0 Multiplexed Output."]
    #[inline(always)]
    pub fn is_adc0fclksel0_mux_out(&self) -> bool {
        *self == Sel::Adc0fclksel0MuxOut
    }
    #[doc = "SYSPLL0 MAIN_CLK (PFD0 Output)"]
    #[inline(always)]
    pub fn is_syspll0_main_clk(&self) -> bool {
        *self == Sel::Syspll0MainClk
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux0_pll_clock(&self) -> bool {
        *self == Sel::Syspll0Aux0PllClock
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux1_pll_clock(&self) -> bool {
        *self == Sel::Syspll0Aux1PllClock
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - ADC Functional Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC0FCLKSEL0 Multiplexed Output."]
    #[inline(always)]
    pub fn adc0fclksel0_mux_out(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Adc0fclksel0MuxOut)
    }
    #[doc = "SYSPLL0 MAIN_CLK (PFD0 Output)"]
    #[inline(always)]
    pub fn syspll0_main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0MainClk)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux0_pll_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux0PllClock)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux1_pll_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux1PllClock)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC0FCLKSEL1")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Adc0fclksel1Spec> {
        SelW::new(self, 0)
    }
}
#[doc = "ADC0 fclk selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0fclksel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0fclksel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0fclksel1Spec;
impl crate::RegisterSpec for Adc0fclksel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0fclksel1::R`](R) reader structure"]
impl crate::Readable for Adc0fclksel1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc0fclksel1::W`](W) writer structure"]
impl crate::Writable for Adc0fclksel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC0FCLKSEL1 to value 0x07"]
impl crate::Resettable for Adc0fclksel1Spec {
    const RESET_VALUE: u32 = 0x07;
}
