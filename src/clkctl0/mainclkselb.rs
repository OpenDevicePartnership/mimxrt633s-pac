#[doc = "Register `MAINCLKSELB` reader"]
pub type R = crate::R<MainclkselbSpec>;
#[doc = "Register `MAINCLKSELB` writer"]
pub type W = crate::W<MainclkselbSpec>;
#[doc = "Main Clock Source Selection. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: MAINCLKSELA 1st Stage Clock."]
    Main1stClk = 0,
    #[doc = "1: SFRO Clock."]
    SfroClk = 1,
    #[doc = "2: Main System PLL Clock."]
    MainPllClk = 2,
    #[doc = "3: RTC 32KHz Clock."]
    Rtc32kClk = 3,
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
#[doc = "Field `SEL` reader - Main Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::Main1stClk,
            1 => Sel::SfroClk,
            2 => Sel::MainPllClk,
            3 => Sel::Rtc32kClk,
            _ => unreachable!(),
        }
    }
    #[doc = "MAINCLKSELA 1st Stage Clock."]
    #[inline(always)]
    pub fn is_main_1st_clk(&self) -> bool {
        *self == Sel::Main1stClk
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn is_main_pll_clk(&self) -> bool {
        *self == Sel::MainPllClk
    }
    #[doc = "RTC 32KHz Clock."]
    #[inline(always)]
    pub fn is_rtc_32k_clk(&self) -> bool {
        *self == Sel::Rtc32kClk
    }
}
#[doc = "Field `SEL` writer - Main Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MAINCLKSELA 1st Stage Clock."]
    #[inline(always)]
    pub fn main_1st_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Main1stClk)
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
    }
    #[doc = "Main System PLL Clock."]
    #[inline(always)]
    pub fn main_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MainPllClk)
    }
    #[doc = "RTC 32KHz Clock."]
    #[inline(always)]
    pub fn rtc_32k_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Rtc32kClk)
    }
}
impl R {
    #[doc = "Bits 0:1 - Main Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAINCLKSELB")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Main Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<MainclkselbSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "main clock selection B\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclkselb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclkselb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainclkselbSpec;
impl crate::RegisterSpec for MainclkselbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mainclkselb::R`](R) reader structure"]
impl crate::Readable for MainclkselbSpec {}
#[doc = "`write(|w| ..)` method takes [`mainclkselb::W`](W) writer structure"]
impl crate::Writable for MainclkselbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAINCLKSELB to value 0"]
impl crate::Resettable for MainclkselbSpec {
    const RESET_VALUE: u32 = 0;
}
