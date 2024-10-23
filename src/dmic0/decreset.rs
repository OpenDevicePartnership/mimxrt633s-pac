#[doc = "Register `DECRESET` reader"]
pub type R = crate::R<DecresetSpec>;
#[doc = "Register `DECRESET` writer"]
pub type W = crate::W<DecresetSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Decreset {
    #[doc = "0: release reset to decimator"]
    ReleaseReset = 0,
    #[doc = "1: assert reset to decimator Note : resets are applied in pairs. So bit 0 corresponds to channels 0/1, bit1 corresponds to channels 2/3, bit2 to channel 4/5 and bit3 to channel 6/7"]
    AssertReset = 1,
}
impl From<Decreset> for u8 {
    #[inline(always)]
    fn from(variant: Decreset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Decreset {
    type Ux = u8;
}
impl crate::IsEnum for Decreset {}
#[doc = "Field `DECRESET` reader - no description available"]
pub type DecresetR = crate::FieldReader<Decreset>;
impl DecresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Decreset> {
        match self.bits {
            0 => Some(Decreset::ReleaseReset),
            1 => Some(Decreset::AssertReset),
            _ => None,
        }
    }
    #[doc = "release reset to decimator"]
    #[inline(always)]
    pub fn is_release_reset(&self) -> bool {
        *self == Decreset::ReleaseReset
    }
    #[doc = "assert reset to decimator Note : resets are applied in pairs. So bit 0 corresponds to channels 0/1, bit1 corresponds to channels 2/3, bit2 to channel 4/5 and bit3 to channel 6/7"]
    #[inline(always)]
    pub fn is_assert_reset(&self) -> bool {
        *self == Decreset::AssertReset
    }
}
#[doc = "Field `DECRESET` writer - no description available"]
pub type DecresetW<'a, REG> = crate::FieldWriter<'a, REG, 8, Decreset>;
impl<'a, REG> DecresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "release reset to decimator"]
    #[inline(always)]
    pub fn release_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Decreset::ReleaseReset)
    }
    #[doc = "assert reset to decimator Note : resets are applied in pairs. So bit 0 corresponds to channels 0/1, bit1 corresponds to channels 2/3, bit2 to channel 4/5 and bit3 to channel 6/7"]
    #[inline(always)]
    pub fn assert_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Decreset::AssertReset)
    }
}
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn decreset(&self) -> DecresetR {
        DecresetR::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECRESET")
            .field("decreset", &self.decreset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn decreset(&mut self) -> DecresetW<DecresetSpec> {
        DecresetW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`decreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecresetSpec;
impl crate::RegisterSpec for DecresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decreset::R`](R) reader structure"]
impl crate::Readable for DecresetSpec {}
#[doc = "`write(|w| ..)` method takes [`decreset::W`](W) writer structure"]
impl crate::Writable for DecresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DECRESET to value 0"]
impl crate::Resettable for DecresetSpec {
    const RESET_VALUE: u32 = 0;
}
