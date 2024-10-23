#[doc = "Register `MCR1` reader"]
pub type R = crate::R<Mcr1Spec>;
#[doc = "Register `MCR1` writer"]
pub type W = crate::W<Mcr1Spec>;
#[doc = "Field `AHBBUSWAIT` reader - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
pub type AhbbuswaitR = crate::FieldReader<u16>;
#[doc = "Field `AHBBUSWAIT` writer - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
pub type AhbbuswaitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SEQWAIT` reader - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
pub type SeqwaitR = crate::FieldReader<u16>;
#[doc = "Field `SEQWAIT` writer - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
pub type SeqwaitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub fn ahbbuswait(&self) -> AhbbuswaitR {
        AhbbuswaitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub fn seqwait(&self) -> SeqwaitR {
        SeqwaitR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR1")
            .field("ahbbuswait", &self.ahbbuswait())
            .field("seqwait", &self.seqwait())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    #[must_use]
    pub fn ahbbuswait(&mut self) -> AhbbuswaitW<Mcr1Spec> {
        AhbbuswaitW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn seqwait(&mut self) -> SeqwaitW<Mcr1Spec> {
        SeqwaitW::new(self, 16)
    }
}
#[doc = "Module Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcr1Spec;
impl crate::RegisterSpec for Mcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr1::R`](R) reader structure"]
impl crate::Readable for Mcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcr1::W`](W) writer structure"]
impl crate::Writable for Mcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR1 to value 0xffff_ffff"]
impl crate::Resettable for Mcr1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
