#[doc = "Register `WDT0FCLKSEL` reader"]
pub type R = crate::R<Wdt0fclkselSpec>;
#[doc = "Register `WDT0FCLKSEL` writer"]
pub type W = crate::W<Wdt0fclkselSpec>;
#[doc = "WDT0 Functional Clock Source Selection. .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 0,
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
#[doc = "Field `SEL` reader - WDT0 Functional Clock Source Selection. ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Lposc),
            7 => Some(Sel::None),
            _ => None,
        }
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
#[doc = "Field `SEL` writer - WDT0 Functional Clock Source Selection. ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
    #[doc = "Bits 0:2 - WDT0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT0FCLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Wdt0fclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "wdt clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt0fclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt0fclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt0fclkselSpec;
impl crate::RegisterSpec for Wdt0fclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt0fclksel::R`](R) reader structure"]
impl crate::Readable for Wdt0fclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt0fclksel::W`](W) writer structure"]
impl crate::Writable for Wdt0fclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT0FCLKSEL to value 0"]
impl crate::Resettable for Wdt0fclkselSpec {
    const RESET_VALUE: u32 = 0;
}
