#[doc = "Register `MEMCTRL` reader"]
pub type R = crate::R<MemctrlSpec>;
#[doc = "Register `MEMCTRL` writer"]
pub type W = crate::W<MemctrlSpec>;
#[doc = "Enables mastering.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Master {
    #[doc = "0: Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    NotUsed = 0,
    #[doc = "1: Mastering is enabled and DMA and INDATA should not be used."]
    Enabled = 1,
}
impl From<Master> for bool {
    #[inline(always)]
    fn from(variant: Master) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - Enables mastering."]
pub type MasterR = crate::BitReader<Master>;
impl MasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Master {
        match self.bits {
            false => Master::NotUsed,
            true => Master::Enabled,
        }
    }
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Master::NotUsed
    }
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Master::Enabled
    }
}
#[doc = "Field `MASTER` writer - Enables mastering."]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG, Master>;
impl<'a, REG> MasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mastering is not used and the normal DMA or Interrupt based model is used with INDATA."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Master::NotUsed)
    }
    #[doc = "Mastering is enabled and DMA and INDATA should not be used."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Master::Enabled)
    }
}
#[doc = "Field `COUNT` reader - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Enables mastering."]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:26 - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEMCTRL")
            .field("master", &self.master())
            .field("count", &self.count())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enables mastering."]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<MemctrlSpec> {
        MasterW::new(self, 0)
    }
    #[doc = "Bits 16:26 - Number of 512-bit (128-bit if AES, except 1st block which may include key and IV) blocks to copy starting at MEMADDR. This register will decrement after each block is copied, ending in 0. For Hash, the DIGEST interrupt will occur when it reaches 0. Fro AES, the DIGEST/OUTDATA interrupt will occur on ever block. If a bus error occurs, it will stop with this field set to the block that failed. 0:Done - nothing to process. 1 to 2K: Number of 512-bit (or 128bit) blocks to hash."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<MemctrlSpec> {
        CountW::new(self, 16)
    }
}
#[doc = "Setup Master to access memory (if available)\n\nYou can [`read`](crate::Reg::read) this register and get [`memctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemctrlSpec;
impl crate::RegisterSpec for MemctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memctrl::R`](R) reader structure"]
impl crate::Readable for MemctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`memctrl::W`](W) writer structure"]
impl crate::Writable for MemctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MemctrlSpec {
    const RESET_VALUE: u32 = 0;
}
