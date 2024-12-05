#[doc = "Register `CT32BITFCLKSEL%s` reader"]
pub type R = crate::R<Ct32bitfclkselSpec>;
#[doc = "Register `CT32BITFCLKSEL%s` writer"]
pub type W = crate::W<Ct32bitfclkselSpec>;
#[doc = "CT32Bit Functional Clock Source Selection. . .\n\nValue on reset: 7"]
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
    #[doc = "3: Audio PLL Clock."]
    AudioPllClk = 3,
    #[doc = "4: Master Clock In."]
    MasterClk = 4,
    #[doc = "5: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 5,
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
#[doc = "Field `SEL` reader - CT32Bit Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::MainClk),
            1 => Some(Sel::SfroClk),
            2 => Some(Sel::FfroClk),
            3 => Some(Sel::AudioPllClk),
            4 => Some(Sel::MasterClk),
            5 => Some(Sel::Lposc),
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
    #[doc = "Audio PLL Clock."]
    #[inline(always)]
    pub fn is_audio_pll_clk(&self) -> bool {
        *self == Sel::AudioPllClk
    }
    #[doc = "Master Clock In."]
    #[inline(always)]
    pub fn is_master_clk(&self) -> bool {
        *self == Sel::MasterClk
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - CT32Bit Functional Clock Source Selection. . ."]
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
    #[doc = "Audio PLL Clock."]
    #[inline(always)]
    pub fn audio_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::AudioPllClk)
    }
    #[doc = "Master Clock In."]
    #[inline(always)]
    pub fn master_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MasterClk)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - CT32Bit Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CT32BITFCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - CT32Bit Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Ct32bitfclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "ct32bit timer N clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`ct32bitfclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ct32bitfclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ct32bitfclkselSpec;
impl crate::RegisterSpec for Ct32bitfclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct32bitfclksel::R`](R) reader structure"]
impl crate::Readable for Ct32bitfclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`ct32bitfclksel::W`](W) writer structure"]
impl crate::Writable for Ct32bitfclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT32BITFCLKSEL%s to value 0x07"]
impl crate::Resettable for Ct32bitfclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
