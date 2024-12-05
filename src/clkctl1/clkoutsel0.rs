#[doc = "Register `CLKOUTSEL0` reader"]
pub type R = crate::R<Clkoutsel0Spec>;
#[doc = "Register `CLKOUTSEL0` writer"]
pub type W = crate::W<Clkoutsel0Spec>;
#[doc = "Clock Output Select 1st Stage. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: SFRO Clock."]
    SfroClk = 0,
    #[doc = "1: XTALIN Clock."]
    XtalinClk = 1,
    #[doc = "2: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 2,
    #[doc = "3: FFRO Clock."]
    FfroClk = 3,
    #[doc = "4: Main Clock."]
    MainClk = 4,
    #[doc = "6: DSP Main Clock."]
    DspMainClk = 6,
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
#[doc = "Field `SEL` reader - Clock Output Select 1st Stage. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::SfroClk),
            1 => Some(Sel::XtalinClk),
            2 => Some(Sel::Lposc),
            3 => Some(Sel::FfroClk),
            4 => Some(Sel::MainClk),
            6 => Some(Sel::DspMainClk),
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
    pub fn is_xtalin_clk(&self) -> bool {
        *self == Sel::XtalinClk
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == Sel::MainClk
    }
    #[doc = "DSP Main Clock."]
    #[inline(always)]
    pub fn is_dsp_main_clk(&self) -> bool {
        *self == Sel::DspMainClk
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - Clock Output Select 1st Stage. . ."]
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
    pub fn xtalin_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::XtalinClk)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
    #[doc = "Main Clock."]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainClk)
    }
    #[doc = "DSP Main Clock."]
    #[inline(always)]
    pub fn dsp_main_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::DspMainClk)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Output Select 1st Stage. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKOUTSEL0")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Output Select 1st Stage. . ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Clkoutsel0Spec> {
        SelW::new(self, 0)
    }
}
#[doc = "clock out selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkoutsel0Spec;
impl crate::RegisterSpec for Clkoutsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutsel0::R`](R) reader structure"]
impl crate::Readable for Clkoutsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`clkoutsel0::W`](W) writer structure"]
impl crate::Writable for Clkoutsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOUTSEL0 to value 0x07"]
impl crate::Resettable for Clkoutsel0Spec {
    const RESET_VALUE: u32 = 0x07;
}
