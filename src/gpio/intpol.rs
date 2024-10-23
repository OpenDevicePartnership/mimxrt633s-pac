#[doc = "Register `INTPOL[%s]` reader"]
pub type R = crate::R<IntpolSpec>;
#[doc = "Register `INTPOL[%s]` writer"]
pub type W = crate::W<IntpolSpec>;
#[doc = "polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PolCtl {
    #[doc = "0: interrupt when gpio high"]
    Hihg = 0,
    #[doc = "1: interrupt when gpio low"]
    Low = 1,
}
impl From<PolCtl> for u32 {
    #[inline(always)]
    fn from(variant: PolCtl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PolCtl {
    type Ux = u32;
}
impl crate::IsEnum for PolCtl {}
#[doc = "Field `POL_CTL` reader - polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)"]
pub type PolCtlR = crate::FieldReader<PolCtl>;
impl PolCtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PolCtl> {
        match self.bits {
            0 => Some(PolCtl::Hihg),
            1 => Some(PolCtl::Low),
            _ => None,
        }
    }
    #[doc = "interrupt when gpio high"]
    #[inline(always)]
    pub fn is_hihg(&self) -> bool {
        *self == PolCtl::Hihg
    }
    #[doc = "interrupt when gpio low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PolCtl::Low
    }
}
#[doc = "Field `POL_CTL` writer - polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)"]
pub type PolCtlW<'a, REG> = crate::FieldWriter<'a, REG, 32, PolCtl>;
impl<'a, REG> PolCtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "interrupt when gpio high"]
    #[inline(always)]
    pub fn hihg(self) -> &'a mut crate::W<REG> {
        self.variant(PolCtl::Hihg)
    }
    #[doc = "interrupt when gpio low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PolCtl::Low)
    }
}
impl R {
    #[doc = "Bits 0:31 - polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)"]
    #[inline(always)]
    pub fn pol_ctl(&self) -> PolCtlR {
        PolCtlR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPOL")
            .field("pol_ctl", &self.pol_ctl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)"]
    #[inline(always)]
    #[must_use]
    pub fn pol_ctl(&mut self) -> PolCtlW<IntpolSpec> {
        PolCtlW::new(self, 0)
    }
}
#[doc = "interupt polarity control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intpol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpolSpec;
impl crate::RegisterSpec for IntpolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpol::R`](R) reader structure"]
impl crate::Readable for IntpolSpec {}
#[doc = "`write(|w| ..)` method takes [`intpol::W`](W) writer structure"]
impl crate::Writable for IntpolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPOL[%s]
to value 0"]
impl crate::Resettable for IntpolSpec {
    const RESET_VALUE: u32 = 0;
}
