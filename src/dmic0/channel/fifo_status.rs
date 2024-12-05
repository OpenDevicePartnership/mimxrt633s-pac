#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FifoStatusSpec>;
#[doc = "Register `FIFO_STATUS` writer"]
pub type W = crate::W<FifoStatusSpec>;
#[doc = "Field `INT` reader - Status of Interrupt (write 1 to clear)"]
pub type IntR = crate::BitReader;
#[doc = "Field `INT` writer - Status of Interrupt (write 1 to clear)"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Overrun Detected (write 1 to clear)"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Overrun Detected (write 1 to clear)"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` reader - Underrun Detected (write 1 to clear)"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Underrun Detected (write 1 to clear)"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Status of Interrupt (write 1 to clear)"]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_STATUS")
            .field("int", &self.int())
            .field("overrun", &self.overrun())
            .field("underrun", &self.underrun())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Status of Interrupt (write 1 to clear)"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<FifoStatusSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<FifoStatusSpec> {
        OverrunW::new(self, 1)
    }
    #[doc = "Bit 2 - Underrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<FifoStatusSpec> {
        UnderrunW::new(self, 2)
    }
}
#[doc = "FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoStatusSpec;
impl crate::RegisterSpec for FifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FifoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_status::W`](W) writer structure"]
impl crate::Writable for FifoStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_STATUS to value 0"]
impl crate::Resettable for FifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
