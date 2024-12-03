#[doc = "Register `AHBRXBUF1CR0` reader"]
pub type R = crate::R<Ahbrxbuf1cr0Spec>;
#[doc = "Register `AHBRXBUF1CR0` writer"]
pub type W = crate::W<Ahbrxbuf1cr0Spec>;
#[doc = "Field `BUFSZ` reader - AHB RX Buffer Size in 64 bits."]
pub type BufszR = crate::FieldReader<u16>;
#[doc = "Field `BUFSZ` writer - AHB RX Buffer Size in 64 bits."]
pub type BufszW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MSTRID` reader - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
pub type MstridR = crate::FieldReader;
#[doc = "Field `MSTRID` writer - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
pub type MstridW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY` reader - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PREFETCHEN` reader - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
pub type PrefetchenR = crate::BitReader;
#[doc = "Field `PREFETCHEN` writer - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
pub type PrefetchenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn bufsz(&self) -> BufszR {
        BufszR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn mstrid(&self) -> MstridR {
        MstridR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn prefetchen(&self) -> PrefetchenR {
        PrefetchenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRXBUF1CR0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn bufsz(&mut self) -> BufszW<Ahbrxbuf1cr0Spec> {
        BufszW::new(self, 0)
    }
    #[doc = "Bits 16:19 - This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn mstrid(&mut self) -> MstridW<Ahbrxbuf1cr0Spec> {
        MstridW::new(self, 16)
    }
    #[doc = "Bits 24:26 - This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn priority(&mut self) -> PriorityW<Ahbrxbuf1cr0Spec> {
        PriorityW::new(self, 24)
    }
    #[doc = "Bit 31 - AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn prefetchen(&mut self) -> PrefetchenW<Ahbrxbuf1cr0Spec> {
        PrefetchenW::new(self, 31)
    }
}
#[doc = "AHB RX Buffer 1 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrxbuf1cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrxbuf1cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbrxbuf1cr0Spec;
impl crate::RegisterSpec for Ahbrxbuf1cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrxbuf1cr0::R`](R) reader structure"]
impl crate::Readable for Ahbrxbuf1cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbrxbuf1cr0::W`](W) writer structure"]
impl crate::Writable for Ahbrxbuf1cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRXBUF1CR0 to value 0x8001_0020"]
impl crate::Resettable for Ahbrxbuf1cr0Spec {
    const RESET_VALUE: u32 = 0x8001_0020;
}
