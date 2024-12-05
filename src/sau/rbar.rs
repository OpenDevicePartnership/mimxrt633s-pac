#[doc = "Register `RBAR` reader"]
pub type R = crate::R<RbarSpec>;
#[doc = "Register `RBAR` writer"]
pub type W = crate::W<RbarSpec>;
#[doc = "Field `BADDR` reader - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
pub type BaddrR = crate::FieldReader<u32>;
#[doc = "Field `BADDR` writer - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
pub type BaddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 5:31 - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
    #[inline(always)]
    pub fn baddr(&self) -> BaddrR {
        BaddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBAR")
            .field("baddr", &self.baddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:31 - Base address. Holds bits\\[31:5\\]
of the base address for the selected SAU region. Bits\\[4:0\\]
of the base address are defined as 0x00."]
    #[inline(always)]
    pub fn baddr(&mut self) -> BaddrW<RbarSpec> {
        BaddrW::new(self, 5)
    }
}
#[doc = "Security Attribution Unit Region Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbarSpec;
impl crate::RegisterSpec for RbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbar::R`](R) reader structure"]
impl crate::Readable for RbarSpec {}
#[doc = "`write(|w| ..)` method takes [`rbar::W`](W) writer structure"]
impl crate::Writable for RbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBAR to value 0"]
impl crate::Resettable for RbarSpec {
    const RESET_VALUE: u32 = 0;
}
