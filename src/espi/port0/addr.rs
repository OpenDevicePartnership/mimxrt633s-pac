#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `OFF` reader - Offset from 0 or the selected mapped base for matching memory or IO"]
pub type OffR = crate::FieldReader<u16>;
#[doc = "Field `OFF` writer - Offset from 0 or the selected mapped base for matching memory or IO"]
pub type OffW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "The meaning is dependent on type of port:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BaseOrAsz {
    #[doc = "0: Is offset from 0 in host memory or IO space."]
    OffsetFrom0 = 0,
    #[doc = "1: Uses BASE0 offset in host memory"]
    UseBase0 = 1,
    #[doc = "2: Uses BASE1 offset in host memory"]
    UseBase1 = 2,
}
impl From<BaseOrAsz> for u8 {
    #[inline(always)]
    fn from(variant: BaseOrAsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BaseOrAsz {
    type Ux = u8;
}
impl crate::IsEnum for BaseOrAsz {}
#[doc = "Field `BASE_or_ASZ` reader - The meaning is dependent on type of port:"]
pub type BaseOrAszR = crate::FieldReader<BaseOrAsz>;
impl BaseOrAszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BaseOrAsz> {
        match self.bits {
            0 => Some(BaseOrAsz::OffsetFrom0),
            1 => Some(BaseOrAsz::UseBase0),
            2 => Some(BaseOrAsz::UseBase1),
            _ => None,
        }
    }
    #[doc = "Is offset from 0 in host memory or IO space."]
    #[inline(always)]
    pub fn is_offset_from_0(&self) -> bool {
        *self == BaseOrAsz::OffsetFrom0
    }
    #[doc = "Uses BASE0 offset in host memory"]
    #[inline(always)]
    pub fn is_use_base0(&self) -> bool {
        *self == BaseOrAsz::UseBase0
    }
    #[doc = "Uses BASE1 offset in host memory"]
    #[inline(always)]
    pub fn is_use_base1(&self) -> bool {
        *self == BaseOrAsz::UseBase1
    }
}
#[doc = "Field `BASE_or_ASZ` writer - The meaning is dependent on type of port:"]
pub type BaseOrAszW<'a, REG> = crate::FieldWriter<'a, REG, 2, BaseOrAsz>;
impl<'a, REG> BaseOrAszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Is offset from 0 in host memory or IO space."]
    #[inline(always)]
    pub fn offset_from_0(self) -> &'a mut crate::W<REG> {
        self.variant(BaseOrAsz::OffsetFrom0)
    }
    #[doc = "Uses BASE0 offset in host memory"]
    #[inline(always)]
    pub fn use_base0(self) -> &'a mut crate::W<REG> {
        self.variant(BaseOrAsz::UseBase0)
    }
    #[doc = "Uses BASE1 offset in host memory"]
    #[inline(always)]
    pub fn use_base1(self) -> &'a mut crate::W<REG> {
        self.variant(BaseOrAsz::UseBase1)
    }
}
#[doc = "Field `IDXOFF` reader - For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
pub type IdxoffR = crate::FieldReader;
#[doc = "Field `IDXOFF` writer - For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
pub type IdxoffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDX1ST` reader - For index/data register only: Is 1 if index is lower address than data (e"]
pub type Idx1stR = crate::BitReader;
#[doc = "Field `IDX1ST` writer - For index/data register only: Is 1 if index is lower address than data (e"]
pub type Idx1stW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub fn off(&self) -> OffR {
        OffR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - The meaning is dependent on type of port:"]
    #[inline(always)]
    pub fn base_or_asz(&self) -> BaseOrAszR {
        BaseOrAszR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub fn idxoff(&self) -> IdxoffR {
        IdxoffR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub fn idx1st(&self) -> Idx1stR {
        Idx1stR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR")
            .field("off", &self.off())
            .field("base_or_asz", &self.base_or_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub fn off(&mut self) -> OffW<AddrSpec> {
        OffW::new(self, 0)
    }
    #[doc = "Bits 16:17 - The meaning is dependent on type of port:"]
    #[inline(always)]
    pub fn base_or_asz(&mut self) -> BaseOrAszW<AddrSpec> {
        BaseOrAszW::new(self, 16)
    }
    #[doc = "Bits 24:27 - For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub fn idxoff(&mut self) -> IdxoffW<AddrSpec> {
        IdxoffW::new(self, 24)
    }
    #[doc = "Bit 28 - For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub fn idx1st(&mut self) -> Idx1stW<AddrSpec> {
        Idx1stW::new(self, 28)
    }
}
#[doc = "Port address offset to host\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
