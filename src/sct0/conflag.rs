#[doc = "Register `CONFLAG` reader"]
pub type R = crate::R<ConflagSpec>;
#[doc = "Register `CONFLAG` writer"]
pub type W = crate::W<ConflagSpec>;
#[doc = "Field `NCFLAG` reader - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NcflagR = crate::FieldReader<u16>;
#[doc = "Field `NCFLAG` writer - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NcflagW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BUSERRL` reader - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
pub type BuserrlR = crate::BitReader;
#[doc = "Field `BUSERRL` writer - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
pub type BuserrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSERRH` reader - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
pub type BuserrhR = crate::BitReader;
#[doc = "Field `BUSERRH` writer - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
pub type BuserrhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&self) -> NcflagR {
        NcflagR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&self) -> BuserrlR {
        BuserrlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&self) -> BuserrhR {
        BuserrhR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFLAG")
            .field("ncflag", &self.ncflag())
            .field("buserrl", &self.buserrl())
            .field("buserrh", &self.buserrh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&mut self) -> NcflagW<ConflagSpec> {
        NcflagW::new(self, 0)
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&mut self) -> BuserrlW<ConflagSpec> {
        BuserrlW::new(self, 30)
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&mut self) -> BuserrhW<ConflagSpec> {
        BuserrhW::new(self, 31)
    }
}
#[doc = "SCT conflict flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`conflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConflagSpec;
impl crate::RegisterSpec for ConflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conflag::R`](R) reader structure"]
impl crate::Readable for ConflagSpec {}
#[doc = "`write(|w| ..)` method takes [`conflag::W`](W) writer structure"]
impl crate::Writable for ConflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFLAG to value 0"]
impl crate::Resettable for ConflagSpec {
    const RESET_VALUE: u32 = 0;
}
