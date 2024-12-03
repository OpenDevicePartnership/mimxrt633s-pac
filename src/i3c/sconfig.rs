#[doc = "Register `SCONFIG` reader"]
pub type R = crate::R<SconfigSpec>;
#[doc = "Register `SCONFIG` writer"]
pub type W = crate::W<SconfigSpec>;
#[doc = "Field `SLVENA` reader - Slave enable"]
pub type SlvenaR = crate::BitReader;
#[doc = "Field `SLVENA` writer - Slave enable"]
pub type SlvenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Not acknowledge"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Not acknowledge"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHSS` reader - Match START or STOP"]
pub type MatchssR = crate::BitReader;
#[doc = "Field `MATCHSS` writer - Match START or STOP"]
pub type MatchssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0IGNORE` reader - S0/S1 errors ignore"]
pub type S0ignoreR = crate::BitReader;
#[doc = "Field `S0IGNORE` writer - S0/S1 errors ignore"]
pub type S0ignoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDROK` reader - Double Data Rate OK"]
pub type DdrokR = crate::BitReader;
#[doc = "Field `DDROK` writer - Double Data Rate OK"]
pub type DdrokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDRAND` reader - ID random"]
pub type IdrandR = crate::BitReader;
#[doc = "Field `IDRAND` writer - ID random"]
pub type IdrandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFLINE` reader - Offline"]
pub type OfflineR = crate::BitReader;
#[doc = "Field `OFFLINE` writer - Offline"]
pub type OfflineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAMATCH` reader - Bus available match"]
pub type BamatchR = crate::FieldReader;
#[doc = "Field `BAMATCH` writer - Bus available match"]
pub type BamatchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SADDR` reader - Static address"]
pub type SaddrR = crate::FieldReader;
#[doc = "Field `SADDR` writer - Static address"]
pub type SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Slave enable"]
    #[inline(always)]
    pub fn slvena(&self) -> SlvenaR {
        SlvenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Not acknowledge"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Match START or STOP"]
    #[inline(always)]
    pub fn matchss(&self) -> MatchssR {
        MatchssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S0/S1 errors ignore"]
    #[inline(always)]
    pub fn s0ignore(&self) -> S0ignoreR {
        S0ignoreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Double Data Rate OK"]
    #[inline(always)]
    pub fn ddrok(&self) -> DdrokR {
        DdrokR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ID random"]
    #[inline(always)]
    pub fn idrand(&self) -> IdrandR {
        IdrandR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Offline"]
    #[inline(always)]
    pub fn offline(&self) -> OfflineR {
        OfflineR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bus available match"]
    #[inline(always)]
    pub fn bamatch(&self) -> BamatchR {
        BamatchR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 25:31 - Static address"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCONFIG")
            .field("slvena", &self.slvena())
            .field("nack", &self.nack())
            .field("matchss", &self.matchss())
            .field("s0ignore", &self.s0ignore())
            .field("ddrok", &self.ddrok())
            .field("idrand", &self.idrand())
            .field("offline", &self.offline())
            .field("bamatch", &self.bamatch())
            .field("saddr", &self.saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Slave enable"]
    #[inline(always)]
    pub fn slvena(&mut self) -> SlvenaW<SconfigSpec> {
        SlvenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Not acknowledge"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<SconfigSpec> {
        NackW::new(self, 1)
    }
    #[doc = "Bit 2 - Match START or STOP"]
    #[inline(always)]
    pub fn matchss(&mut self) -> MatchssW<SconfigSpec> {
        MatchssW::new(self, 2)
    }
    #[doc = "Bit 3 - S0/S1 errors ignore"]
    #[inline(always)]
    pub fn s0ignore(&mut self) -> S0ignoreW<SconfigSpec> {
        S0ignoreW::new(self, 3)
    }
    #[doc = "Bit 4 - Double Data Rate OK"]
    #[inline(always)]
    pub fn ddrok(&mut self) -> DdrokW<SconfigSpec> {
        DdrokW::new(self, 4)
    }
    #[doc = "Bit 8 - ID random"]
    #[inline(always)]
    pub fn idrand(&mut self) -> IdrandW<SconfigSpec> {
        IdrandW::new(self, 8)
    }
    #[doc = "Bit 9 - Offline"]
    #[inline(always)]
    pub fn offline(&mut self) -> OfflineW<SconfigSpec> {
        OfflineW::new(self, 9)
    }
    #[doc = "Bits 16:23 - Bus available match"]
    #[inline(always)]
    pub fn bamatch(&mut self) -> BamatchW<SconfigSpec> {
        BamatchW::new(self, 16)
    }
    #[doc = "Bits 25:31 - Static address"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SaddrW<SconfigSpec> {
        SaddrW::new(self, 25)
    }
}
#[doc = "Slave Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SconfigSpec;
impl crate::RegisterSpec for SconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sconfig::R`](R) reader structure"]
impl crate::Readable for SconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`sconfig::W`](W) writer structure"]
impl crate::Writable for SconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCONFIG to value 0"]
impl crate::Resettable for SconfigSpec {
    const RESET_VALUE: u32 = 0;
}
