#[doc = "Register `CLCR` reader"]
pub type R = crate::R<ClcrSpec>;
#[doc = "Register `CLCR` writer"]
pub type W = crate::W<ClcrSpec>;
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lgo {
    #[doc = "0: Write: no effect. Read: no line command active."]
    NoEffect = 0,
    #[doc = "1: Write: initiate line command indicated by bits 27-24. Read: line command active."]
    InitCmd = 1,
}
impl From<Lgo> for bool {
    #[inline(always)]
    fn from(variant: Lgo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LGO` reader - Initiate Cache Line Command"]
pub type LgoR = crate::BitReader<Lgo>;
impl LgoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lgo {
        match self.bits {
            false => Lgo::NoEffect,
            true => Lgo::InitCmd,
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Lgo::NoEffect
    }
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == Lgo::InitCmd
    }
}
#[doc = "Field `LGO` writer - Initiate Cache Line Command"]
pub type LgoW<'a, REG> = crate::BitWriter<'a, REG, Lgo>;
impl<'a, REG> LgoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lgo::NoEffect)
    }
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Lgo::InitCmd)
    }
}
#[doc = "Field `CACHEADDR` reader - Cache address"]
pub type CacheaddrR = crate::FieldReader<u16>;
#[doc = "Field `CACHEADDR` writer - Cache address"]
pub type CacheaddrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Way select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wsel {
    #[doc = "0: Way 0"]
    Way0 = 0,
    #[doc = "1: Way 1"]
    Way1 = 1,
}
impl From<Wsel> for bool {
    #[inline(always)]
    fn from(variant: Wsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSEL` reader - Way select"]
pub type WselR = crate::BitReader<Wsel>;
impl WselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wsel {
        match self.bits {
            false => Wsel::Way0,
            true => Wsel::Way1,
        }
    }
    #[doc = "Way 0"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        *self == Wsel::Way0
    }
    #[doc = "Way 1"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        *self == Wsel::Way1
    }
}
#[doc = "Field `WSEL` writer - Way select"]
pub type WselW<'a, REG> = crate::BitWriter<'a, REG, Wsel>;
impl<'a, REG> WselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Way 0"]
    #[inline(always)]
    pub fn way0(self) -> &'a mut crate::W<REG> {
        self.variant(Wsel::Way0)
    }
    #[doc = "Way 1"]
    #[inline(always)]
    pub fn way1(self) -> &'a mut crate::W<REG> {
        self.variant(Wsel::Way1)
    }
}
#[doc = "Tag/Data Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdsel {
    #[doc = "0: Data"]
    Data = 0,
    #[doc = "1: Tag"]
    Tag = 1,
}
impl From<Tdsel> for bool {
    #[inline(always)]
    fn from(variant: Tdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDSEL` reader - Tag/Data Select"]
pub type TdselR = crate::BitReader<Tdsel>;
impl TdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdsel {
        match self.bits {
            false => Tdsel::Data,
            true => Tdsel::Tag,
        }
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Tdsel::Data
    }
    #[doc = "Tag"]
    #[inline(always)]
    pub fn is_tag(&self) -> bool {
        *self == Tdsel::Tag
    }
}
#[doc = "Field `TDSEL` writer - Tag/Data Select"]
pub type TdselW<'a, REG> = crate::BitWriter<'a, REG, Tdsel>;
impl<'a, REG> TdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Tdsel::Data)
    }
    #[doc = "Tag"]
    #[inline(always)]
    pub fn tag(self) -> &'a mut crate::W<REG> {
        self.variant(Tdsel::Tag)
    }
}
#[doc = "Field `LCIVB` reader - Line Command Initial Valid Bit"]
pub type LcivbR = crate::BitReader;
#[doc = "Field `LCIVB` writer - Line Command Initial Valid Bit"]
pub type LcivbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCIMB` reader - Line Command Initial Modified Bit"]
pub type LcimbR = crate::BitReader;
#[doc = "Field `LCIMB` writer - Line Command Initial Modified Bit"]
pub type LcimbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCWAY` reader - Line Command Way"]
pub type LcwayR = crate::BitReader;
#[doc = "Field `LCWAY` writer - Line Command Way"]
pub type LcwayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Line Command\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcmd {
    #[doc = "0: Search and read or write"]
    SearchRw = 0,
    #[doc = "1: Invalidate"]
    Invalidate = 1,
    #[doc = "2: Push"]
    Push = 2,
    #[doc = "3: Clear"]
    Clear = 3,
}
impl From<Lcmd> for u8 {
    #[inline(always)]
    fn from(variant: Lcmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcmd {
    type Ux = u8;
}
impl crate::IsEnum for Lcmd {}
#[doc = "Field `LCMD` reader - Line Command"]
pub type LcmdR = crate::FieldReader<Lcmd>;
impl LcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcmd {
        match self.bits {
            0 => Lcmd::SearchRw,
            1 => Lcmd::Invalidate,
            2 => Lcmd::Push,
            3 => Lcmd::Clear,
            _ => unreachable!(),
        }
    }
    #[doc = "Search and read or write"]
    #[inline(always)]
    pub fn is_search_rw(&self) -> bool {
        *self == Lcmd::SearchRw
    }
    #[doc = "Invalidate"]
    #[inline(always)]
    pub fn is_invalidate(&self) -> bool {
        *self == Lcmd::Invalidate
    }
    #[doc = "Push"]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == Lcmd::Push
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Lcmd::Clear
    }
}
#[doc = "Field `LCMD` writer - Line Command"]
pub type LcmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lcmd, crate::Safe>;
impl<'a, REG> LcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Search and read or write"]
    #[inline(always)]
    pub fn search_rw(self) -> &'a mut crate::W<REG> {
        self.variant(Lcmd::SearchRw)
    }
    #[doc = "Invalidate"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut crate::W<REG> {
        self.variant(Lcmd::Invalidate)
    }
    #[doc = "Push"]
    #[inline(always)]
    pub fn push(self) -> &'a mut crate::W<REG> {
        self.variant(Lcmd::Push)
    }
    #[doc = "Clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Lcmd::Clear)
    }
}
#[doc = "Line Address Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ladsel {
    #[doc = "0: Cache address"]
    CacheAddr = 0,
    #[doc = "1: Physical address"]
    PhysAddr = 1,
}
impl From<Ladsel> for bool {
    #[inline(always)]
    fn from(variant: Ladsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LADSEL` reader - Line Address Select"]
pub type LadselR = crate::BitReader<Ladsel>;
impl LadselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ladsel {
        match self.bits {
            false => Ladsel::CacheAddr,
            true => Ladsel::PhysAddr,
        }
    }
    #[doc = "Cache address"]
    #[inline(always)]
    pub fn is_cache_addr(&self) -> bool {
        *self == Ladsel::CacheAddr
    }
    #[doc = "Physical address"]
    #[inline(always)]
    pub fn is_phys_addr(&self) -> bool {
        *self == Ladsel::PhysAddr
    }
}
#[doc = "Field `LADSEL` writer - Line Address Select"]
pub type LadselW<'a, REG> = crate::BitWriter<'a, REG, Ladsel>;
impl<'a, REG> LadselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache address"]
    #[inline(always)]
    pub fn cache_addr(self) -> &'a mut crate::W<REG> {
        self.variant(Ladsel::CacheAddr)
    }
    #[doc = "Physical address"]
    #[inline(always)]
    pub fn phys_addr(self) -> &'a mut crate::W<REG> {
        self.variant(Ladsel::PhysAddr)
    }
}
#[doc = "Line access type\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lacc {
    #[doc = "0: Read"]
    Read = 0,
    #[doc = "1: Write"]
    Write = 1,
}
impl From<Lacc> for bool {
    #[inline(always)]
    fn from(variant: Lacc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LACC` reader - Line access type"]
pub type LaccR = crate::BitReader<Lacc>;
impl LaccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lacc {
        match self.bits {
            false => Lacc::Read,
            true => Lacc::Write,
        }
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Lacc::Read
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Lacc::Write
    }
}
#[doc = "Field `LACC` writer - Line access type"]
pub type LaccW<'a, REG> = crate::BitWriter<'a, REG, Lacc>;
impl<'a, REG> LaccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Lacc::Read)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Lacc::Write)
    }
}
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LgoR {
        LgoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:13 - Cache address"]
    #[inline(always)]
    pub fn cacheaddr(&self) -> CacheaddrR {
        CacheaddrR::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - Way select"]
    #[inline(always)]
    pub fn wsel(&self) -> WselR {
        WselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline(always)]
    pub fn tdsel(&self) -> TdselR {
        TdselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline(always)]
    pub fn lcivb(&self) -> LcivbR {
        LcivbR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline(always)]
    pub fn lcimb(&self) -> LcimbR {
        LcimbR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline(always)]
    pub fn lcway(&self) -> LcwayR {
        LcwayR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline(always)]
    pub fn lcmd(&self) -> LcmdR {
        LcmdR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline(always)]
    pub fn ladsel(&self) -> LadselR {
        LadselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline(always)]
    pub fn lacc(&self) -> LaccR {
        LaccR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLCR")
            .field("lgo", &self.lgo())
            .field("cacheaddr", &self.cacheaddr())
            .field("wsel", &self.wsel())
            .field("tdsel", &self.tdsel())
            .field("lcivb", &self.lcivb())
            .field("lcimb", &self.lcimb())
            .field("lcway", &self.lcway())
            .field("lcmd", &self.lcmd())
            .field("ladsel", &self.ladsel())
            .field("lacc", &self.lacc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    #[must_use]
    pub fn lgo(&mut self) -> LgoW<ClcrSpec> {
        LgoW::new(self, 0)
    }
    #[doc = "Bits 2:13 - Cache address"]
    #[inline(always)]
    #[must_use]
    pub fn cacheaddr(&mut self) -> CacheaddrW<ClcrSpec> {
        CacheaddrW::new(self, 2)
    }
    #[doc = "Bit 14 - Way select"]
    #[inline(always)]
    #[must_use]
    pub fn wsel(&mut self) -> WselW<ClcrSpec> {
        WselW::new(self, 14)
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdsel(&mut self) -> TdselW<ClcrSpec> {
        TdselW::new(self, 16)
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lcivb(&mut self) -> LcivbW<ClcrSpec> {
        LcivbW::new(self, 20)
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lcimb(&mut self) -> LcimbW<ClcrSpec> {
        LcimbW::new(self, 21)
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline(always)]
    #[must_use]
    pub fn lcway(&mut self) -> LcwayW<ClcrSpec> {
        LcwayW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline(always)]
    #[must_use]
    pub fn lcmd(&mut self) -> LcmdW<ClcrSpec> {
        LcmdW::new(self, 24)
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline(always)]
    #[must_use]
    pub fn ladsel(&mut self) -> LadselW<ClcrSpec> {
        LadselW::new(self, 26)
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline(always)]
    #[must_use]
    pub fn lacc(&mut self) -> LaccW<ClcrSpec> {
        LaccW::new(self, 27)
    }
}
#[doc = "Cache line control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClcrSpec;
impl crate::RegisterSpec for ClcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clcr::R`](R) reader structure"]
impl crate::Readable for ClcrSpec {}
#[doc = "`write(|w| ..)` method takes [`clcr::W`](W) writer structure"]
impl crate::Writable for ClcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLCR to value 0"]
impl crate::Resettable for ClcrSpec {
    const RESET_VALUE: u32 = 0;
}
