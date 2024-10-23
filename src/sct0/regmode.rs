#[doc = "Register `REGMODE` reader"]
pub type R = crate::R<RegmodeSpec>;
#[doc = "Register `REGMODE` writer"]
pub type W = crate::W<RegmodeSpec>;
#[doc = "Field `REGMOD_L` reader - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
pub type RegmodLR = crate::FieldReader<u16>;
#[doc = "Field `REGMOD_L` writer - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
pub type RegmodLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGMOD_H` reader - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
pub type RegmodHR = crate::FieldReader<u16>;
#[doc = "Field `REGMOD_H` writer - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
pub type RegmodHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub fn regmod_l(&self) -> RegmodLR {
        RegmodLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub fn regmod_h(&self) -> RegmodHR {
        RegmodHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGMODE")
            .field("regmod_l", &self.regmod_l())
            .field("regmod_h", &self.regmod_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l(&mut self) -> RegmodLW<RegmodeSpec> {
        RegmodLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h(&mut self) -> RegmodHW<RegmodeSpec> {
        RegmodHW::new(self, 16)
    }
}
#[doc = "SCT match/capture mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`regmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegmodeSpec;
impl crate::RegisterSpec for RegmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regmode::R`](R) reader structure"]
impl crate::Readable for RegmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`regmode::W`](W) writer structure"]
impl crate::Writable for RegmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGMODE to value 0"]
impl crate::Resettable for RegmodeSpec {
    const RESET_VALUE: u32 = 0;
}
