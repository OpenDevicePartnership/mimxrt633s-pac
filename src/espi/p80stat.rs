#[doc = "Register `P80STAT` reader"]
pub type R = crate::R<P80statSpec>;
#[doc = "Register `P80STAT` writer"]
pub type W = crate::W<P80statSpec>;
#[doc = "Field `CURR` reader - Has the Current Port80 value."]
pub type CurrR = crate::FieldReader;
#[doc = "Field `PREV` reader - Has the previous Port80 value."]
pub type PrevR = crate::FieldReader;
#[doc = "Field `CNT` reader - A counter (set to 0 on enable of p80) which increments with each new value. Will wrap back to 0."]
pub type CntR = crate::FieldReader;
#[doc = "Field `RST` reader - Resets counter, CNT, back to 0."]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Resets counter, CNT, back to 0."]
pub type RstW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Has the Current Port80 value."]
    #[inline(always)]
    pub fn curr(&self) -> CurrR {
        CurrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Has the previous Port80 value."]
    #[inline(always)]
    pub fn prev(&self) -> PrevR {
        PrevR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - A counter (set to 0 on enable of p80) which increments with each new value. Will wrap back to 0."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Resets counter, CNT, back to 0."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P80STAT")
            .field("curr", &self.curr())
            .field("prev", &self.prev())
            .field("cnt", &self.cnt())
            .field("rst", &self.rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - Resets counter, CNT, back to 0."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<P80statSpec> {
        RstW::new(self, 20)
    }
}
#[doc = "Port 80 Status (byte and prev byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`p80stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p80stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P80statSpec;
impl crate::RegisterSpec for P80statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p80stat::R`](R) reader structure"]
impl crate::Readable for P80statSpec {}
#[doc = "`write(|w| ..)` method takes [`p80stat::W`](W) writer structure"]
impl crate::Writable for P80statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_0000;
}
#[doc = "`reset()` method sets P80STAT to value 0"]
impl crate::Resettable for P80statSpec {
    const RESET_VALUE: u32 = 0;
}
