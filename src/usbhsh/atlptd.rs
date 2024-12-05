#[doc = "Register `ATLPTD` reader"]
pub type R = crate::R<AtlptdSpec>;
#[doc = "Register `ATLPTD` writer"]
pub type W = crate::W<AtlptdSpec>;
#[doc = "Field `ATL_CUR` reader - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
pub type AtlCurR = crate::FieldReader;
#[doc = "Field `ATL_CUR` writer - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
pub type AtlCurW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATL_BASE` reader - Base address to be used by the hardware to find the start of the ATL list."]
pub type AtlBaseR = crate::FieldReader<u32>;
#[doc = "Field `ATL_BASE` writer - Base address to be used by the hardware to find the start of the ATL list."]
pub type AtlBaseW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub fn atl_cur(&self) -> AtlCurR {
        AtlCurR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub fn atl_base(&self) -> AtlBaseR {
        AtlBaseR::new((self.bits >> 9) & 0x007f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATLPTD")
            .field("atl_cur", &self.atl_cur())
            .field("atl_base", &self.atl_base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub fn atl_cur(&mut self) -> AtlCurW<AtlptdSpec> {
        AtlCurW::new(self, 4)
    }
    #[doc = "Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub fn atl_base(&mut self) -> AtlBaseW<AtlptdSpec> {
        AtlBaseW::new(self, 9)
    }
}
#[doc = "Memory base address where ATL PTD0 is stored\n\nYou can [`read`](crate::Reg::read) this register and get [`atlptd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atlptd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtlptdSpec;
impl crate::RegisterSpec for AtlptdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atlptd::R`](R) reader structure"]
impl crate::Readable for AtlptdSpec {}
#[doc = "`write(|w| ..)` method takes [`atlptd::W`](W) writer structure"]
impl crate::Writable for AtlptdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATLPTD to value 0"]
impl crate::Resettable for AtlptdSpec {
    const RESET_VALUE: u32 = 0;
}
