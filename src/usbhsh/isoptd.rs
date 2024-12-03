#[doc = "Register `ISOPTD` reader"]
pub type R = crate::R<IsoptdSpec>;
#[doc = "Register `ISOPTD` writer"]
pub type W = crate::W<IsoptdSpec>;
#[doc = "Field `ISO_FIRST` reader - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
pub type IsoFirstR = crate::FieldReader;
#[doc = "Field `ISO_FIRST` writer - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
pub type IsoFirstW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ISO_BASE` reader - Base address to be used by the hardware to find the start of the ISO list."]
pub type IsoBaseR = crate::FieldReader<u32>;
#[doc = "Field `ISO_BASE` writer - Base address to be used by the hardware to find the start of the ISO list."]
pub type IsoBaseW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&self) -> IsoFirstR {
        IsoFirstR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&self) -> IsoBaseR {
        IsoBaseR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISOPTD")
            .field("iso_first", &self.iso_first())
            .field("iso_base", &self.iso_base())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&mut self) -> IsoFirstW<IsoptdSpec> {
        IsoFirstW::new(self, 5)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&mut self) -> IsoBaseW<IsoptdSpec> {
        IsoBaseW::new(self, 10)
    }
}
#[doc = "Memory base address where ISO PTD0 is stored\n\nYou can [`read`](crate::Reg::read) this register and get [`isoptd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isoptd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsoptdSpec;
impl crate::RegisterSpec for IsoptdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isoptd::R`](R) reader structure"]
impl crate::Readable for IsoptdSpec {}
#[doc = "`write(|w| ..)` method takes [`isoptd::W`](W) writer structure"]
impl crate::Writable for IsoptdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISOPTD to value 0"]
impl crate::Resettable for IsoptdSpec {
    const RESET_VALUE: u32 = 0;
}
