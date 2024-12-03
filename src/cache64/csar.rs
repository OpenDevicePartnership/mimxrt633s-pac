#[doc = "Register `CSAR` reader"]
pub type R = crate::R<CsarSpec>;
#[doc = "Register `CSAR` writer"]
pub type W = crate::W<CsarSpec>;
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgo {
    #[doc = "0: Write: no effect. Read: no line command active."]
    NoEffect = 0,
    #[doc = "1: Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    InitCmd = 1,
}
impl From<Lgo> for bool {
    #[inline(always)]
    fn from(variant: Lgo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGO` reader - Initiate Cache Line Command"]
pub type LgoR = crate::BitReader<Lgo>;
impl LgoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgo {
        match self.bits {
            false => Lgo::NoEffect,
            true => Lgo::InitCmd,
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Lgo::NoEffect
    }
    #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == Lgo::InitCmd
    }
}
#[doc = "Field `LGO` writer - Initiate Cache Line Command"]
pub type LgoW<'a, REG> = crate::BitWriter<'a, REG, Lgo>;
impl<'a, REG> LgoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lgo::NoEffect)
    }
    #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Lgo::InitCmd)
    }
}
#[doc = "Field `PHYADDR27_1` reader - Physical Address"]
pub type Phyaddr27_1R = crate::FieldReader<u32>;
#[doc = "Field `PHYADDR27_1` writer - Physical Address"]
pub type Phyaddr27_1W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `PHYADDR31_29` reader - Physical Address"]
pub type Phyaddr31_29R = crate::FieldReader;
#[doc = "Field `PHYADDR31_29` writer - Physical Address"]
pub type Phyaddr31_29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LgoR {
        LgoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:27 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr27_1(&self) -> Phyaddr27_1R {
        Phyaddr27_1R::new((self.bits >> 1) & 0x07ff_ffff)
    }
    #[doc = "Bits 29:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr31_29(&self) -> Phyaddr31_29R {
        Phyaddr31_29R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSAR")
            .field("lgo", &self.lgo())
            .field("phyaddr27_1", &self.phyaddr27_1())
            .field("phyaddr31_29", &self.phyaddr31_29())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&mut self) -> LgoW<CsarSpec> {
        LgoW::new(self, 0)
    }
    #[doc = "Bits 1:27 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr27_1(&mut self) -> Phyaddr27_1W<CsarSpec> {
        Phyaddr27_1W::new(self, 1)
    }
    #[doc = "Bits 29:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr31_29(&mut self) -> Phyaddr31_29W<CsarSpec> {
        Phyaddr31_29W::new(self, 29)
    }
}
#[doc = "Cache search address register\n\nYou can [`read`](crate::Reg::read) this register and get [`csar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsarSpec;
impl crate::RegisterSpec for CsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csar::R`](R) reader structure"]
impl crate::Readable for CsarSpec {}
#[doc = "`write(|w| ..)` method takes [`csar::W`](W) writer structure"]
impl crate::Writable for CsarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSAR to value 0"]
impl crate::Resettable for CsarSpec {
    const RESET_VALUE: u32 = 0;
}
