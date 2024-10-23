#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "Field `VOSEL` reader - DAC Output Voltage Select"]
pub type VoselR = crate::FieldReader;
#[doc = "Field `VOSEL` writer - DAC Output Voltage Select"]
pub type VoselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "DAC Mode Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmode {
    #[doc = "0: DAC is selected to work in low speed and low power mode."]
    Dmode0 = 0,
    #[doc = "1: DAC is selected to work in high speed high power mode."]
    Dmode1 = 1,
}
impl From<Dmode> for bool {
    #[inline(always)]
    fn from(variant: Dmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMODE` reader - DAC Mode Selection"]
pub type DmodeR = crate::BitReader<Dmode>;
impl DmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmode {
        match self.bits {
            false => Dmode::Dmode0,
            true => Dmode::Dmode1,
        }
    }
    #[doc = "DAC is selected to work in low speed and low power mode."]
    #[inline(always)]
    pub fn is_dmode_0(&self) -> bool {
        *self == Dmode::Dmode0
    }
    #[doc = "DAC is selected to work in high speed high power mode."]
    #[inline(always)]
    pub fn is_dmode_1(&self) -> bool {
        *self == Dmode::Dmode1
    }
}
#[doc = "Field `DMODE` writer - DAC Mode Selection"]
pub type DmodeW<'a, REG> = crate::BitWriter<'a, REG, Dmode>;
impl<'a, REG> DmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC is selected to work in low speed and low power mode."]
    #[inline(always)]
    pub fn dmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmode::Dmode0)
    }
    #[doc = "DAC is selected to work in high speed high power mode."]
    #[inline(always)]
    pub fn dmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmode::Dmode1)
    }
}
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrsel {
    #[doc = "0: Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
    Vrsel0 = 0,
    #[doc = "1: Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
    Vrsel1 = 1,
}
impl From<Vrsel> for bool {
    #[inline(always)]
    fn from(variant: Vrsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRSEL` reader - Supply Voltage Reference Source Select"]
pub type VrselR = crate::BitReader<Vrsel>;
impl VrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrsel {
        match self.bits {
            false => Vrsel::Vrsel0,
            true => Vrsel::Vrsel1,
        }
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
    #[inline(always)]
    pub fn is_vrsel_0(&self) -> bool {
        *self == Vrsel::Vrsel0
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
    #[inline(always)]
    pub fn is_vrsel_1(&self) -> bool {
        *self == Vrsel::Vrsel1
    }
}
#[doc = "Field `VRSEL` writer - Supply Voltage Reference Source Select"]
pub type VrselW<'a, REG> = crate::BitWriter<'a, REG, Vrsel>;
impl<'a, REG> VrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
    #[inline(always)]
    pub fn vrsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Vrsel0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
    #[inline(always)]
    pub fn vrsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Vrsel1)
    }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacen {
    #[doc = "0: DAC is disabled."]
    Dacen0 = 0,
    #[doc = "1: DAC is enabled."]
    Dacen1 = 1,
}
impl From<Dacen> for bool {
    #[inline(always)]
    fn from(variant: Dacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub type DacenR = crate::BitReader<Dacen>;
impl DacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacen {
        match self.bits {
            false => Dacen::Dacen0,
            true => Dacen::Dacen1,
        }
    }
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn is_dacen_0(&self) -> bool {
        *self == Dacen::Dacen0
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn is_dacen_1(&self) -> bool {
        *self == Dacen::Dacen1
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG, Dacen>;
impl<'a, REG> DacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn dacen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::Dacen0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn dacen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::Dacen1)
    }
}
#[doc = "Field `CHN0` reader - Channel 0 input enable"]
pub type Chn0R = crate::BitReader;
#[doc = "Field `CHN0` writer - Channel 0 input enable"]
pub type Chn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHN1` reader - Channel 1 input enable"]
pub type Chn1R = crate::BitReader;
#[doc = "Field `CHN1` writer - Channel 1 input enable"]
pub type Chn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHN2` reader - Channel 2 input enable"]
pub type Chn2R = crate::BitReader;
#[doc = "Field `CHN2` writer - Channel 2 input enable"]
pub type Chn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHN3` reader - Channel 3 input enable"]
pub type Chn3R = crate::BitReader;
#[doc = "Field `CHN3` writer - Channel 3 input enable"]
pub type Chn3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHN4` reader - Channel 4 input enable"]
pub type Chn4R = crate::BitReader;
#[doc = "Field `CHN4` writer - Channel 4 input enable"]
pub type Chn4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHN5` reader - Channel 5 input enable"]
pub type Chn5R = crate::BitReader;
#[doc = "Field `CHN5` writer - Channel 5 input enable"]
pub type Chn5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Minus Input MUX Control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msel {
    #[doc = "0: Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
    Msel0 = 0,
    #[doc = "1: External Input 1 for Minus Channel -- Reference Input 0"]
    Msel1 = 1,
    #[doc = "2: External Input 2 for Minus Channel -- Reference Input 1"]
    Msel2 = 2,
    #[doc = "3: External Input 3 for Minus Channel -- Reference Input 2"]
    Msel3 = 3,
    #[doc = "4: External Input 4 for Minus Channel -- Reference Input 3"]
    Msel4 = 4,
    #[doc = "5: External Input 5 for Minus Channel -- Reference Input 4"]
    Msel5 = 5,
    #[doc = "6: External Input 6 for Minus Channel -- Reference Input 5"]
    Msel6 = 6,
    #[doc = "7: Internal 8b DAC output"]
    Msel7 = 7,
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(variant: Msel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msel {
    type Ux = u8;
}
impl crate::IsEnum for Msel {}
#[doc = "Field `MSEL` reader - Minus Input MUX Control"]
pub type MselR = crate::FieldReader<Msel>;
impl MselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msel {
        match self.bits {
            0 => Msel::Msel0,
            1 => Msel::Msel1,
            2 => Msel::Msel2,
            3 => Msel::Msel3,
            4 => Msel::Msel4,
            5 => Msel::Msel5,
            6 => Msel::Msel6,
            7 => Msel::Msel7,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == Msel::Msel0
    }
    #[doc = "External Input 1 for Minus Channel -- Reference Input 0"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == Msel::Msel1
    }
    #[doc = "External Input 2 for Minus Channel -- Reference Input 1"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == Msel::Msel2
    }
    #[doc = "External Input 3 for Minus Channel -- Reference Input 2"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == Msel::Msel3
    }
    #[doc = "External Input 4 for Minus Channel -- Reference Input 3"]
    #[inline(always)]
    pub fn is_msel_4(&self) -> bool {
        *self == Msel::Msel4
    }
    #[doc = "External Input 5 for Minus Channel -- Reference Input 4"]
    #[inline(always)]
    pub fn is_msel_5(&self) -> bool {
        *self == Msel::Msel5
    }
    #[doc = "External Input 6 for Minus Channel -- Reference Input 5"]
    #[inline(always)]
    pub fn is_msel_6(&self) -> bool {
        *self == Msel::Msel6
    }
    #[doc = "Internal 8b DAC output"]
    #[inline(always)]
    pub fn is_msel_7(&self) -> bool {
        *self == Msel::Msel7
    }
}
#[doc = "Field `MSEL` writer - Minus Input MUX Control"]
pub type MselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Msel, crate::Safe>;
impl<'a, REG> MselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel0)
    }
    #[doc = "External Input 1 for Minus Channel -- Reference Input 0"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel1)
    }
    #[doc = "External Input 2 for Minus Channel -- Reference Input 1"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel2)
    }
    #[doc = "External Input 3 for Minus Channel -- Reference Input 2"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel3)
    }
    #[doc = "External Input 4 for Minus Channel -- Reference Input 3"]
    #[inline(always)]
    pub fn msel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel4)
    }
    #[doc = "External Input 5 for Minus Channel -- Reference Input 4"]
    #[inline(always)]
    pub fn msel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel5)
    }
    #[doc = "External Input 6 for Minus Channel -- Reference Input 5"]
    #[inline(always)]
    pub fn msel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel6)
    }
    #[doc = "Internal 8b DAC output"]
    #[inline(always)]
    pub fn msel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel7)
    }
}
#[doc = "Plus Input MUX Control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: Internal Posivite Input 0 for Plus Channel -- Internal Minus Input"]
    Psel0 = 0,
    #[doc = "1: External Input 1 for Plus Channel -- Reference Input 0"]
    Psel1 = 1,
    #[doc = "2: External Input 2 for Plus Channel -- Reference Input 1"]
    Psel2 = 2,
    #[doc = "3: External Input 3 for Plus Channel -- Reference Input 2"]
    Psel3 = 3,
    #[doc = "4: External Input 4 for Plus Channel -- Reference Input 3"]
    Psel4 = 4,
    #[doc = "5: External Input 4 for Plus Channel -- Reference Input 4"]
    Psel5 = 5,
    #[doc = "6: External Input 4 for Plus Channel -- Reference Input 5"]
    Psel6 = 6,
    #[doc = "7: Internal 8b DAC output"]
    Psel7 = 7,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
impl crate::IsEnum for Psel {}
#[doc = "Field `PSEL` reader - Plus Input MUX Control"]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psel {
        match self.bits {
            0 => Psel::Psel0,
            1 => Psel::Psel1,
            2 => Psel::Psel2,
            3 => Psel::Psel3,
            4 => Psel::Psel4,
            5 => Psel::Psel5,
            6 => Psel::Psel6,
            7 => Psel::Psel7,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Posivite Input 0 for Plus Channel -- Internal Minus Input"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == Psel::Psel0
    }
    #[doc = "External Input 1 for Plus Channel -- Reference Input 0"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == Psel::Psel1
    }
    #[doc = "External Input 2 for Plus Channel -- Reference Input 1"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == Psel::Psel2
    }
    #[doc = "External Input 3 for Plus Channel -- Reference Input 2"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        *self == Psel::Psel3
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 3"]
    #[inline(always)]
    pub fn is_psel_4(&self) -> bool {
        *self == Psel::Psel4
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 4"]
    #[inline(always)]
    pub fn is_psel_5(&self) -> bool {
        *self == Psel::Psel5
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 5"]
    #[inline(always)]
    pub fn is_psel_6(&self) -> bool {
        *self == Psel::Psel6
    }
    #[doc = "Internal 8b DAC output"]
    #[inline(always)]
    pub fn is_psel_7(&self) -> bool {
        *self == Psel::Psel7
    }
}
#[doc = "Field `PSEL` writer - Plus Input MUX Control"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Psel, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Posivite Input 0 for Plus Channel -- Internal Minus Input"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel0)
    }
    #[doc = "External Input 1 for Plus Channel -- Reference Input 0"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel1)
    }
    #[doc = "External Input 2 for Plus Channel -- Reference Input 1"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel2)
    }
    #[doc = "External Input 3 for Plus Channel -- Reference Input 2"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel3)
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 3"]
    #[inline(always)]
    pub fn psel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel4)
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 4"]
    #[inline(always)]
    pub fn psel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel5)
    }
    #[doc = "External Input 4 for Plus Channel -- Reference Input 5"]
    #[inline(always)]
    pub fn psel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel6)
    }
    #[doc = "Internal 8b DAC output"]
    #[inline(always)]
    pub fn psel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::Psel7)
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VoselR {
        VoselR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - DAC Mode Selection"]
    #[inline(always)]
    pub fn dmode(&self) -> DmodeR {
        DmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VrselR {
        VrselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    pub fn chn0(&self) -> Chn0R {
        Chn0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    pub fn chn1(&self) -> Chn1R {
        Chn1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    pub fn chn2(&self) -> Chn2R {
        Chn2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    pub fn chn3(&self) -> Chn3R {
        Chn3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    pub fn chn4(&self) -> Chn4R {
        Chn4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    pub fn chn5(&self) -> Chn5R {
        Chn5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Minus Input MUX Control"]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Plus Input MUX Control"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1")
            .field("vosel", &self.vosel())
            .field("dmode", &self.dmode())
            .field("vrsel", &self.vrsel())
            .field("dacen", &self.dacen())
            .field("chn0", &self.chn0())
            .field("chn1", &self.chn1())
            .field("chn2", &self.chn2())
            .field("chn3", &self.chn3())
            .field("chn4", &self.chn4())
            .field("chn5", &self.chn5())
            .field("msel", &self.msel())
            .field("psel", &self.psel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn vosel(&mut self) -> VoselW<C1Spec> {
        VoselW::new(self, 0)
    }
    #[doc = "Bit 8 - DAC Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DmodeW<C1Spec> {
        DmodeW::new(self, 8)
    }
    #[doc = "Bit 9 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VrselW<C1Spec> {
        VrselW::new(self, 9)
    }
    #[doc = "Bit 10 - DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DacenW<C1Spec> {
        DacenW::new(self, 10)
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn0(&mut self) -> Chn0W<C1Spec> {
        Chn0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn1(&mut self) -> Chn1W<C1Spec> {
        Chn1W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn2(&mut self) -> Chn2W<C1Spec> {
        Chn2W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn3(&mut self) -> Chn3W<C1Spec> {
        Chn3W::new(self, 19)
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn4(&mut self) -> Chn4W<C1Spec> {
        Chn4W::new(self, 20)
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    #[must_use]
    pub fn chn5(&mut self) -> Chn5W<C1Spec> {
        Chn5W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Minus Input MUX Control"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MselW<C1Spec> {
        MselW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Plus Input MUX Control"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<C1Spec> {
        PselW::new(self, 28)
    }
}
#[doc = "CMP Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u32 = 0;
}
