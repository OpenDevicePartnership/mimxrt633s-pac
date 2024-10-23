#[doc = "Register `ESPICLKSEL` reader"]
pub type R = crate::R<EspiclkselSpec>;
#[doc = "Register `ESPICLKSEL` writer"]
pub type W = crate::W<EspiclkselSpec>;
#[doc = "no description available\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Use 48/60m_irc clock"]
    Use48_60m = 0,
    #[doc = "3: clock gated"]
    ClkGated = 3,
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
#[doc = "Field `SEL` reader - no description available"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Use48_60m),
            3 => Some(Sel::ClkGated),
            _ => None,
        }
    }
    #[doc = "Use 48/60m_irc clock"]
    #[inline(always)]
    pub fn is_use_48_60m(&self) -> bool {
        *self == Sel::Use48_60m
    }
    #[doc = "clock gated"]
    #[inline(always)]
    pub fn is_clk_gated(&self) -> bool {
        *self == Sel::ClkGated
    }
}
#[doc = "Field `SEL` writer - no description available"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use 48/60m_irc clock"]
    #[inline(always)]
    pub fn use_48_60m(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Use48_60m)
    }
    #[doc = "clock gated"]
    #[inline(always)]
    pub fn clk_gated(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::ClkGated)
    }
}
impl R {
    #[doc = "Bits 0:2 - no description available"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESPICLKSEL")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<EspiclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "ESPI clock selection\n\nYou can [`read`](crate::Reg::read) this register and get [`espiclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EspiclkselSpec;
impl crate::RegisterSpec for EspiclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`espiclksel::R`](R) reader structure"]
impl crate::Readable for EspiclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`espiclksel::W`](W) writer structure"]
impl crate::Writable for EspiclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESPICLKSEL to value 0x03"]
impl crate::Resettable for EspiclkselSpec {
    const RESET_VALUE: u32 = 0x03;
}
