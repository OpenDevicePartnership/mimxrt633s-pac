#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "ADC trigger priority control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tprictrl {
    #[doc = "0: If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    Tprictrl0 = 0,
    #[doc = "1: If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations and compare function if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated."]
    Tprictrl1 = 1,
}
impl From<Tprictrl> for bool {
    #[inline(always)]
    fn from(variant: Tprictrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPRICTRL` reader - ADC trigger priority control"]
pub type TprictrlR = crate::BitReader<Tprictrl>;
impl TprictrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tprictrl {
        match self.bits {
            false => Tprictrl::Tprictrl0,
            true => Tprictrl::Tprictrl1,
        }
    }
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    #[inline(always)]
    pub fn is_tprictrl_0(&self) -> bool {
        *self == Tprictrl::Tprictrl0
    }
    #[doc = "If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations and compare function if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated."]
    #[inline(always)]
    pub fn is_tprictrl_1(&self) -> bool {
        *self == Tprictrl::Tprictrl1
    }
}
#[doc = "Field `TPRICTRL` writer - ADC trigger priority control"]
pub type TprictrlW<'a, REG> = crate::BitWriter<'a, REG, Tprictrl>;
impl<'a, REG> TprictrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    #[inline(always)]
    pub fn tprictrl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tprictrl::Tprictrl0)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations and compare function if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated."]
    #[inline(always)]
    pub fn tprictrl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tprictrl::Tprictrl1)
    }
}
#[doc = "Power Configuration Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwrsel {
    #[doc = "0: Lowest power setting."]
    Pwrsel0 = 0,
    #[doc = "1: Next lowest power setting."]
    Pwrsel1 = 1,
    #[doc = "2: ...."]
    Pwrsel2 = 2,
    #[doc = "3: Highest power setting."]
    Pwrsel3 = 3,
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(variant: Pwrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwrsel {
    type Ux = u8;
}
impl crate::IsEnum for Pwrsel {}
#[doc = "Field `PWRSEL` reader - Power Configuration Select"]
pub type PwrselR = crate::FieldReader<Pwrsel>;
impl PwrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrsel {
        match self.bits {
            0 => Pwrsel::Pwrsel0,
            1 => Pwrsel::Pwrsel1,
            2 => Pwrsel::Pwrsel2,
            3 => Pwrsel::Pwrsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest power setting."]
    #[inline(always)]
    pub fn is_pwrsel_0(&self) -> bool {
        *self == Pwrsel::Pwrsel0
    }
    #[doc = "Next lowest power setting."]
    #[inline(always)]
    pub fn is_pwrsel_1(&self) -> bool {
        *self == Pwrsel::Pwrsel1
    }
    #[doc = "...."]
    #[inline(always)]
    pub fn is_pwrsel_2(&self) -> bool {
        *self == Pwrsel::Pwrsel2
    }
    #[doc = "Highest power setting."]
    #[inline(always)]
    pub fn is_pwrsel_3(&self) -> bool {
        *self == Pwrsel::Pwrsel3
    }
}
#[doc = "Field `PWRSEL` writer - Power Configuration Select"]
pub type PwrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pwrsel, crate::Safe>;
impl<'a, REG> PwrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest power setting."]
    #[inline(always)]
    pub fn pwrsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsel::Pwrsel0)
    }
    #[doc = "Next lowest power setting."]
    #[inline(always)]
    pub fn pwrsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsel::Pwrsel1)
    }
    #[doc = "...."]
    #[inline(always)]
    pub fn pwrsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsel::Pwrsel2)
    }
    #[doc = "Highest power setting."]
    #[inline(always)]
    pub fn pwrsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsel::Pwrsel3)
    }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: (Default) Option 1 setting."]
    Refsel0 = 0,
    #[doc = "1: Option 2 setting."]
    Refsel1 = 1,
    #[doc = "2: Option 3 setting."]
    Refsel2 = 2,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refsel> {
        match self.bits {
            0 => Some(Refsel::Refsel0),
            1 => Some(Refsel::Refsel1),
            2 => Some(Refsel::Refsel2),
            _ => None,
        }
    }
    #[doc = "(Default) Option 1 setting."]
    #[inline(always)]
    pub fn is_refsel_0(&self) -> bool {
        *self == Refsel::Refsel0
    }
    #[doc = "Option 2 setting."]
    #[inline(always)]
    pub fn is_refsel_1(&self) -> bool {
        *self == Refsel::Refsel1
    }
    #[doc = "Option 3 setting."]
    #[inline(always)]
    pub fn is_refsel_2(&self) -> bool {
        *self == Refsel::Refsel2
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "(Default) Option 1 setting."]
    #[inline(always)]
    pub fn refsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Refsel0)
    }
    #[doc = "Option 2 setting."]
    #[inline(always)]
    pub fn refsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Refsel1)
    }
    #[doc = "Option 3 setting."]
    #[inline(always)]
    pub fn refsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Refsel2)
    }
}
#[doc = "Field `PUDLY` reader - Power Up Delay"]
pub type PudlyR = crate::FieldReader;
#[doc = "Field `PUDLY` writer - Power Up Delay"]
pub type PudlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "ADC Analog Pre-Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwren {
    #[doc = "0: ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    Pwren0 = 0,
    #[doc = "1: ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
    Pwren1 = 1,
}
impl From<Pwren> for bool {
    #[inline(always)]
    fn from(variant: Pwren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWREN` reader - ADC Analog Pre-Enable"]
pub type PwrenR = crate::BitReader<Pwren>;
impl PwrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwren {
        match self.bits {
            false => Pwren::Pwren0,
            true => Pwren::Pwren1,
        }
    }
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    #[inline(always)]
    pub fn is_pwren_0(&self) -> bool {
        *self == Pwren::Pwren0
    }
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
    #[inline(always)]
    pub fn is_pwren_1(&self) -> bool {
        *self == Pwren::Pwren1
    }
}
#[doc = "Field `PWREN` writer - ADC Analog Pre-Enable"]
pub type PwrenW<'a, REG> = crate::BitWriter<'a, REG, Pwren>;
impl<'a, REG> PwrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    #[inline(always)]
    pub fn pwren_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwren::Pwren0)
    }
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
    #[inline(always)]
    pub fn pwren_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwren::Pwren1)
    }
}
impl R {
    #[doc = "Bit 0 - ADC trigger priority control"]
    #[inline(always)]
    pub fn tprictrl(&self) -> TprictrlR {
        TprictrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PwrselR {
        PwrselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline(always)]
    pub fn pudly(&self) -> PudlyR {
        PudlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("tprictrl", &self.tprictrl())
            .field("pwrsel", &self.pwrsel())
            .field("refsel", &self.refsel())
            .field("pudly", &self.pudly())
            .field("pwren", &self.pwren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC trigger priority control"]
    #[inline(always)]
    pub fn tprictrl(&mut self) -> TprictrlW<CfgSpec> {
        TprictrlW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&mut self) -> PwrselW<CfgSpec> {
        PwrselW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<CfgSpec> {
        RefselW::new(self, 6)
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline(always)]
    pub fn pudly(&mut self) -> PudlyW<CfgSpec> {
        PudlyW::new(self, 16)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PwrenW<CfgSpec> {
        PwrenW::new(self, 28)
    }
}
#[doc = "ADC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x0080_0000"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
