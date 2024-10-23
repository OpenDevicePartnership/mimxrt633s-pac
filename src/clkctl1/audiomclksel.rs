#[doc = "Register `AUDIOMCLKSEL` reader"]
pub type R = crate::R<AudiomclkselSpec>;
#[doc = "Register `AUDIOMCLKSEL` writer"]
pub type W = crate::W<AudiomclkselSpec>;
#[doc = "Audio MCLK Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: FFRO Clock."]
    FfroClk = 0,
    #[doc = "1: AUDIO PLL Clock. (Shared Domain)"]
    AudioPllClk = 1,
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
#[doc = "Field `SEL` reader - Audio MCLK Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::FfroClk),
            1 => Some(Sel::AudioPllClk),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
    #[doc = "AUDIO PLL Clock. (Shared Domain)"]
    #[inline(always)]
    pub fn is_audio_pll_clk(&self) -> bool {
        *self == Sel::AudioPllClk
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - Audio MCLK Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
    #[doc = "AUDIO PLL Clock. (Shared Domain)"]
    #[inline(always)]
    pub fn audio_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::AudioPllClk)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - Audio MCLK Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUDIOMCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Audio MCLK Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<AudiomclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "audio mclock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`audiomclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audiomclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudiomclkselSpec;
impl crate::RegisterSpec for AudiomclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audiomclksel::R`](R) reader structure"]
impl crate::Readable for AudiomclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`audiomclksel::W`](W) writer structure"]
impl crate::Writable for AudiomclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIOMCLKSEL to value 0x07"]
impl crate::Resettable for AudiomclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
