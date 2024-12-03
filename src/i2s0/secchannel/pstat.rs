#[doc = "Register `PSTAT` reader"]
pub type R = crate::R<PstatSpec>;
#[doc = "Register `PSTAT` writer"]
pub type W = crate::W<PstatSpec>;
#[doc = "Field `BUSY` reader - Busy status for this channel pair."]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Busy status for this channel pair."]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVFRMERR` reader - Save Frame Error flag."]
pub type SlvfrmerrR = crate::BitReader;
#[doc = "Field `SLVFRMERR` writer - Save Frame Error flag."]
pub type SlvfrmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LR` reader - Left/Right indication."]
pub type LrR = crate::BitReader;
#[doc = "Field `LR` writer - Left/Right indication."]
pub type LrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAPAUSED` reader - Data Paused status flag."]
pub type DatapausedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy status for this channel pair."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Save Frame Error flag."]
    #[inline(always)]
    pub fn slvfrmerr(&self) -> SlvfrmerrR {
        SlvfrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left/Right indication."]
    #[inline(always)]
    pub fn lr(&self) -> LrR {
        LrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Paused status flag."]
    #[inline(always)]
    pub fn datapaused(&self) -> DatapausedR {
        DatapausedR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSTAT")
            .field("busy", &self.busy())
            .field("slvfrmerr", &self.slvfrmerr())
            .field("lr", &self.lr())
            .field("datapaused", &self.datapaused())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Busy status for this channel pair."]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<PstatSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Save Frame Error flag."]
    #[inline(always)]
    pub fn slvfrmerr(&mut self) -> SlvfrmerrW<PstatSpec> {
        SlvfrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Left/Right indication."]
    #[inline(always)]
    pub fn lr(&mut self) -> LrW<PstatSpec> {
        LrW::new(self, 2)
    }
}
#[doc = "Status register for channel pair\n\nYou can [`read`](crate::Reg::read) this register and get [`pstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PstatSpec;
impl crate::RegisterSpec for PstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pstat::R`](R) reader structure"]
impl crate::Readable for PstatSpec {}
#[doc = "`write(|w| ..)` method takes [`pstat::W`](W) writer structure"]
impl crate::Writable for PstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSTAT to value 0"]
impl crate::Resettable for PstatSpec {
    const RESET_VALUE: u32 = 0;
}
