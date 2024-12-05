#[doc = "Register `CMDL%s` reader"]
pub type R = crate::R<CmdlSpec>;
#[doc = "Register `CMDL%s` writer"]
pub type W = crate::W<CmdlSpec>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adch {
    #[doc = "0: Select CH0A or CH0B or CH0A/CH0B pair."]
    Adch0 = 0,
    #[doc = "1: Select CH1A or CH1B or CH1A/CH1B pair."]
    Adch1 = 1,
    #[doc = "2: Select CH2A or CH2B or CH2A/CH2B pair."]
    Adch2 = 2,
    #[doc = "3: Select CH3A or CH3B or CH3A/CH3B pair."]
    Adch3 = 3,
    #[doc = "4: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch4 = 4,
    #[doc = "5: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch5 = 5,
    #[doc = "6: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch6 = 6,
    #[doc = "7: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch7 = 7,
    #[doc = "8: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch8 = 8,
    #[doc = "9: Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    Adch9 = 9,
    #[doc = "30: Select CH30A or CH30B or CH30A/CH30B pair."]
    Adch30 = 30,
    #[doc = "31: Select CH31A or CH31B or CH31A/CH31B pair."]
    Adch31 = 31,
}
impl From<Adch> for u8 {
    #[inline(always)]
    fn from(variant: Adch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adch {
    type Ux = u8;
}
impl crate::IsEnum for Adch {}
#[doc = "Field `ADCH` reader - Input channel select"]
pub type AdchR = crate::FieldReader<Adch>;
impl AdchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adch> {
        match self.bits {
            0 => Some(Adch::Adch0),
            1 => Some(Adch::Adch1),
            2 => Some(Adch::Adch2),
            3 => Some(Adch::Adch3),
            4 => Some(Adch::Adch4),
            5 => Some(Adch::Adch5),
            6 => Some(Adch::Adch6),
            7 => Some(Adch::Adch7),
            8 => Some(Adch::Adch8),
            9 => Some(Adch::Adch9),
            30 => Some(Adch::Adch30),
            31 => Some(Adch::Adch31),
            _ => None,
        }
    }
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn is_adch_0(&self) -> bool {
        *self == Adch::Adch0
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn is_adch_1(&self) -> bool {
        *self == Adch::Adch1
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn is_adch_2(&self) -> bool {
        *self == Adch::Adch2
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn is_adch_3(&self) -> bool {
        *self == Adch::Adch3
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_4(&self) -> bool {
        *self == Adch::Adch4
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_5(&self) -> bool {
        *self == Adch::Adch5
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_6(&self) -> bool {
        *self == Adch::Adch6
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_7(&self) -> bool {
        *self == Adch::Adch7
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_8(&self) -> bool {
        *self == Adch::Adch8
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn is_adch_9(&self) -> bool {
        *self == Adch::Adch9
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn is_adch_30(&self) -> bool {
        *self == Adch::Adch30
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        *self == Adch::Adch31
    }
}
#[doc = "Field `ADCH` writer - Input channel select"]
pub type AdchW<'a, REG> = crate::FieldWriter<'a, REG, 5, Adch>;
impl<'a, REG> AdchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    #[inline(always)]
    pub fn adch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch0)
    }
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    #[inline(always)]
    pub fn adch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch1)
    }
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    #[inline(always)]
    pub fn adch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch2)
    }
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    #[inline(always)]
    pub fn adch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch3)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch4)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch5)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch6)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch7)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch8)
    }
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    #[inline(always)]
    pub fn adch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch9)
    }
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    #[inline(always)]
    pub fn adch_30(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch30)
    }
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::Adch31)
    }
}
#[doc = "A-side vs. B-side Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Absel {
    #[doc = "0: When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    Absel0 = 0,
    #[doc = "1: When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    Absel1 = 1,
}
impl From<Absel> for bool {
    #[inline(always)]
    fn from(variant: Absel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABSEL` reader - A-side vs. B-side Select"]
pub type AbselR = crate::BitReader<Absel>;
impl AbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Absel {
        match self.bits {
            false => Absel::Absel0,
            true => Absel::Absel1,
        }
    }
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    #[inline(always)]
    pub fn is_absel_0(&self) -> bool {
        *self == Absel::Absel0
    }
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    #[inline(always)]
    pub fn is_absel_1(&self) -> bool {
        *self == Absel::Absel1
    }
}
#[doc = "Field `ABSEL` writer - A-side vs. B-side Select"]
pub type AbselW<'a, REG> = crate::BitWriter<'a, REG, Absel>;
impl<'a, REG> AbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    #[inline(always)]
    pub fn absel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Absel::Absel0)
    }
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    #[inline(always)]
    pub fn absel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Absel::Absel1)
    }
}
#[doc = "Differential Mode Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diff {
    #[doc = "0: Single-ended mode."]
    Diff0 = 0,
    #[doc = "1: Differential mode."]
    Diff1 = 1,
}
impl From<Diff> for bool {
    #[inline(always)]
    fn from(variant: Diff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode Enable"]
pub type DiffR = crate::BitReader<Diff>;
impl DiffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diff {
        match self.bits {
            false => Diff::Diff0,
            true => Diff::Diff1,
        }
    }
    #[doc = "Single-ended mode."]
    #[inline(always)]
    pub fn is_diff_0(&self) -> bool {
        *self == Diff::Diff0
    }
    #[doc = "Differential mode."]
    #[inline(always)]
    pub fn is_diff_1(&self) -> bool {
        *self == Diff::Diff1
    }
}
#[doc = "Field `DIFF` writer - Differential Mode Enable"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG, Diff>;
impl<'a, REG> DiffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-ended mode."]
    #[inline(always)]
    pub fn diff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Diff::Diff0)
    }
    #[doc = "Differential mode."]
    #[inline(always)]
    pub fn diff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Diff::Diff1)
    }
}
#[doc = "Channel Scale\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscale {
    #[doc = "0: Scale selected analog channel (Factor of 30/64)"]
    Cscale0 = 0,
    #[doc = "1: (Default) Full scale (Factor of 1)"]
    Cscale1 = 1,
}
impl From<Cscale> for bool {
    #[inline(always)]
    fn from(variant: Cscale) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCALE` reader - Channel Scale"]
pub type CscaleR = crate::BitReader<Cscale>;
impl CscaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscale {
        match self.bits {
            false => Cscale::Cscale0,
            true => Cscale::Cscale1,
        }
    }
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    #[inline(always)]
    pub fn is_cscale_0(&self) -> bool {
        *self == Cscale::Cscale0
    }
    #[doc = "(Default) Full scale (Factor of 1)"]
    #[inline(always)]
    pub fn is_cscale_1(&self) -> bool {
        *self == Cscale::Cscale1
    }
}
#[doc = "Field `CSCALE` writer - Channel Scale"]
pub type CscaleW<'a, REG> = crate::BitWriter<'a, REG, Cscale>;
impl<'a, REG> CscaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    #[inline(always)]
    pub fn cscale_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cscale::Cscale0)
    }
    #[doc = "(Default) Full scale (Factor of 1)"]
    #[inline(always)]
    pub fn cscale_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cscale::Cscale1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> AdchR {
        AdchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - A-side vs. B-side Select"]
    #[inline(always)]
    pub fn absel(&self) -> AbselR {
        AbselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Scale"]
    #[inline(always)]
    pub fn cscale(&self) -> CscaleR {
        CscaleR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL")
            .field("adch", &self.adch())
            .field("absel", &self.absel())
            .field("diff", &self.diff())
            .field("cscale", &self.cscale())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&mut self) -> AdchW<CmdlSpec> {
        AdchW::new(self, 0)
    }
    #[doc = "Bit 5 - A-side vs. B-side Select"]
    #[inline(always)]
    pub fn absel(&mut self) -> AbselW<CmdlSpec> {
        AbselW::new(self, 5)
    }
    #[doc = "Bit 6 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&mut self) -> DiffW<CmdlSpec> {
        DiffW::new(self, 6)
    }
    #[doc = "Bit 13 - Channel Scale"]
    #[inline(always)]
    pub fn cscale(&mut self) -> CscaleW<CmdlSpec> {
        CscaleW::new(self, 13)
    }
}
#[doc = "ADC Command Low Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdlSpec;
impl crate::RegisterSpec for CmdlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdl::R`](R) reader structure"]
impl crate::Readable for CmdlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdl::W`](W) writer structure"]
impl crate::Writable for CmdlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDL%s to value 0x2000"]
impl crate::Resettable for CmdlSpec {
    const RESET_VALUE: u32 = 0x2000;
}
