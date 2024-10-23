#[doc = "Register `FC15FCLKSEL` reader"]
pub type R = crate::R<Fc15fclkselSpec>;
#[doc = "Register `FC15FCLKSEL` writer"]
pub type W = crate::W<Fc15fclkselSpec>;
#[doc = "Flexxcomm Functional Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: SFRO Clock."]
    SfroClk = 0,
    #[doc = "1: FFRO Clock."]
    FfroClk = 1,
    #[doc = "2: Audio PLL Clock."]
    AudioPllClk = 2,
    #[doc = "3: Master Clock In."]
    MasterClk = 3,
    #[doc = "4: FCn FRG Clock."]
    FcnFrgClk = 4,
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
#[doc = "Field `SEL` reader - Flexxcomm Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::SfroClk),
            1 => Some(Sel::FfroClk),
            2 => Some(Sel::AudioPllClk),
            3 => Some(Sel::MasterClk),
            4 => Some(Sel::FcnFrgClk),
            7 => Some(Sel::None),
            _ => None,
        }
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
    #[doc = "FCn FRG Clock."]
    #[inline(always)]
    pub fn is_fcn_frg_clk(&self) -> bool {
        *self == Sel::FcnFrgClk
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - Flexxcomm Functional Clock Source Selection. . ."]
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
    #[doc = "FCn FRG Clock."]
    #[inline(always)]
    pub fn fcn_frg_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FcnFrgClk)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - Flexxcomm Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC15FCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Flexxcomm Functional Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Fc15fclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "flexcomm15 clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`fc15fclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc15fclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fc15fclkselSpec;
impl crate::RegisterSpec for Fc15fclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc15fclksel::R`](R) reader structure"]
impl crate::Readable for Fc15fclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`fc15fclksel::W`](W) writer structure"]
impl crate::Writable for Fc15fclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC15FCLKSEL to value 0x07"]
impl crate::Resettable for Fc15fclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
