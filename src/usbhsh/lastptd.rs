#[doc = "Register `LASTPTD` reader"]
pub type R = crate::R<LastptdSpec>;
#[doc = "Register `LASTPTD` writer"]
pub type W = crate::W<LastptdSpec>;
#[doc = "Field `ATL_LAST` reader - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
pub type AtlLastR = crate::FieldReader;
#[doc = "Field `ATL_LAST` writer - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
pub type AtlLastW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ISO_LAST` reader - This indicates the last PTD in the ISO list."]
pub type IsoLastR = crate::FieldReader;
#[doc = "Field `ISO_LAST` writer - This indicates the last PTD in the ISO list."]
pub type IsoLastW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT_LAST` reader - This indicates the last PTD in the INT list."]
pub type IntLastR = crate::FieldReader;
#[doc = "Field `INT_LAST` writer - This indicates the last PTD in the INT list."]
pub type IntLastW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&self) -> AtlLastR {
        AtlLastR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&self) -> IsoLastR {
        IsoLastR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&self) -> IntLastR {
        IntLastR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LASTPTD")
            .field("atl_last", &self.atl_last())
            .field("iso_last", &self.iso_last())
            .field("int_last", &self.int_last())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&mut self) -> AtlLastW<LastptdSpec> {
        AtlLastW::new(self, 0)
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&mut self) -> IsoLastW<LastptdSpec> {
        IsoLastW::new(self, 8)
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&mut self) -> IntLastW<LastptdSpec> {
        IntLastW::new(self, 16)
    }
}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL\n\nYou can [`read`](crate::Reg::read) this register and get [`lastptd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lastptd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LastptdSpec;
impl crate::RegisterSpec for LastptdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lastptd::R`](R) reader structure"]
impl crate::Readable for LastptdSpec {}
#[doc = "`write(|w| ..)` method takes [`lastptd::W`](W) writer structure"]
impl crate::Writable for LastptdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LASTPTD to value 0"]
impl crate::Resettable for LastptdSpec {
    const RESET_VALUE: u32 = 0;
}
