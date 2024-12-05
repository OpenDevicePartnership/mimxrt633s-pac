#[doc = "Register `LVDCORECTRL` reader"]
pub type R = crate::R<LvdcorectrlSpec>;
#[doc = "Register `LVDCORECTRL` writer"]
pub type W = crate::W<LvdcorectrlSpec>;
#[doc = "Field `LVDCORELVL` reader - Vddcore LVD falling trip voltage"]
pub type LvdcorelvlR = crate::FieldReader;
#[doc = "Field `LVDCORELVL` writer - Vddcore LVD falling trip voltage"]
pub type LvdcorelvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Vddcore LVD falling trip voltage"]
    #[inline(always)]
    pub fn lvdcorelvl(&self) -> LvdcorelvlR {
        LvdcorelvlR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVDCORECTRL")
            .field("lvdcorelvl", &self.lvdcorelvl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Vddcore LVD falling trip voltage"]
    #[inline(always)]
    pub fn lvdcorelvl(&mut self) -> LvdcorelvlW<LvdcorectrlSpec> {
        LvdcorelvlW::new(self, 0)
    }
}
#[doc = "Active vddcore LVD monitor trip adjust\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdcorectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcorectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LvdcorectrlSpec;
impl crate::RegisterSpec for LvdcorectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvdcorectrl::R`](R) reader structure"]
impl crate::Readable for LvdcorectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lvdcorectrl::W`](W) writer structure"]
impl crate::Writable for LvdcorectrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LVDCORECTRL to value 0"]
impl crate::Resettable for LvdcorectrlSpec {
    const RESET_VALUE: u32 = 0;
}
