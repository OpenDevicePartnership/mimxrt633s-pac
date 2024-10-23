#[doc = "Register `FRG14CLKSEL` reader"]
pub type R = crate::R<Frg14clkselSpec>;
#[doc = "Register `FRG14CLKSEL` writer"]
pub type W = crate::W<Frg14clkselSpec>;
#[doc = "Fractional Gen. Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main Clock."]
    MainClk = 0,
    #[doc = "1: Main System PLL Clock."]
    FrgPllClk = 1,
    #[doc = "2: SFRO Clock."]
    SfroClk = 2,
    #[doc = "3: FFRO Clock."]
    FfroClk = 3,
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
#[doc = "Field `SEL` reader - Fractional Gen. Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::MainClk),
            1 => Some(Sel::FrgPllClk),
            2 => Some(Sel::SfroClk),
            3 => Some(Sel::FfroClk),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Sel::MainClk
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn is_frg_pll_clk(&self) -> bool {
        *self == Sel::FrgPllClk
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
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - Fractional Gen. Clock Source Selection. . ."]
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
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn frg_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FrgPllClk)
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
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - Fractional Gen. Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRG14CLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Fractional Gen. Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Frg14clkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "FRG clock selection register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`frg14clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frg14clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frg14clkselSpec;
impl crate::RegisterSpec for Frg14clkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frg14clksel::R`](R) reader structure"]
impl crate::Readable for Frg14clkselSpec {}
#[doc = "`write(|w| ..)` method takes [`frg14clksel::W`](W) writer structure"]
impl crate::Writable for Frg14clkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRG14CLKSEL to value 0x07"]
impl crate::Resettable for Frg14clkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
