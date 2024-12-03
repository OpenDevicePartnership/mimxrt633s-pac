#[doc = "Register `AHBCR` reader"]
pub type R = crate::R<AhbcrSpec>;
#[doc = "Register `AHBCR` writer"]
pub type W = crate::W<AhbcrSpec>;
#[doc = "Parallel mode enabled for AHB triggered Command (both read and write) .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aparen {
    #[doc = "0: Flash will be accessed in Individual mode."]
    Aparen0 = 0,
    #[doc = "1: Flash will be accessed in Parallel mode."]
    Aparen1 = 1,
}
impl From<Aparen> for bool {
    #[inline(always)]
    fn from(variant: Aparen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APAREN` reader - Parallel mode enabled for AHB triggered Command (both read and write) ."]
pub type AparenR = crate::BitReader<Aparen>;
impl AparenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aparen {
        match self.bits {
            false => Aparen::Aparen0,
            true => Aparen::Aparen1,
        }
    }
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn is_aparen_0(&self) -> bool {
        *self == Aparen::Aparen0
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn is_aparen_1(&self) -> bool {
        *self == Aparen::Aparen1
    }
}
#[doc = "Field `APAREN` writer - Parallel mode enabled for AHB triggered Command (both read and write) ."]
pub type AparenW<'a, REG> = crate::BitWriter<'a, REG, Aparen>;
impl<'a, REG> AparenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn aparen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aparen::Aparen0)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn aparen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aparen::Aparen1)
    }
}
#[doc = "Enable AHB bus cachable read access support.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cachableen {
    #[doc = "0: Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    Cachableen0 = 0,
    #[doc = "1: Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    Cachableen1 = 1,
}
impl From<Cachableen> for bool {
    #[inline(always)]
    fn from(variant: Cachableen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHABLEEN` reader - Enable AHB bus cachable read access support."]
pub type CachableenR = crate::BitReader<Cachableen>;
impl CachableenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cachableen {
        match self.bits {
            false => Cachableen::Cachableen0,
            true => Cachableen::Cachableen1,
        }
    }
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    #[inline(always)]
    pub fn is_cachableen_0(&self) -> bool {
        *self == Cachableen::Cachableen0
    }
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    #[inline(always)]
    pub fn is_cachableen_1(&self) -> bool {
        *self == Cachableen::Cachableen1
    }
}
#[doc = "Field `CACHABLEEN` writer - Enable AHB bus cachable read access support."]
pub type CachableenW<'a, REG> = crate::BitWriter<'a, REG, Cachableen>;
impl<'a, REG> CachableenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    #[inline(always)]
    pub fn cachableen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cachableen::Cachableen0)
    }
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    #[inline(always)]
    pub fn cachableen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cachableen::Cachableen1)
    }
}
#[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufferableen {
    #[doc = "0: Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    Bufferableen0 = 0,
    #[doc = "1: Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    Bufferableen1 = 1,
}
impl From<Bufferableen> for bool {
    #[inline(always)]
    fn from(variant: Bufferableen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFERABLEEN` reader - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
pub type BufferableenR = crate::BitReader<Bufferableen>;
impl BufferableenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufferableen {
        match self.bits {
            false => Bufferableen::Bufferableen0,
            true => Bufferableen::Bufferableen1,
        }
    }
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    #[inline(always)]
    pub fn is_bufferableen_0(&self) -> bool {
        *self == Bufferableen::Bufferableen0
    }
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    #[inline(always)]
    pub fn is_bufferableen_1(&self) -> bool {
        *self == Bufferableen::Bufferableen1
    }
}
#[doc = "Field `BUFFERABLEEN` writer - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
pub type BufferableenW<'a, REG> = crate::BitWriter<'a, REG, Bufferableen>;
impl<'a, REG> BufferableenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    #[inline(always)]
    pub fn bufferableen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferableen::Bufferableen0)
    }
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    #[inline(always)]
    pub fn bufferableen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufferableen::Bufferableen1)
    }
}
#[doc = "Field `PREFETCHEN` reader - AHB Read Prefetch Enable."]
pub type PrefetchenR = crate::BitReader;
#[doc = "Field `PREFETCHEN` writer - AHB Read Prefetch Enable."]
pub type PrefetchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readaddropt {
    #[doc = "0: There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    Readaddropt0 = 0,
    #[doc = "1: There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
    Readaddropt1 = 1,
}
impl From<Readaddropt> for bool {
    #[inline(always)]
    fn from(variant: Readaddropt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READADDROPT` reader - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
pub type ReadaddroptR = crate::BitReader<Readaddropt>;
impl ReadaddroptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readaddropt {
        match self.bits {
            false => Readaddropt::Readaddropt0,
            true => Readaddropt::Readaddropt1,
        }
    }
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    #[inline(always)]
    pub fn is_readaddropt_0(&self) -> bool {
        *self == Readaddropt::Readaddropt0
    }
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
    #[inline(always)]
    pub fn is_readaddropt_1(&self) -> bool {
        *self == Readaddropt::Readaddropt1
    }
}
#[doc = "Field `READADDROPT` writer - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
pub type ReadaddroptW<'a, REG> = crate::BitWriter<'a, REG, Readaddropt>;
impl<'a, REG> ReadaddroptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    #[inline(always)]
    pub fn readaddropt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Readaddropt::Readaddropt0)
    }
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more data than AHB burst required to meet the alignment requirement."]
    #[inline(always)]
    pub fn readaddropt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Readaddropt::Readaddropt1)
    }
}
impl R {
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub fn aparen(&self) -> AparenR {
        AparenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub fn cachableen(&self) -> CachableenR {
        CachableenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub fn bufferableen(&self) -> BufferableenR {
        BufferableenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline(always)]
    pub fn prefetchen(&self) -> PrefetchenR {
        PrefetchenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub fn readaddropt(&self) -> ReadaddroptR {
        ReadaddroptR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBCR")
            .field("aparen", &self.aparen())
            .field("cachableen", &self.cachableen())
            .field("bufferableen", &self.bufferableen())
            .field("prefetchen", &self.prefetchen())
            .field("readaddropt", &self.readaddropt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub fn aparen(&mut self) -> AparenW<AhbcrSpec> {
        AparenW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub fn cachableen(&mut self) -> CachableenW<AhbcrSpec> {
        CachableenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub fn bufferableen(&mut self) -> BufferableenW<AhbcrSpec> {
        BufferableenW::new(self, 4)
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline(always)]
    pub fn prefetchen(&mut self) -> PrefetchenW<AhbcrSpec> {
        PrefetchenW::new(self, 5)
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub fn readaddropt(&mut self) -> ReadaddroptW<AhbcrSpec> {
        ReadaddroptW::new(self, 6)
    }
}
#[doc = "AHB Bus Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbcrSpec;
impl crate::RegisterSpec for AhbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbcr::R`](R) reader structure"]
impl crate::Readable for AhbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbcr::W`](W) writer structure"]
impl crate::Writable for AhbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBCR to value 0x18"]
impl crate::Resettable for AhbcrSpec {
    const RESET_VALUE: u32 = 0x18;
}
