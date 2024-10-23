#[doc = "Register `SEC_CFG` reader"]
pub type R = crate::R<SecCfgSpec>;
#[doc = "Register `SEC_CFG` writer"]
pub type W = crate::W<SecCfgSpec>;
#[doc = "If set, the TRNG registers cannot be programmed\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoPrgm {
    #[doc = "0: Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    NoPrgm0 = 0,
    #[doc = "1: Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    NoPrgm1 = 1,
}
impl From<NoPrgm> for bool {
    #[inline(always)]
    fn from(variant: NoPrgm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NO_PRGM` reader - If set, the TRNG registers cannot be programmed"]
pub type NoPrgmR = crate::BitReader<NoPrgm>;
impl NoPrgmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NoPrgm {
        match self.bits {
            false => NoPrgm::NoPrgm0,
            true => NoPrgm::NoPrgm1,
        }
    }
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    #[inline(always)]
    pub fn is_no_prgm_0(&self) -> bool {
        *self == NoPrgm::NoPrgm0
    }
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    #[inline(always)]
    pub fn is_no_prgm_1(&self) -> bool {
        *self == NoPrgm::NoPrgm1
    }
}
#[doc = "Field `NO_PRGM` writer - If set, the TRNG registers cannot be programmed"]
pub type NoPrgmW<'a, REG> = crate::BitWriter<'a, REG, NoPrgm>;
impl<'a, REG> NoPrgmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    #[inline(always)]
    pub fn no_prgm_0(self) -> &'a mut crate::W<REG> {
        self.variant(NoPrgm::NoPrgm0)
    }
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    #[inline(always)]
    pub fn no_prgm_1(self) -> &'a mut crate::W<REG> {
        self.variant(NoPrgm::NoPrgm1)
    }
}
impl R {
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub fn no_prgm(&self) -> NoPrgmR {
        NoPrgmR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CFG")
            .field("no_prgm", &self.no_prgm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    #[must_use]
    pub fn no_prgm(&mut self) -> NoPrgmW<SecCfgSpec> {
        NoPrgmW::new(self, 1)
    }
}
#[doc = "Security Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCfgSpec;
impl crate::RegisterSpec for SecCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_cfg::R`](R) reader structure"]
impl crate::Readable for SecCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_cfg::W`](W) writer structure"]
impl crate::Writable for SecCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CFG to value 0"]
impl crate::Resettable for SecCfgSpec {
    const RESET_VALUE: u32 = 0;
}
