#[doc = "Register `DSPSTALL` reader"]
pub type R = crate::R<DspstallSpec>;
#[doc = "Register `DSPSTALL` writer"]
pub type W = crate::W<DspstallSpec>;
#[doc = "Run / Stall Register. . .\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dspstall {
    #[doc = "0: Run (Normal) Mode."]
    RunMode = 0,
    #[doc = "1: Stall Mode."]
    StallMode = 1,
}
impl From<Dspstall> for bool {
    #[inline(always)]
    fn from(variant: Dspstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSPSTALL` reader - Run / Stall Register. . ."]
pub type DspstallR = crate::BitReader<Dspstall>;
impl DspstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dspstall {
        match self.bits {
            false => Dspstall::RunMode,
            true => Dspstall::StallMode,
        }
    }
    #[doc = "Run (Normal) Mode."]
    #[inline(always)]
    pub fn is_run_mode(&self) -> bool {
        *self == Dspstall::RunMode
    }
    #[doc = "Stall Mode."]
    #[inline(always)]
    pub fn is_stall_mode(&self) -> bool {
        *self == Dspstall::StallMode
    }
}
#[doc = "Field `DSPSTALL` writer - Run / Stall Register. . ."]
pub type DspstallW<'a, REG> = crate::BitWriter<'a, REG, Dspstall>;
impl<'a, REG> DspstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Run (Normal) Mode."]
    #[inline(always)]
    pub fn run_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Dspstall::RunMode)
    }
    #[doc = "Stall Mode."]
    #[inline(always)]
    pub fn stall_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Dspstall::StallMode)
    }
}
impl R {
    #[doc = "Bit 0 - Run / Stall Register. . ."]
    #[inline(always)]
    pub fn dspstall(&self) -> DspstallR {
        DspstallR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSPSTALL")
            .field("dspstall", &self.dspstall())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Run / Stall Register. . ."]
    #[inline(always)]
    pub fn dspstall(&mut self) -> DspstallW<DspstallSpec> {
        DspstallW::new(self, 0)
    }
}
#[doc = "DSP stall register\n\nYou can [`read`](crate::Reg::read) this register and get [`dspstall::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dspstall::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DspstallSpec;
impl crate::RegisterSpec for DspstallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dspstall::R`](R) reader structure"]
impl crate::Readable for DspstallSpec {}
#[doc = "`write(|w| ..)` method takes [`dspstall::W`](W) writer structure"]
impl crate::Writable for DspstallSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSPSTALL to value 0x01"]
impl crate::Resettable for DspstallSpec {
    const RESET_VALUE: u32 = 0x01;
}
