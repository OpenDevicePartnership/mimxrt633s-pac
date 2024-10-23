#[doc = "Register `IDLE_CH` reader"]
pub type R = crate::R<IdleChSpec>;
#[doc = "Field `CHAN` reader - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
pub type ChanR = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub fn chan(&self) -> ChanR {
        ChanR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_CH")
            .field("chan", &self.chan())
            .finish()
    }
}
#[doc = "Idle channel register. This register returns the number of the first idle channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleChSpec;
impl crate::RegisterSpec for IdleChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_ch::R`](R) reader structure"]
impl crate::Readable for IdleChSpec {}
#[doc = "`reset()` method sets IDLE_CH to value 0"]
impl crate::Resettable for IdleChSpec {
    const RESET_VALUE: u32 = 0;
}
