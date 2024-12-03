#[doc = "Register `ACMP0FCLKSEL` reader"]
pub type R = crate::R<Acmp0fclkselSpec>;
#[doc = "Register `ACMP0FCLKSEL` writer"]
pub type W = crate::W<Acmp0fclkselSpec>;
#[doc = "ACMP0 Fast Functional Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main Clock."]
    MainClk = 0,
    #[doc = "1: SFRO Clock."]
    SfroClk = 1,
    #[doc = "2: FFRO Clock."]
    FfroClk = 2,
    #[doc = "3: SYSPLL0 AUX0_PLL_Clock."]
    Syspll0Aux0PllClk = 3,
    #[doc = "4: SYSPLL0 AUX1_PLL_Clock."]
    Syspll0Aux1PllClk = 4,
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
#[doc = "Field `SEL` reader - ACMP0 Fast Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::MainClk),
            1 => Some(Sel::SfroClk),
            2 => Some(Sel::FfroClk),
            3 => Some(Sel::Syspll0Aux0PllClk),
            4 => Some(Sel::Syspll0Aux1PllClk),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Sel::MainClk
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux0_pll_clk(&self) -> bool {
        *self == Sel::Syspll0Aux0PllClk
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn is_syspll0_aux1_pll_clk(&self) -> bool {
        *self == Sel::Syspll0Aux1PllClk
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - ACMP0 Fast Functional Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainClk)
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux0_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux0PllClk)
    }
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    #[inline(always)]
    pub fn syspll0_aux1_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Syspll0Aux1PllClk)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - ACMP0 Fast Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACMP0FCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - ACMP0 Fast Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Acmp0fclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "acomparator 0 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0fclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0fclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acmp0fclkselSpec;
impl crate::RegisterSpec for Acmp0fclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmp0fclksel::R`](R) reader structure"]
impl crate::Readable for Acmp0fclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`acmp0fclksel::W`](W) writer structure"]
impl crate::Writable for Acmp0fclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACMP0FCLKSEL to value 0x07"]
impl crate::Resettable for Acmp0fclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
