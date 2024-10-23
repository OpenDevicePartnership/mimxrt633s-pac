#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SSAEN` reader - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SsaenR = crate::BitReader;
#[doc = "Field `SSAEN` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SsaenW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SSDEN` reader - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SsdenR = crate::BitReader;
#[doc = "Field `SSDEN` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type SsdenW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MSTIDLE` reader - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type MstidleR = crate::BitReader;
#[doc = "Field `MSTIDLE` writer - Writing 1 clears the corresponding bit in the INTENSET register."]
pub type MstidleW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 4 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn ssaen(&self) -> SsaenR {
        SsaenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn ssden(&self) -> SsdenR {
        SsdenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn mstidle(&self) -> MstidleR {
        MstidleR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("ssaen", &self.ssaen())
            .field("ssden", &self.ssden())
            .field("mstidle", &self.mstidle())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn ssaen(&mut self) -> SsaenW<IntenclrSpec> {
        SsaenW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn ssden(&mut self) -> SsdenW<IntenclrSpec> {
        SsdenW::new(self, 5)
    }
    #[doc = "Bit 8 - Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn mstidle(&mut self) -> MstidleW<IntenclrSpec> {
        MstidleW::new(self, 8)
    }
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0130;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
