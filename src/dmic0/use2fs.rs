#[doc = "Register `USE2FS` reader"]
pub type R = crate::R<Use2fsSpec>;
#[doc = "Register `USE2FS` writer"]
pub type W = crate::W<Use2fsSpec>;
#[doc = "Use 2FS register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Use2fs {
    #[doc = "0: Use 1FS output for PCM data."]
    Use1fs = 0,
    #[doc = "1: Use 2FS output for PCM data."]
    Use2fs = 1,
}
impl From<Use2fs> for bool {
    #[inline(always)]
    fn from(variant: Use2fs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USE2FS` reader - Use 2FS register"]
pub type Use2fsR = crate::BitReader<Use2fs>;
impl Use2fsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Use2fs {
        match self.bits {
            false => Use2fs::Use1fs,
            true => Use2fs::Use2fs,
        }
    }
    #[doc = "Use 1FS output for PCM data."]
    #[inline(always)]
    pub fn is_use_1fs(&self) -> bool {
        *self == Use2fs::Use1fs
    }
    #[doc = "Use 2FS output for PCM data."]
    #[inline(always)]
    pub fn is_use_2fs(&self) -> bool {
        *self == Use2fs::Use2fs
    }
}
#[doc = "Field `USE2FS` writer - Use 2FS register"]
pub type Use2fsW<'a, REG> = crate::BitWriter<'a, REG, Use2fs>;
impl<'a, REG> Use2fsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 1FS output for PCM data."]
    #[inline(always)]
    pub fn use_1fs(self) -> &'a mut crate::W<REG> {
        self.variant(Use2fs::Use1fs)
    }
    #[doc = "Use 2FS output for PCM data."]
    #[inline(always)]
    pub fn use_2fs(self) -> &'a mut crate::W<REG> {
        self.variant(Use2fs::Use2fs)
    }
}
impl R {
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&self) -> Use2fsR {
        Use2fsR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USE2FS")
            .field("use2fs", &self.use2fs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&mut self) -> Use2fsW<Use2fsSpec> {
        Use2fsW::new(self, 0)
    }
}
#[doc = "Use 2FS register\n\nYou can [`read`](crate::Reg::read) this register and get [`use2fs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`use2fs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Use2fsSpec;
impl crate::RegisterSpec for Use2fsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`use2fs::R`](R) reader structure"]
impl crate::Readable for Use2fsSpec {}
#[doc = "`write(|w| ..)` method takes [`use2fs::W`](W) writer structure"]
impl crate::Writable for Use2fsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USE2FS to value 0"]
impl crate::Resettable for Use2fsSpec {
    const RESET_VALUE: u32 = 0;
}
