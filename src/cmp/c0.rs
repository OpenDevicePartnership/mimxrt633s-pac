#[doc = "Register `C0` reader"]
pub type R = crate::R<C0Spec>;
#[doc = "Register `C0` writer"]
pub type W = crate::W<C0Spec>;
#[doc = "Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hystctr {
    #[doc = "0: The hard block output has level 0 hysteresis internally."]
    Hystctr0 = 0,
    #[doc = "1: The hard block output has level 1 hysteresis internally."]
    Hystctr1 = 1,
    #[doc = "2: The hard block output has level 2 hysteresis internally."]
    Hystctr2 = 2,
    #[doc = "3: The hard block output has level 3 hysteresis internally."]
    Hystctr3 = 3,
}
impl From<Hystctr> for u8 {
    #[inline(always)]
    fn from(variant: Hystctr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hystctr {
    type Ux = u8;
}
impl crate::IsEnum for Hystctr {}
#[doc = "Field `HYSTCTR` reader - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
pub type HystctrR = crate::FieldReader<Hystctr>;
impl HystctrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hystctr {
        match self.bits {
            0 => Hystctr::Hystctr0,
            1 => Hystctr::Hystctr1,
            2 => Hystctr::Hystctr2,
            3 => Hystctr::Hystctr3,
            _ => unreachable!(),
        }
    }
    #[doc = "The hard block output has level 0 hysteresis internally."]
    #[inline(always)]
    pub fn is_hystctr_0(&self) -> bool {
        *self == Hystctr::Hystctr0
    }
    #[doc = "The hard block output has level 1 hysteresis internally."]
    #[inline(always)]
    pub fn is_hystctr_1(&self) -> bool {
        *self == Hystctr::Hystctr1
    }
    #[doc = "The hard block output has level 2 hysteresis internally."]
    #[inline(always)]
    pub fn is_hystctr_2(&self) -> bool {
        *self == Hystctr::Hystctr2
    }
    #[doc = "The hard block output has level 3 hysteresis internally."]
    #[inline(always)]
    pub fn is_hystctr_3(&self) -> bool {
        *self == Hystctr::Hystctr3
    }
}
#[doc = "Field `HYSTCTR` writer - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
pub type HystctrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hystctr, crate::Safe>;
impl<'a, REG> HystctrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The hard block output has level 0 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hystctr::Hystctr0)
    }
    #[doc = "The hard block output has level 1 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hystctr::Hystctr1)
    }
    #[doc = "The hard block output has level 2 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Hystctr::Hystctr2)
    }
    #[doc = "The hard block output has level 3 hysteresis internally."]
    #[inline(always)]
    pub fn hystctr_3(self) -> &'a mut crate::W<REG> {
        self.variant(Hystctr::Hystctr3)
    }
}
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FilterCnt {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    FilterCnt0 = 0,
    #[doc = "1: 1 consecutive sample must agree (comparator output is simply sampled)."]
    FilterCnt1 = 1,
    #[doc = "2: 2 consecutive samples must agree."]
    FilterCnt2 = 2,
    #[doc = "3: 3 consecutive samples must agree."]
    FilterCnt3 = 3,
    #[doc = "4: 4 consecutive samples must agree."]
    FilterCnt4 = 4,
    #[doc = "5: 5 consecutive samples must agree."]
    FilterCnt5 = 5,
    #[doc = "6: 6 consecutive samples must agree."]
    FilterCnt6 = 6,
    #[doc = "7: 7 consecutive samples must agree."]
    FilterCnt7 = 7,
}
impl From<FilterCnt> for u8 {
    #[inline(always)]
    fn from(variant: FilterCnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FilterCnt {
    type Ux = u8;
}
impl crate::IsEnum for FilterCnt {}
#[doc = "Field `FILTER_CNT` reader - Filter Sample Count"]
pub type FilterCntR = crate::FieldReader<FilterCnt>;
impl FilterCntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FilterCnt {
        match self.bits {
            0 => FilterCnt::FilterCnt0,
            1 => FilterCnt::FilterCnt1,
            2 => FilterCnt::FilterCnt2,
            3 => FilterCnt::FilterCnt3,
            4 => FilterCnt::FilterCnt4,
            5 => FilterCnt::FilterCnt5,
            6 => FilterCnt::FilterCnt6,
            7 => FilterCnt::FilterCnt7,
            _ => unreachable!(),
        }
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn is_filter_cnt_0(&self) -> bool {
        *self == FilterCnt::FilterCnt0
    }
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    #[inline(always)]
    pub fn is_filter_cnt_1(&self) -> bool {
        *self == FilterCnt::FilterCnt1
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn is_filter_cnt_2(&self) -> bool {
        *self == FilterCnt::FilterCnt2
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn is_filter_cnt_3(&self) -> bool {
        *self == FilterCnt::FilterCnt3
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn is_filter_cnt_4(&self) -> bool {
        *self == FilterCnt::FilterCnt4
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn is_filter_cnt_5(&self) -> bool {
        *self == FilterCnt::FilterCnt5
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn is_filter_cnt_6(&self) -> bool {
        *self == FilterCnt::FilterCnt6
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn is_filter_cnt_7(&self) -> bool {
        *self == FilterCnt::FilterCnt7
    }
}
#[doc = "Field `FILTER_CNT` writer - Filter Sample Count"]
pub type FilterCntW<'a, REG> = crate::FieldWriter<'a, REG, 3, FilterCnt, crate::Safe>;
impl<'a, REG> FilterCntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn filter_cnt_0(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt0)
    }
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    #[inline(always)]
    pub fn filter_cnt_1(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt1)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_2(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt2)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_3(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt3)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_4(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt4)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_5(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt5)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_6(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt6)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_7(self) -> &'a mut crate::W<REG> {
        self.variant(FilterCnt::FilterCnt7)
    }
}
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Analog Comparator is disabled."]
    En0 = 0,
    #[doc = "1: Analog Comparator is enabled."]
    En1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator Module Enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::En0,
            true => En::En1,
        }
    }
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == En::En0
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == En::En1
    }
}
#[doc = "Field `EN` writer - Comparator Module Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut crate::W<REG> {
        self.variant(En::En0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut crate::W<REG> {
        self.variant(En::En1)
    }
}
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ope {
    #[doc = "0: When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    Ope0 = 0,
    #[doc = "1: When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    Ope1 = 1,
}
impl From<Ope> for bool {
    #[inline(always)]
    fn from(variant: Ope) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPE` reader - Comparator Output Pin Enable"]
pub type OpeR = crate::BitReader<Ope>;
impl OpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ope {
        match self.bits {
            false => Ope::Ope0,
            true => Ope::Ope1,
        }
    }
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    #[inline(always)]
    pub fn is_ope_0(&self) -> bool {
        *self == Ope::Ope0
    }
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    #[inline(always)]
    pub fn is_ope_1(&self) -> bool {
        *self == Ope::Ope1
    }
}
#[doc = "Field `OPE` writer - Comparator Output Pin Enable"]
pub type OpeW<'a, REG> = crate::BitWriter<'a, REG, Ope>;
impl<'a, REG> OpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    #[inline(always)]
    pub fn ope_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ope::Ope0)
    }
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    #[inline(always)]
    pub fn ope_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ope::Ope1)
    }
}
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cos {
    #[doc = "0: Set CMPO to equal COUT (filtered comparator output)."]
    Cos0 = 0,
    #[doc = "1: Set CMPO to equal COUTA (unfiltered comparator output)."]
    Cos1 = 1,
}
impl From<Cos> for bool {
    #[inline(always)]
    fn from(variant: Cos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COS` reader - Comparator Output Select"]
pub type CosR = crate::BitReader<Cos>;
impl CosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cos {
        match self.bits {
            false => Cos::Cos0,
            true => Cos::Cos1,
        }
    }
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    #[inline(always)]
    pub fn is_cos_0(&self) -> bool {
        *self == Cos::Cos0
    }
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    #[inline(always)]
    pub fn is_cos_1(&self) -> bool {
        *self == Cos::Cos1
    }
}
#[doc = "Field `COS` writer - Comparator Output Select"]
pub type CosW<'a, REG> = crate::BitWriter<'a, REG, Cos>;
impl<'a, REG> CosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    #[inline(always)]
    pub fn cos_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cos::Cos0)
    }
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    #[inline(always)]
    pub fn cos_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cos::Cos1)
    }
}
#[doc = "Comparator invert\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invt {
    #[doc = "0: Does not invert the comparator output."]
    Invt0 = 0,
    #[doc = "1: Inverts the comparator output."]
    Invt1 = 1,
}
impl From<Invt> for bool {
    #[inline(always)]
    fn from(variant: Invt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVT` reader - Comparator invert"]
pub type InvtR = crate::BitReader<Invt>;
impl InvtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invt {
        match self.bits {
            false => Invt::Invt0,
            true => Invt::Invt1,
        }
    }
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn is_invt_0(&self) -> bool {
        *self == Invt::Invt0
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn is_invt_1(&self) -> bool {
        *self == Invt::Invt1
    }
}
#[doc = "Field `INVT` writer - Comparator invert"]
pub type InvtW<'a, REG> = crate::BitWriter<'a, REG, Invt>;
impl<'a, REG> InvtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn invt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Invt::Invt0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn invt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Invt::Invt1)
    }
}
#[doc = "Power Mode Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmode {
    #[doc = "0: Low Speed (LS) comparison mode is selected."]
    Pmode0 = 0,
    #[doc = "1: High Speed (HS) comparison mode is selected."]
    Pmode1 = 1,
}
impl From<Pmode> for bool {
    #[inline(always)]
    fn from(variant: Pmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Power Mode Select"]
pub type PmodeR = crate::BitReader<Pmode>;
impl PmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmode {
        match self.bits {
            false => Pmode::Pmode0,
            true => Pmode::Pmode1,
        }
    }
    #[doc = "Low Speed (LS) comparison mode is selected."]
    #[inline(always)]
    pub fn is_pmode_0(&self) -> bool {
        *self == Pmode::Pmode0
    }
    #[doc = "High Speed (HS) comparison mode is selected."]
    #[inline(always)]
    pub fn is_pmode_1(&self) -> bool {
        *self == Pmode::Pmode1
    }
}
#[doc = "Field `PMODE` writer - Power Mode Select"]
pub type PmodeW<'a, REG> = crate::BitWriter<'a, REG, Pmode>;
impl<'a, REG> PmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low Speed (LS) comparison mode is selected."]
    #[inline(always)]
    pub fn pmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmode::Pmode0)
    }
    #[doc = "High Speed (HS) comparison mode is selected."]
    #[inline(always)]
    pub fn pmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmode::Pmode1)
    }
}
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum We {
    #[doc = "0: Windowing mode is not selected."]
    We0 = 0,
    #[doc = "1: Windowing mode is selected."]
    We1 = 1,
}
impl From<We> for bool {
    #[inline(always)]
    fn from(variant: We) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE` reader - Windowing Enable"]
pub type WeR = crate::BitReader<We>;
impl WeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> We {
        match self.bits {
            false => We::We0,
            true => We::We1,
        }
    }
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn is_we_0(&self) -> bool {
        *self == We::We0
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn is_we_1(&self) -> bool {
        *self == We::We1
    }
}
#[doc = "Field `WE` writer - Windowing Enable"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG, We>;
impl<'a, REG> WeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn we_0(self) -> &'a mut crate::W<REG> {
        self.variant(We::We0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn we_1(self) -> &'a mut crate::W<REG> {
        self.variant(We::We1)
    }
}
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Se {
    #[doc = "0: Sampling mode is not selected."]
    Se0 = 0,
    #[doc = "1: Sampling mode is selected."]
    Se1 = 1,
}
impl From<Se> for bool {
    #[inline(always)]
    fn from(variant: Se) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Sample Enable"]
pub type SeR = crate::BitReader<Se>;
impl SeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Se {
        match self.bits {
            false => Se::Se0,
            true => Se::Se1,
        }
    }
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn is_se_0(&self) -> bool {
        *self == Se::Se0
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn is_se_1(&self) -> bool {
        *self == Se::Se1
    }
}
#[doc = "Field `SE` writer - Sample Enable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG, Se>;
impl<'a, REG> SeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn se_0(self) -> &'a mut crate::W<REG> {
        self.variant(Se::Se0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn se_1(self) -> &'a mut crate::W<REG> {
        self.variant(Se::Se1)
    }
}
#[doc = "Field `FPR` reader - Filter Sample Period"]
pub type FprR = crate::FieldReader;
#[doc = "Field `FPR` writer - Filter Sample Period"]
pub type FprW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COUT` reader - Analog Comparator Output"]
pub type CoutR = crate::BitReader;
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cff {
    #[doc = "0: A falling edge has not been detected on COUT."]
    Cff0 = 0,
    #[doc = "1: A falling edge on COUT has occurred."]
    Cff1 = 1,
}
impl From<Cff> for bool {
    #[inline(always)]
    fn from(variant: Cff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFF` reader - Analog Comparator Flag Falling"]
pub type CffR = crate::BitReader<Cff>;
impl CffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cff {
        match self.bits {
            false => Cff::Cff0,
            true => Cff::Cff1,
        }
    }
    #[doc = "A falling edge has not been detected on COUT."]
    #[inline(always)]
    pub fn is_cff_0(&self) -> bool {
        *self == Cff::Cff0
    }
    #[doc = "A falling edge on COUT has occurred."]
    #[inline(always)]
    pub fn is_cff_1(&self) -> bool {
        *self == Cff::Cff1
    }
}
#[doc = "Field `CFF` writer - Analog Comparator Flag Falling"]
pub type CffW<'a, REG> = crate::BitWriter1C<'a, REG, Cff>;
impl<'a, REG> CffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A falling edge has not been detected on COUT."]
    #[inline(always)]
    pub fn cff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cff::Cff0)
    }
    #[doc = "A falling edge on COUT has occurred."]
    #[inline(always)]
    pub fn cff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cff::Cff1)
    }
}
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfr {
    #[doc = "0: A rising edge has not been detected on COUT."]
    Cfr0 = 0,
    #[doc = "1: A rising edge on COUT has occurred."]
    Cfr1 = 1,
}
impl From<Cfr> for bool {
    #[inline(always)]
    fn from(variant: Cfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFR` reader - Analog Comparator Flag Rising"]
pub type CfrR = crate::BitReader<Cfr>;
impl CfrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfr {
        match self.bits {
            false => Cfr::Cfr0,
            true => Cfr::Cfr1,
        }
    }
    #[doc = "A rising edge has not been detected on COUT."]
    #[inline(always)]
    pub fn is_cfr_0(&self) -> bool {
        *self == Cfr::Cfr0
    }
    #[doc = "A rising edge on COUT has occurred."]
    #[inline(always)]
    pub fn is_cfr_1(&self) -> bool {
        *self == Cfr::Cfr1
    }
}
#[doc = "Field `CFR` writer - Analog Comparator Flag Rising"]
pub type CfrW<'a, REG> = crate::BitWriter1C<'a, REG, Cfr>;
impl<'a, REG> CfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A rising edge has not been detected on COUT."]
    #[inline(always)]
    pub fn cfr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfr::Cfr0)
    }
    #[doc = "A rising edge on COUT has occurred."]
    #[inline(always)]
    pub fn cfr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfr::Cfr1)
    }
}
#[doc = "Comparator Interrupt Enable Falling\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ief {
    #[doc = "0: Interrupt is disabled."]
    Ief0 = 0,
    #[doc = "1: Interrupt is enabled."]
    Ief1 = 1,
}
impl From<Ief> for bool {
    #[inline(always)]
    fn from(variant: Ief) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEF` reader - Comparator Interrupt Enable Falling"]
pub type IefR = crate::BitReader<Ief>;
impl IefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ief {
        match self.bits {
            false => Ief::Ief0,
            true => Ief::Ief1,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_ief_0(&self) -> bool {
        *self == Ief::Ief0
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_ief_1(&self) -> bool {
        *self == Ief::Ief1
    }
}
#[doc = "Field `IEF` writer - Comparator Interrupt Enable Falling"]
pub type IefW<'a, REG> = crate::BitWriter<'a, REG, Ief>;
impl<'a, REG> IefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn ief_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ief::Ief0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn ief_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ief::Ief1)
    }
}
#[doc = "Comparator Interrupt Enable Rising\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ier {
    #[doc = "0: Interrupt is disabled."]
    Ier0 = 0,
    #[doc = "1: Interrupt is enabled."]
    Ier1 = 1,
}
impl From<Ier> for bool {
    #[inline(always)]
    fn from(variant: Ier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IER` reader - Comparator Interrupt Enable Rising"]
pub type IerR = crate::BitReader<Ier>;
impl IerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ier {
        match self.bits {
            false => Ier::Ier0,
            true => Ier::Ier1,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_ier_0(&self) -> bool {
        *self == Ier::Ier0
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_ier_1(&self) -> bool {
        *self == Ier::Ier1
    }
}
#[doc = "Field `IER` writer - Comparator Interrupt Enable Rising"]
pub type IerW<'a, REG> = crate::BitWriter<'a, REG, Ier>;
impl<'a, REG> IerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn ier_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ier::Ier0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn ier_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ier::Ier1)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA is disabled."]
    Dmaen0 = 0,
    #[doc = "1: DMA is enabled."]
    Dmaen1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Dmaen0,
            true => Dmaen::Dmaen1,
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == Dmaen::Dmaen0
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == Dmaen::Dmaen1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Dmaen0)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Dmaen1)
    }
}
#[doc = "CMP to DAC link enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Linken {
    #[doc = "0: CMP to DAC link is disabled"]
    Linken0 = 0,
    #[doc = "1: CMP to DAC link is enabled."]
    Linken1 = 1,
}
impl From<Linken> for bool {
    #[inline(always)]
    fn from(variant: Linken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINKEN` reader - CMP to DAC link enable."]
pub type LinkenR = crate::BitReader<Linken>;
impl LinkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linken {
        match self.bits {
            false => Linken::Linken0,
            true => Linken::Linken1,
        }
    }
    #[doc = "CMP to DAC link is disabled"]
    #[inline(always)]
    pub fn is_linken_0(&self) -> bool {
        *self == Linken::Linken0
    }
    #[doc = "CMP to DAC link is enabled."]
    #[inline(always)]
    pub fn is_linken_1(&self) -> bool {
        *self == Linken::Linken1
    }
}
#[doc = "Field `LINKEN` writer - CMP to DAC link enable."]
pub type LinkenW<'a, REG> = crate::BitWriter<'a, REG, Linken>;
impl<'a, REG> LinkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMP to DAC link is disabled"]
    #[inline(always)]
    pub fn linken_0(self) -> &'a mut crate::W<REG> {
        self.variant(Linken::Linken0)
    }
    #[doc = "CMP to DAC link is enabled."]
    #[inline(always)]
    pub fn linken_1(self) -> &'a mut crate::W<REG> {
        self.variant(Linken::Linken1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline(always)]
    pub fn hystctr(&self) -> HystctrR {
        HystctrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FilterCntR {
        FilterCntR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&self) -> OpeR {
        OpeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&self) -> CosR {
        CosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline(always)]
    pub fn invt(&self) -> InvtR {
        InvtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&self) -> PmodeR {
        PmodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline(always)]
    pub fn fpr(&self) -> FprR {
        FprR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Analog Comparator Output"]
    #[inline(always)]
    pub fn cout(&self) -> CoutR {
        CoutR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&self) -> CffR {
        CffR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&self) -> CfrR {
        CfrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&self) -> IefR {
        IefR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&self) -> IerR {
        IerR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMP to DAC link enable."]
    #[inline(always)]
    pub fn linken(&self) -> LinkenR {
        LinkenR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C0")
            .field("hystctr", &self.hystctr())
            .field("filter_cnt", &self.filter_cnt())
            .field("en", &self.en())
            .field("ope", &self.ope())
            .field("cos", &self.cos())
            .field("invt", &self.invt())
            .field("pmode", &self.pmode())
            .field("we", &self.we())
            .field("se", &self.se())
            .field("fpr", &self.fpr())
            .field("cout", &self.cout())
            .field("cff", &self.cff())
            .field("cfr", &self.cfr())
            .field("ief", &self.ief())
            .field("ier", &self.ier())
            .field("dmaen", &self.dmaen())
            .field("linken", &self.linken())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline(always)]
    pub fn hystctr(&mut self) -> HystctrW<C0Spec> {
        HystctrW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&mut self) -> FilterCntW<C0Spec> {
        FilterCntW::new(self, 4)
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<C0Spec> {
        EnW::new(self, 8)
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&mut self) -> OpeW<C0Spec> {
        OpeW::new(self, 9)
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&mut self) -> CosW<C0Spec> {
        CosW::new(self, 10)
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline(always)]
    pub fn invt(&mut self) -> InvtW<C0Spec> {
        InvtW::new(self, 11)
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PmodeW<C0Spec> {
        PmodeW::new(self, 12)
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&mut self) -> WeW<C0Spec> {
        WeW::new(self, 14)
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline(always)]
    pub fn se(&mut self) -> SeW<C0Spec> {
        SeW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline(always)]
    pub fn fpr(&mut self) -> FprW<C0Spec> {
        FprW::new(self, 16)
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&mut self) -> CffW<C0Spec> {
        CffW::new(self, 25)
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&mut self) -> CfrW<C0Spec> {
        CfrW::new(self, 26)
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&mut self) -> IefW<C0Spec> {
        IefW::new(self, 27)
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&mut self) -> IerW<C0Spec> {
        IerW::new(self, 28)
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<C0Spec> {
        DmaenW::new(self, 30)
    }
    #[doc = "Bit 31 - CMP to DAC link enable."]
    #[inline(always)]
    pub fn linken(&mut self) -> LinkenW<C0Spec> {
        LinkenW::new(self, 31)
    }
}
#[doc = "CMP Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0Spec;
impl crate::RegisterSpec for C0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0::R`](R) reader structure"]
impl crate::Readable for C0Spec {}
#[doc = "`write(|w| ..)` method takes [`c0::W`](W) writer structure"]
impl crate::Writable for C0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0600_0000;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0Spec {
    const RESET_VALUE: u32 = 0;
}
