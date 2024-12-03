#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `READYEN` reader - Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
pub type ReadyenR = crate::BitReader;
#[doc = "Field `READYEN` writer - Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
pub type ReadyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUCCESEN` reader - Enable corresponding interrupt in STAT, which indicates last operation was successful."]
pub type SuccesenR = crate::BitReader;
#[doc = "Field `SUCCESEN` writer - Enable corresponding interrupt in STAT, which indicates last operation was successful."]
pub type SuccesenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROREN` reader - Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
pub type ErrorenR = crate::BitReader;
#[doc = "Field `ERROREN` writer - Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
pub type ErrorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYINREQEN` reader - Enable corresponding interrupt in STAT, which is request for next part of key."]
pub type KeyinreqenR = crate::BitReader;
#[doc = "Field `KEYINREQEN` writer - Enable corresponding interrupt in STAT, which is request for next part of key."]
pub type KeyinreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYOUTAVAILEN` reader - Enable corresponding interrupt in STAT, which is next part of key is available."]
pub type KeyoutavailenR = crate::BitReader;
#[doc = "Field `KEYOUTAVAILEN` writer - Enable corresponding interrupt in STAT, which is next part of key is available."]
pub type KeyoutavailenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEINREQEN` reader - Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
pub type CodeinreqenR = crate::BitReader;
#[doc = "Field `CODEINREQEN` writer - Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
pub type CodeinreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEOUTAVAILEN` reader - Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
pub type CodeoutavailenR = crate::BitReader;
#[doc = "Field `CODEOUTAVAILEN` writer - Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
pub type CodeoutavailenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
    #[inline(always)]
    pub fn readyen(&self) -> ReadyenR {
        ReadyenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable corresponding interrupt in STAT, which indicates last operation was successful."]
    #[inline(always)]
    pub fn succesen(&self) -> SuccesenR {
        SuccesenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
    #[inline(always)]
    pub fn erroren(&self) -> ErrorenR {
        ErrorenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable corresponding interrupt in STAT, which is request for next part of key."]
    #[inline(always)]
    pub fn keyinreqen(&self) -> KeyinreqenR {
        KeyinreqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable corresponding interrupt in STAT, which is next part of key is available."]
    #[inline(always)]
    pub fn keyoutavailen(&self) -> KeyoutavailenR {
        KeyoutavailenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
    #[inline(always)]
    pub fn codeinreqen(&self) -> CodeinreqenR {
        CodeinreqenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
    #[inline(always)]
    pub fn codeoutavailen(&self) -> CodeoutavailenR {
        CodeoutavailenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("readyen", &self.readyen())
            .field("succesen", &self.succesen())
            .field("erroren", &self.erroren())
            .field("keyinreqen", &self.keyinreqen())
            .field("keyoutavailen", &self.keyoutavailen())
            .field("codeinreqen", &self.codeinreqen())
            .field("codeoutavailen", &self.codeoutavailen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
    #[inline(always)]
    pub fn readyen(&mut self) -> ReadyenW<IntenSpec> {
        ReadyenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable corresponding interrupt in STAT, which indicates last operation was successful."]
    #[inline(always)]
    pub fn succesen(&mut self) -> SuccesenW<IntenSpec> {
        SuccesenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
    #[inline(always)]
    pub fn erroren(&mut self) -> ErrorenW<IntenSpec> {
        ErrorenW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable corresponding interrupt in STAT, which is request for next part of key."]
    #[inline(always)]
    pub fn keyinreqen(&mut self) -> KeyinreqenW<IntenSpec> {
        KeyinreqenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable corresponding interrupt in STAT, which is next part of key is available."]
    #[inline(always)]
    pub fn keyoutavailen(&mut self) -> KeyoutavailenW<IntenSpec> {
        KeyoutavailenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
    #[inline(always)]
    pub fn codeinreqen(&mut self) -> CodeinreqenW<IntenSpec> {
        CodeinreqenW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
    #[inline(always)]
    pub fn codeoutavailen(&mut self) -> CodeoutavailenW<IntenSpec> {
        CodeoutavailenW::new(self, 7)
    }
}
#[doc = "PUF Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
