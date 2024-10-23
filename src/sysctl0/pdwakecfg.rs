#[doc = "Register `PDWAKECFG` reader"]
pub type R = crate::R<PdwakecfgSpec>;
#[doc = "Register `PDWAKECFG` writer"]
pub type W = crate::W<PdwakecfgSpec>;
#[doc = "RBB mode on wakeup\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbbkeepst {
    #[doc = "0: Use value of RBB_PD in PDRUNCFG on wakeup."]
    Rbbkeepst0 = 0,
    #[doc = "1: Copy PDSLEEPCFG RBB_PD value to PDRUNCFG RBB_PD on wakeup to keep RBB state."]
    Rbbkeepst1 = 1,
}
impl From<Rbbkeepst> for bool {
    #[inline(always)]
    fn from(variant: Rbbkeepst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBBKEEPST` reader - RBB mode on wakeup"]
pub type RbbkeepstR = crate::BitReader<Rbbkeepst>;
impl RbbkeepstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbbkeepst {
        match self.bits {
            false => Rbbkeepst::Rbbkeepst0,
            true => Rbbkeepst::Rbbkeepst1,
        }
    }
    #[doc = "Use value of RBB_PD in PDRUNCFG on wakeup."]
    #[inline(always)]
    pub fn is_rbbkeepst_0(&self) -> bool {
        *self == Rbbkeepst::Rbbkeepst0
    }
    #[doc = "Copy PDSLEEPCFG RBB_PD value to PDRUNCFG RBB_PD on wakeup to keep RBB state."]
    #[inline(always)]
    pub fn is_rbbkeepst_1(&self) -> bool {
        *self == Rbbkeepst::Rbbkeepst1
    }
}
#[doc = "Field `RBBKEEPST` writer - RBB mode on wakeup"]
pub type RbbkeepstW<'a, REG> = crate::BitWriter<'a, REG, Rbbkeepst>;
impl<'a, REG> RbbkeepstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use value of RBB_PD in PDRUNCFG on wakeup."]
    #[inline(always)]
    pub fn rbbkeepst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rbbkeepst::Rbbkeepst0)
    }
    #[doc = "Copy PDSLEEPCFG RBB_PD value to PDRUNCFG RBB_PD on wakeup to keep RBB state."]
    #[inline(always)]
    pub fn rbbkeepst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rbbkeepst::Rbbkeepst1)
    }
}
#[doc = "FBB mode on wakeup\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fbbkeepst {
    #[doc = "0: Use value of FBB_PD in PDRUNCFG on wakeup"]
    Fbbkeepst0 = 0,
    #[doc = "1: Copy PDSLEEPCFG FBB_PD value to PDRUNCFG FBB_PD on wakeup to keep FBB state"]
    Fbbkeepst1 = 1,
}
impl From<Fbbkeepst> for bool {
    #[inline(always)]
    fn from(variant: Fbbkeepst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBBKEEPST` reader - FBB mode on wakeup"]
pub type FbbkeepstR = crate::BitReader<Fbbkeepst>;
impl FbbkeepstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fbbkeepst {
        match self.bits {
            false => Fbbkeepst::Fbbkeepst0,
            true => Fbbkeepst::Fbbkeepst1,
        }
    }
    #[doc = "Use value of FBB_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn is_fbbkeepst_0(&self) -> bool {
        *self == Fbbkeepst::Fbbkeepst0
    }
    #[doc = "Copy PDSLEEPCFG FBB_PD value to PDRUNCFG FBB_PD on wakeup to keep FBB state"]
    #[inline(always)]
    pub fn is_fbbkeepst_1(&self) -> bool {
        *self == Fbbkeepst::Fbbkeepst1
    }
}
#[doc = "Field `FBBKEEPST` writer - FBB mode on wakeup"]
pub type FbbkeepstW<'a, REG> = crate::BitWriter<'a, REG, Fbbkeepst>;
impl<'a, REG> FbbkeepstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use value of FBB_PD in PDRUNCFG on wakeup"]
    #[inline(always)]
    pub fn fbbkeepst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fbbkeepst::Fbbkeepst0)
    }
    #[doc = "Copy PDSLEEPCFG FBB_PD value to PDRUNCFG FBB_PD on wakeup to keep FBB state"]
    #[inline(always)]
    pub fn fbbkeepst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fbbkeepst::Fbbkeepst1)
    }
}
impl R {
    #[doc = "Bit 0 - RBB mode on wakeup"]
    #[inline(always)]
    pub fn rbbkeepst(&self) -> RbbkeepstR {
        RbbkeepstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FBB mode on wakeup"]
    #[inline(always)]
    pub fn fbbkeepst(&self) -> FbbkeepstR {
        FbbkeepstR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDWAKECFG")
            .field("rbbkeepst", &self.rbbkeepst())
            .field("fbbkeepst", &self.fbbkeepst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RBB mode on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rbbkeepst(&mut self) -> RbbkeepstW<PdwakecfgSpec> {
        RbbkeepstW::new(self, 0)
    }
    #[doc = "Bit 1 - FBB mode on wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fbbkeepst(&mut self) -> FbbkeepstW<PdwakecfgSpec> {
        FbbkeepstW::new(self, 1)
    }
}
#[doc = "PD Wake Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwakecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwakecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdwakecfgSpec;
impl crate::RegisterSpec for PdwakecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdwakecfg::R`](R) reader structure"]
impl crate::Readable for PdwakecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pdwakecfg::W`](W) writer structure"]
impl crate::Writable for PdwakecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDWAKECFG to value 0"]
impl crate::Resettable for PdwakecfgSpec {
    const RESET_VALUE: u32 = 0;
}
