#[doc = "Register `MDMACTRL` reader"]
pub type R = crate::R<MdmactrlSpec>;
#[doc = "Register `MDMACTRL` writer"]
pub type W = crate::W<MdmactrlSpec>;
#[doc = "DMA from bus\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmafb {
    #[doc = "0: NOT_USED: DMA is not used"]
    NotUsed = 0,
    #[doc = "1: ENABLE_ONE_FRAME: DMA is enabled for 1 frame. DMAFB auto-clears on STOP or repeated START. See MCONFIG.MATCHSS."]
    EnableOneFrame = 1,
    #[doc = "2: ENABLE: DMA is enabled until the DMA is turned off."]
    Enable = 2,
}
impl From<Dmafb> for u8 {
    #[inline(always)]
    fn from(variant: Dmafb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmafb {
    type Ux = u8;
}
impl crate::IsEnum for Dmafb {}
#[doc = "Field `DMAFB` reader - DMA from bus"]
pub type DmafbR = crate::FieldReader<Dmafb>;
impl DmafbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmafb> {
        match self.bits {
            0 => Some(Dmafb::NotUsed),
            1 => Some(Dmafb::EnableOneFrame),
            2 => Some(Dmafb::Enable),
            _ => None,
        }
    }
    #[doc = "NOT_USED: DMA is not used"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Dmafb::NotUsed
    }
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 frame. DMAFB auto-clears on STOP or repeated START. See MCONFIG.MATCHSS."]
    #[inline(always)]
    pub fn is_enable_one_frame(&self) -> bool {
        *self == Dmafb::EnableOneFrame
    }
    #[doc = "ENABLE: DMA is enabled until the DMA is turned off."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmafb::Enable
    }
}
#[doc = "Field `DMAFB` writer - DMA from bus"]
pub type DmafbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmafb>;
impl<'a, REG> DmafbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOT_USED: DMA is not used"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Dmafb::NotUsed)
    }
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 frame. DMAFB auto-clears on STOP or repeated START. See MCONFIG.MATCHSS."]
    #[inline(always)]
    pub fn enable_one_frame(self) -> &'a mut crate::W<REG> {
        self.variant(Dmafb::EnableOneFrame)
    }
    #[doc = "ENABLE: DMA is enabled until the DMA is turned off."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmafb::Enable)
    }
}
#[doc = "DMA to bus\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmatb {
    #[doc = "0: NOT_USED: DMA is not used"]
    NotUsed = 0,
    #[doc = "1: ENABLE_ONE_FRAME: DMA is enabled for 1 frame (ended by DMA or Terminated). DMATB auto-clears on STOP or START. See MCONFIG.MATCHSS."]
    EnableOneFrame = 1,
    #[doc = "2: ENABLE: DMA is enabled until DMA is turned off. Normally DMA ENABLE should only be used in Master Message mode."]
    Enable = 2,
}
impl From<Dmatb> for u8 {
    #[inline(always)]
    fn from(variant: Dmatb) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmatb {
    type Ux = u8;
}
impl crate::IsEnum for Dmatb {}
#[doc = "Field `DMATB` reader - DMA to bus"]
pub type DmatbR = crate::FieldReader<Dmatb>;
impl DmatbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmatb> {
        match self.bits {
            0 => Some(Dmatb::NotUsed),
            1 => Some(Dmatb::EnableOneFrame),
            2 => Some(Dmatb::Enable),
            _ => None,
        }
    }
    #[doc = "NOT_USED: DMA is not used"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == Dmatb::NotUsed
    }
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 frame (ended by DMA or Terminated). DMATB auto-clears on STOP or START. See MCONFIG.MATCHSS."]
    #[inline(always)]
    pub fn is_enable_one_frame(&self) -> bool {
        *self == Dmatb::EnableOneFrame
    }
    #[doc = "ENABLE: DMA is enabled until DMA is turned off. Normally DMA ENABLE should only be used in Master Message mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmatb::Enable
    }
}
#[doc = "Field `DMATB` writer - DMA to bus"]
pub type DmatbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmatb>;
impl<'a, REG> DmatbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOT_USED: DMA is not used"]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatb::NotUsed)
    }
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 frame (ended by DMA or Terminated). DMATB auto-clears on STOP or START. See MCONFIG.MATCHSS."]
    #[inline(always)]
    pub fn enable_one_frame(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatb::EnableOneFrame)
    }
    #[doc = "ENABLE: DMA is enabled until DMA is turned off. Normally DMA ENABLE should only be used in Master Message mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmatb::Enable)
    }
}
#[doc = "DMA width\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmawidth {
    #[doc = "0: BYTE"]
    Byte = 0,
    #[doc = "1: BYTE_AGAIN"]
    ByteAgain = 1,
    #[doc = "2: HALF_WORD: Half-word (16 bits). This will make sure that 2 bytes are free/available in FIFO."]
    HalfWord = 2,
}
impl From<Dmawidth> for u8 {
    #[inline(always)]
    fn from(variant: Dmawidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmawidth {
    type Ux = u8;
}
impl crate::IsEnum for Dmawidth {}
#[doc = "Field `DMAWIDTH` reader - DMA width"]
pub type DmawidthR = crate::FieldReader<Dmawidth>;
impl DmawidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmawidth> {
        match self.bits {
            0 => Some(Dmawidth::Byte),
            1 => Some(Dmawidth::ByteAgain),
            2 => Some(Dmawidth::HalfWord),
            _ => None,
        }
    }
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Dmawidth::Byte
    }
    #[doc = "BYTE_AGAIN"]
    #[inline(always)]
    pub fn is_byte_again(&self) -> bool {
        *self == Dmawidth::ByteAgain
    }
    #[doc = "HALF_WORD: Half-word (16 bits). This will make sure that 2 bytes are free/available in FIFO."]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == Dmawidth::HalfWord
    }
}
#[doc = "Field `DMAWIDTH` writer - DMA width"]
pub type DmawidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmawidth>;
impl<'a, REG> DmawidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawidth::Byte)
    }
    #[doc = "BYTE_AGAIN"]
    #[inline(always)]
    pub fn byte_again(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawidth::ByteAgain)
    }
    #[doc = "HALF_WORD: Half-word (16 bits). This will make sure that 2 bytes are free/available in FIFO."]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawidth::HalfWord)
    }
}
impl R {
    #[doc = "Bits 0:1 - DMA from bus"]
    #[inline(always)]
    pub fn dmafb(&self) -> DmafbR {
        DmafbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DMA to bus"]
    #[inline(always)]
    pub fn dmatb(&self) -> DmatbR {
        DmatbR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DMA width"]
    #[inline(always)]
    pub fn dmawidth(&self) -> DmawidthR {
        DmawidthR::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDMACTRL")
            .field("dmafb", &self.dmafb())
            .field("dmatb", &self.dmatb())
            .field("dmawidth", &self.dmawidth())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA from bus"]
    #[inline(always)]
    pub fn dmafb(&mut self) -> DmafbW<MdmactrlSpec> {
        DmafbW::new(self, 0)
    }
    #[doc = "Bits 2:3 - DMA to bus"]
    #[inline(always)]
    pub fn dmatb(&mut self) -> DmatbW<MdmactrlSpec> {
        DmatbW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DMA width"]
    #[inline(always)]
    pub fn dmawidth(&mut self) -> DmawidthW<MdmactrlSpec> {
        DmawidthW::new(self, 4)
    }
}
#[doc = "Master DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmactrlSpec;
impl crate::RegisterSpec for MdmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmactrl::R`](R) reader structure"]
impl crate::Readable for MdmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mdmactrl::W`](W) writer structure"]
impl crate::Writable for MdmactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMACTRL to value 0x10"]
impl crate::Resettable for MdmactrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
