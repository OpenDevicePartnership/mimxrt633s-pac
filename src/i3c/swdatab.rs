#[doc = "Register `SWDATAB` writer"]
pub type W = crate::W<SwdatabSpec>;
#[doc = "Field `DATA` writer - The data byte to send to the master"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `END` writer - End"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_ALSO` writer - End also"]
pub type EndAlsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SwdatabSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - The data byte to send to the master"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<SwdatabSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - End"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<SwdatabSpec> {
        EndW::new(self, 8)
    }
    #[doc = "Bit 16 - End also"]
    #[inline(always)]
    pub fn end_also(&mut self) -> EndAlsoW<SwdatabSpec> {
        EndAlsoW::new(self, 16)
    }
}
#[doc = "Slave Write Data Byte Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatab::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwdatabSpec;
impl crate::RegisterSpec for SwdatabSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swdatab::W`](W) writer structure"]
impl crate::Writable for SwdatabSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWDATAB to value 0"]
impl crate::Resettable for SwdatabSpec {
    const RESET_VALUE: u32 = 0;
}
