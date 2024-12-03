#[doc = "Register `BLK_ATT` reader"]
pub type R = crate::R<BlkAttSpec>;
#[doc = "Register `BLK_ATT` writer"]
pub type W = crate::W<BlkAttSpec>;
#[doc = "Block Size\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Blksize {
    #[doc = "0: No data transfer"]
    Blksize0 = 0,
    #[doc = "1: 1 Byte"]
    Blksize1 = 1,
    #[doc = "2: 2 Bytes"]
    Blksize2 = 2,
    #[doc = "3: 3 Bytes"]
    Blksize3 = 3,
    #[doc = "4: 4 Bytes"]
    Blksize4 = 4,
    #[doc = "511: 511 Bytes"]
    Blksize511 = 511,
    #[doc = "512: 512 Bytes"]
    Blksize512 = 512,
    #[doc = "2048: 2048 Bytes"]
    Blksize2048 = 2048,
    #[doc = "4096: 4096 Bytes"]
    Blksize4096 = 4096,
}
impl From<Blksize> for u16 {
    #[inline(always)]
    fn from(variant: Blksize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blksize {
    type Ux = u16;
}
impl crate::IsEnum for Blksize {}
#[doc = "Field `BLKSIZE` reader - Block Size"]
pub type BlksizeR = crate::FieldReader<Blksize>;
impl BlksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blksize> {
        match self.bits {
            0 => Some(Blksize::Blksize0),
            1 => Some(Blksize::Blksize1),
            2 => Some(Blksize::Blksize2),
            3 => Some(Blksize::Blksize3),
            4 => Some(Blksize::Blksize4),
            511 => Some(Blksize::Blksize511),
            512 => Some(Blksize::Blksize512),
            2048 => Some(Blksize::Blksize2048),
            4096 => Some(Blksize::Blksize4096),
            _ => None,
        }
    }
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn is_blksize_0(&self) -> bool {
        *self == Blksize::Blksize0
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn is_blksize_1(&self) -> bool {
        *self == Blksize::Blksize1
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn is_blksize_2(&self) -> bool {
        *self == Blksize::Blksize2
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn is_blksize_3(&self) -> bool {
        *self == Blksize::Blksize3
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn is_blksize_4(&self) -> bool {
        *self == Blksize::Blksize4
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn is_blksize_511(&self) -> bool {
        *self == Blksize::Blksize511
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn is_blksize_512(&self) -> bool {
        *self == Blksize::Blksize512
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn is_blksize_2048(&self) -> bool {
        *self == Blksize::Blksize2048
    }
    #[doc = "4096 Bytes"]
    #[inline(always)]
    pub fn is_blksize_4096(&self) -> bool {
        *self == Blksize::Blksize4096
    }
}
#[doc = "Field `BLKSIZE` writer - Block Size"]
pub type BlksizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, Blksize>;
impl<'a, REG> BlksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn blksize_0(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize0)
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn blksize_1(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize1)
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn blksize_2(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize2)
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn blksize_3(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize3)
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn blksize_4(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize4)
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn blksize_511(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize511)
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn blksize_512(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize512)
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn blksize_2048(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize2048)
    }
    #[doc = "4096 Bytes"]
    #[inline(always)]
    pub fn blksize_4096(self) -> &'a mut crate::W<REG> {
        self.variant(Blksize::Blksize4096)
    }
}
#[doc = "Block Count\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Blkcnt {
    #[doc = "0: Stop Count"]
    Blkcnt0 = 0,
    #[doc = "1: 1 block"]
    Blkcnt1 = 1,
    #[doc = "2: 2 blocks"]
    Blkcnt2 = 2,
    #[doc = "65535: 65535 blocks"]
    Blkcnt65535 = 65535,
}
impl From<Blkcnt> for u16 {
    #[inline(always)]
    fn from(variant: Blkcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blkcnt {
    type Ux = u16;
}
impl crate::IsEnum for Blkcnt {}
#[doc = "Field `BLKCNT` reader - Block Count"]
pub type BlkcntR = crate::FieldReader<Blkcnt>;
impl BlkcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Blkcnt> {
        match self.bits {
            0 => Some(Blkcnt::Blkcnt0),
            1 => Some(Blkcnt::Blkcnt1),
            2 => Some(Blkcnt::Blkcnt2),
            65535 => Some(Blkcnt::Blkcnt65535),
            _ => None,
        }
    }
    #[doc = "Stop Count"]
    #[inline(always)]
    pub fn is_blkcnt_0(&self) -> bool {
        *self == Blkcnt::Blkcnt0
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn is_blkcnt_1(&self) -> bool {
        *self == Blkcnt::Blkcnt1
    }
    #[doc = "2 blocks"]
    #[inline(always)]
    pub fn is_blkcnt_2(&self) -> bool {
        *self == Blkcnt::Blkcnt2
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn is_blkcnt_65535(&self) -> bool {
        *self == Blkcnt::Blkcnt65535
    }
}
#[doc = "Field `BLKCNT` writer - Block Count"]
pub type BlkcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, Blkcnt>;
impl<'a, REG> BlkcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Stop Count"]
    #[inline(always)]
    pub fn blkcnt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::Blkcnt0)
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn blkcnt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::Blkcnt1)
    }
    #[doc = "2 blocks"]
    #[inline(always)]
    pub fn blkcnt_2(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::Blkcnt2)
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn blkcnt_65535(self) -> &'a mut crate::W<REG> {
        self.variant(Blkcnt::Blkcnt65535)
    }
}
impl R {
    #[doc = "Bits 0:12 - Block Size"]
    #[inline(always)]
    pub fn blksize(&self) -> BlksizeR {
        BlksizeR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - Block Count"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BlkcntR {
        BlkcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK_ATT")
            .field("blksize", &self.blksize())
            .field("blkcnt", &self.blkcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Block Size"]
    #[inline(always)]
    pub fn blksize(&mut self) -> BlksizeW<BlkAttSpec> {
        BlksizeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Block Count"]
    #[inline(always)]
    pub fn blkcnt(&mut self) -> BlkcntW<BlkAttSpec> {
        BlkcntW::new(self, 16)
    }
}
#[doc = "Block Attributes\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_att::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_att::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkAttSpec;
impl crate::RegisterSpec for BlkAttSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_att::R`](R) reader structure"]
impl crate::Readable for BlkAttSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_att::W`](W) writer structure"]
impl crate::Writable for BlkAttSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK_ATT to value 0x0001_0000"]
impl crate::Resettable for BlkAttSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
