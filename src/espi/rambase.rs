#[doc = "Register `RAMBASE` reader"]
pub type R = crate::R<RambaseSpec>;
#[doc = "Register `RAMBASE` writer"]
pub type W = crate::W<RambaseSpec>;
#[doc = "Field `ZERO` reader - Always 0"]
pub type ZeroR = crate::FieldReader<u16>;
#[doc = "Field `RAM` reader - Is location in System memory space where RAM is located that is used by this peripheral's bus master"]
pub type RamR = crate::FieldReader<u32>;
#[doc = "Field `RAM` writer - Is location in System memory space where RAM is located that is used by this peripheral's bus master"]
pub type RamW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - Always 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - Is location in System memory space where RAM is located that is used by this peripheral's bus master"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMBASE")
            .field("zero", &self.zero())
            .field("ram", &self.ram())
            .finish()
    }
}
impl W {
    #[doc = "Bits 12:31 - Is location in System memory space where RAM is located that is used by this peripheral's bus master"]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RamW<RambaseSpec> {
        RamW::new(self, 12)
    }
}
#[doc = "Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable).\n\nYou can [`read`](crate::Reg::read) this register and get [`rambase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rambase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RambaseSpec;
impl crate::RegisterSpec for RambaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rambase::R`](R) reader structure"]
impl crate::Readable for RambaseSpec {}
#[doc = "`write(|w| ..)` method takes [`rambase::W`](W) writer structure"]
impl crate::Writable for RambaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAMBASE to value 0"]
impl crate::Resettable for RambaseSpec {
    const RESET_VALUE: u32 = 0;
}
