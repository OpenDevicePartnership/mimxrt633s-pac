#[doc = "Register `AUDIOPLL0CLKSEL` reader"]
pub type R = crate::R<Audiopll0clkselSpec>;
#[doc = "Register `AUDIOPLL0CLKSEL` writer"]
pub type W = crate::W<Audiopll0clkselSpec>;
#[doc = "System PLL Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: SFRO Clock."]
    SfroClk = 0,
    #[doc = "1: XTALIN Clock."]
    XtalClk = 1,
    #[doc = "2: FFRO Clock Divided by 2."]
    FfroDiv2 = 2,
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
#[doc = "Field `SEL` reader - System PLL Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::SfroClk),
            1 => Some(Sel::XtalClk),
            2 => Some(Sel::FfroDiv2),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
    #[doc = "XTALIN Clock."]
    #[inline(always)]
    pub fn is_xtal_clk(&self) -> bool {
        *self == Sel::XtalClk
    }
    #[doc = "FFRO Clock Divided by 2."]
    #[inline(always)]
    pub fn is_ffro_div_2(&self) -> bool {
        *self == Sel::FfroDiv2
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - System PLL Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
    }
    #[doc = "XTALIN Clock."]
    #[inline(always)]
    pub fn xtal_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::XtalClk)
    }
    #[doc = "FFRO Clock Divided by 2."]
    #[inline(always)]
    pub fn ffro_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroDiv2)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - System PLL Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIOPLL0CLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - System PLL Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Audiopll0clkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "audio pll0 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`audiopll0clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiopll0clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Audiopll0clkselSpec;
impl crate::RegisterSpec for Audiopll0clkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audiopll0clksel::R`](R) reader structure"]
impl crate::Readable for Audiopll0clkselSpec {}
#[doc = "`write(|w| ..)` method takes [`audiopll0clksel::W`](W) writer structure"]
impl crate::Writable for Audiopll0clkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0CLKSEL to value 0x07"]
impl crate::Resettable for Audiopll0clkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
