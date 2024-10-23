#[doc = "Register `SYSTICKFCLKSEL` reader"]
pub type R = crate::R<SystickfclkselSpec>;
#[doc = "Register `SYSTICKFCLKSEL` writer"]
pub type W = crate::W<SystickfclkselSpec>;
#[doc = "SYSTICK Functional Clock Source Selection. . .\n\nValue on reset: 7"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Systick Divider Output Clock."]
    SystickDivClk = 0,
    #[doc = "1: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 1,
    #[doc = "2: 32KHz RTC Clock."]
    Rtc32khz = 2,
    #[doc = "3: SFRO Clock."]
    SfroClk = 3,
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
#[doc = "Field `SEL` reader - SYSTICK Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::SystickDivClk),
            1 => Some(Sel::Lposc),
            2 => Some(Sel::Rtc32khz),
            3 => Some(Sel::SfroClk),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "Systick Divider Output Clock."]
    #[inline(always)]
    pub fn is_systick_div_clk(&self) -> bool {
        *self == Sel::SystickDivClk
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "32KHz RTC Clock."]
    #[inline(always)]
    pub fn is_rtc_32khz(&self) -> bool {
        *self == Sel::Rtc32khz
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - SYSTICK Functional Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Systick Divider Output Clock."]
    #[inline(always)]
    pub fn systick_div_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SystickDivClk)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "32KHz RTC Clock."]
    #[inline(always)]
    pub fn rtc_32khz(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Rtc32khz)
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - SYSTICK Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTICKFCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SYSTICK Functional Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<SystickfclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "system tick fclk selection\n\nYou can [`read`](crate::Reg::read) this register and get [`systickfclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickfclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystickfclkselSpec;
impl crate::RegisterSpec for SystickfclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systickfclksel::R`](R) reader structure"]
impl crate::Readable for SystickfclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`systickfclksel::W`](W) writer structure"]
impl crate::Writable for SystickfclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTICKFCLKSEL to value 0x07"]
impl crate::Resettable for SystickfclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
