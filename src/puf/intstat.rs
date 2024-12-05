#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `READY` reader - Ready"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - Ready"]
pub type ReadyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SUCCESS` reader - Success"]
pub type SuccessR = crate::BitReader;
#[doc = "Field `SUCCESS` writer - Success"]
pub type SuccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYINREQ` reader - Key In Request"]
pub type KeyinreqR = crate::BitReader;
#[doc = "Field `KEYINREQ` writer - Key In Request"]
pub type KeyinreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYOUTAVAIL` reader - Key Out Available"]
pub type KeyoutavailR = crate::BitReader;
#[doc = "Field `KEYOUTAVAIL` writer - Key Out Available"]
pub type KeyoutavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEINREQ` reader - Code In Request"]
pub type CodeinreqR = crate::BitReader;
#[doc = "Field `CODEINREQ` writer - Code In Request"]
pub type CodeinreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEOUTAVAIL` reader - Code Out Available"]
pub type CodeoutavailR = crate::BitReader;
#[doc = "Field `CODEOUTAVAIL` writer - Code Out Available"]
pub type CodeoutavailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Success"]
    #[inline(always)]
    pub fn success(&self) -> SuccessR {
        SuccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Key In Request"]
    #[inline(always)]
    pub fn keyinreq(&self) -> KeyinreqR {
        KeyinreqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Key Out Available"]
    #[inline(always)]
    pub fn keyoutavail(&self) -> KeyoutavailR {
        KeyoutavailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Code In Request"]
    #[inline(always)]
    pub fn codeinreq(&self) -> CodeinreqR {
        CodeinreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Code Out Available"]
    #[inline(always)]
    pub fn codeoutavail(&self) -> CodeoutavailR {
        CodeoutavailR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("ready", &self.ready())
            .field("success", &self.success())
            .field("error", &self.error())
            .field("keyinreq", &self.keyinreq())
            .field("keyoutavail", &self.keyoutavail())
            .field("codeinreq", &self.codeinreq())
            .field("codeoutavail", &self.codeoutavail())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Ready"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<IntstatSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Success"]
    #[inline(always)]
    pub fn success(&mut self) -> SuccessW<IntstatSpec> {
        SuccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Error"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IntstatSpec> {
        ErrorW::new(self, 2)
    }
    #[doc = "Bit 4 - Key In Request"]
    #[inline(always)]
    pub fn keyinreq(&mut self) -> KeyinreqW<IntstatSpec> {
        KeyinreqW::new(self, 4)
    }
    #[doc = "Bit 5 - Key Out Available"]
    #[inline(always)]
    pub fn keyoutavail(&mut self) -> KeyoutavailW<IntstatSpec> {
        KeyoutavailW::new(self, 5)
    }
    #[doc = "Bit 6 - Code In Request"]
    #[inline(always)]
    pub fn codeinreq(&mut self) -> CodeinreqW<IntstatSpec> {
        CodeinreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Code Out Available"]
    #[inline(always)]
    pub fn codeoutavail(&mut self) -> CodeoutavailW<IntstatSpec> {
        CodeoutavailW::new(self, 7)
    }
}
#[doc = "PUF Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {
    const RESET_VALUE: u32 = 0;
}
