#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Applycfg {
    #[doc = "0: Always reads 0. Write 0 has no effect"]
    Applycfg0 = 0,
    #[doc = "1: Write 1 = initiate update sequencing of PMC state machines"]
    Applycfg1 = 1,
}
impl From<Applycfg> for bool {
    #[inline(always)]
    fn from(variant: Applycfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPLYCFG` reader - Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting"]
pub type ApplycfgR = crate::BitReader<Applycfg>;
impl ApplycfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Applycfg {
        match self.bits {
            false => Applycfg::Applycfg0,
            true => Applycfg::Applycfg1,
        }
    }
    #[doc = "Always reads 0. Write 0 has no effect"]
    #[inline(always)]
    pub fn is_applycfg_0(&self) -> bool {
        *self == Applycfg::Applycfg0
    }
    #[doc = "Write 1 = initiate update sequencing of PMC state machines"]
    #[inline(always)]
    pub fn is_applycfg_1(&self) -> bool {
        *self == Applycfg::Applycfg1
    }
}
#[doc = "Field `APPLYCFG` writer - Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting"]
pub type ApplycfgW<'a, REG> = crate::BitWriter<'a, REG, Applycfg>;
impl<'a, REG> ApplycfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always reads 0. Write 0 has no effect"]
    #[inline(always)]
    pub fn applycfg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Applycfg::Applycfg0)
    }
    #[doc = "Write 1 = initiate update sequencing of PMC state machines"]
    #[inline(always)]
    pub fn applycfg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Applycfg::Applycfg1)
    }
}
#[doc = "Enable analog buffer for references or ATX2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufen {
    #[doc = "0: disabled"]
    Bufen0 = 0,
    #[doc = "1: enabled"]
    Bufen1 = 1,
}
impl From<Bufen> for bool {
    #[inline(always)]
    fn from(variant: Bufen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFEN` reader - Enable analog buffer for references or ATX2"]
pub type BufenR = crate::BitReader<Bufen>;
impl BufenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufen {
        match self.bits {
            false => Bufen::Bufen0,
            true => Bufen::Bufen1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_bufen_0(&self) -> bool {
        *self == Bufen::Bufen0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_bufen_1(&self) -> bool {
        *self == Bufen::Bufen1
    }
}
#[doc = "Field `BUFEN` writer - Enable analog buffer for references or ATX2"]
pub type BufenW<'a, REG> = crate::BitWriter<'a, REG, Bufen>;
impl<'a, REG> BufenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn bufen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::Bufen0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn bufen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Bufen::Bufen1)
    }
}
#[doc = "vddcore Low-Voltage Detector Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdcoreie {
    #[doc = "0: vddcore LVD interrupt disabled"]
    Lvdcoreie0 = 0,
    #[doc = "1: vddcore LVD causes interrupt and wakeup from deep sleep."]
    Lvdcoreie1 = 1,
}
impl From<Lvdcoreie> for bool {
    #[inline(always)]
    fn from(variant: Lvdcoreie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCOREIE` reader - vddcore Low-Voltage Detector Interrupt Enable"]
pub type LvdcoreieR = crate::BitReader<Lvdcoreie>;
impl LvdcoreieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdcoreie {
        match self.bits {
            false => Lvdcoreie::Lvdcoreie0,
            true => Lvdcoreie::Lvdcoreie1,
        }
    }
    #[doc = "vddcore LVD interrupt disabled"]
    #[inline(always)]
    pub fn is_lvdcoreie_0(&self) -> bool {
        *self == Lvdcoreie::Lvdcoreie0
    }
    #[doc = "vddcore LVD causes interrupt and wakeup from deep sleep."]
    #[inline(always)]
    pub fn is_lvdcoreie_1(&self) -> bool {
        *self == Lvdcoreie::Lvdcoreie1
    }
}
#[doc = "Field `LVDCOREIE` writer - vddcore Low-Voltage Detector Interrupt Enable"]
pub type LvdcoreieW<'a, REG> = crate::BitWriter<'a, REG, Lvdcoreie>;
impl<'a, REG> LvdcoreieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore LVD interrupt disabled"]
    #[inline(always)]
    pub fn lvdcoreie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdcoreie::Lvdcoreie0)
    }
    #[doc = "vddcore LVD causes interrupt and wakeup from deep sleep."]
    #[inline(always)]
    pub fn lvdcoreie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdcoreie::Lvdcoreie1)
    }
}
#[doc = "vddcore Low-Voltage Detector Reset Enable\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdcorere {
    #[doc = "0: vddcore LVD reset disabled"]
    Lvdcorere0 = 0,
    #[doc = "1: vddcore LVD causes reset"]
    Lvdcorere1 = 1,
}
impl From<Lvdcorere> for bool {
    #[inline(always)]
    fn from(variant: Lvdcorere) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCORERE` reader - vddcore Low-Voltage Detector Reset Enable"]
pub type LvdcorereR = crate::BitReader<Lvdcorere>;
impl LvdcorereR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdcorere {
        match self.bits {
            false => Lvdcorere::Lvdcorere0,
            true => Lvdcorere::Lvdcorere1,
        }
    }
    #[doc = "vddcore LVD reset disabled"]
    #[inline(always)]
    pub fn is_lvdcorere_0(&self) -> bool {
        *self == Lvdcorere::Lvdcorere0
    }
    #[doc = "vddcore LVD causes reset"]
    #[inline(always)]
    pub fn is_lvdcorere_1(&self) -> bool {
        *self == Lvdcorere::Lvdcorere1
    }
}
#[doc = "Field `LVDCORERE` writer - vddcore Low-Voltage Detector Reset Enable"]
pub type LvdcorereW<'a, REG> = crate::BitWriter<'a, REG, Lvdcorere>;
impl<'a, REG> LvdcorereW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore LVD reset disabled"]
    #[inline(always)]
    pub fn lvdcorere_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdcorere::Lvdcorere0)
    }
    #[doc = "vddcore LVD causes reset"]
    #[inline(always)]
    pub fn lvdcorere_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdcorere::Lvdcorere1)
    }
}
#[doc = "vddcore High-Voltage Detector Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvdcoreie {
    #[doc = "0: vddcore HVD interrupt disabled"]
    Hvdcoreie0 = 0,
    #[doc = "1: vddcore HVD causes interrupt and wakeup from deep sleep."]
    Hvdcoreie1 = 1,
}
impl From<Hvdcoreie> for bool {
    #[inline(always)]
    fn from(variant: Hvdcoreie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCOREIE` reader - vddcore High-Voltage Detector Interrupt Enable"]
pub type HvdcoreieR = crate::BitReader<Hvdcoreie>;
impl HvdcoreieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvdcoreie {
        match self.bits {
            false => Hvdcoreie::Hvdcoreie0,
            true => Hvdcoreie::Hvdcoreie1,
        }
    }
    #[doc = "vddcore HVD interrupt disabled"]
    #[inline(always)]
    pub fn is_hvdcoreie_0(&self) -> bool {
        *self == Hvdcoreie::Hvdcoreie0
    }
    #[doc = "vddcore HVD causes interrupt and wakeup from deep sleep."]
    #[inline(always)]
    pub fn is_hvdcoreie_1(&self) -> bool {
        *self == Hvdcoreie::Hvdcoreie1
    }
}
#[doc = "Field `HVDCOREIE` writer - vddcore High-Voltage Detector Interrupt Enable"]
pub type HvdcoreieW<'a, REG> = crate::BitWriter<'a, REG, Hvdcoreie>;
impl<'a, REG> HvdcoreieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore HVD interrupt disabled"]
    #[inline(always)]
    pub fn hvdcoreie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvdcoreie::Hvdcoreie0)
    }
    #[doc = "vddcore HVD causes interrupt and wakeup from deep sleep."]
    #[inline(always)]
    pub fn hvdcoreie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hvdcoreie::Hvdcoreie1)
    }
}
#[doc = "vddcore High-Voltage Detector Reset Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvdcorere {
    #[doc = "0: vddcore HVD reset disabled"]
    Hvdcorere0 = 0,
    #[doc = "1: vddcore HVD causes reset"]
    Hvdcorere1 = 1,
}
impl From<Hvdcorere> for bool {
    #[inline(always)]
    fn from(variant: Hvdcorere) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCORERE` reader - vddcore High-Voltage Detector Reset Enable"]
pub type HvdcorereR = crate::BitReader<Hvdcorere>;
impl HvdcorereR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvdcorere {
        match self.bits {
            false => Hvdcorere::Hvdcorere0,
            true => Hvdcorere::Hvdcorere1,
        }
    }
    #[doc = "vddcore HVD reset disabled"]
    #[inline(always)]
    pub fn is_hvdcorere_0(&self) -> bool {
        *self == Hvdcorere::Hvdcorere0
    }
    #[doc = "vddcore HVD causes reset"]
    #[inline(always)]
    pub fn is_hvdcorere_1(&self) -> bool {
        *self == Hvdcorere::Hvdcorere1
    }
}
#[doc = "Field `HVDCORERE` writer - vddcore High-Voltage Detector Reset Enable"]
pub type HvdcorereW<'a, REG> = crate::BitWriter<'a, REG, Hvdcorere>;
impl<'a, REG> HvdcorereW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore HVD reset disabled"]
    #[inline(always)]
    pub fn hvdcorere_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvdcorere::Hvdcorere0)
    }
    #[doc = "vddcore HVD causes reset"]
    #[inline(always)]
    pub fn hvdcorere_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hvdcorere::Hvdcorere1)
    }
}
#[doc = "vdd1v8 High-Voltage Detector Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvd1v8ie {
    #[doc = "0: vdd1v8 HVD interrupt disabled"]
    Hvd1v8ie0 = 0,
    #[doc = "1: vdd1v8 HVD causes interrupt and wakeup from deep sleep or deep power down mode"]
    Hvd1v8ie1 = 1,
}
impl From<Hvd1v8ie> for bool {
    #[inline(always)]
    fn from(variant: Hvd1v8ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8IE` reader - vdd1v8 High-Voltage Detector Interrupt Enable"]
pub type Hvd1v8ieR = crate::BitReader<Hvd1v8ie>;
impl Hvd1v8ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvd1v8ie {
        match self.bits {
            false => Hvd1v8ie::Hvd1v8ie0,
            true => Hvd1v8ie::Hvd1v8ie1,
        }
    }
    #[doc = "vdd1v8 HVD interrupt disabled"]
    #[inline(always)]
    pub fn is_hvd1v8ie_0(&self) -> bool {
        *self == Hvd1v8ie::Hvd1v8ie0
    }
    #[doc = "vdd1v8 HVD causes interrupt and wakeup from deep sleep or deep power down mode"]
    #[inline(always)]
    pub fn is_hvd1v8ie_1(&self) -> bool {
        *self == Hvd1v8ie::Hvd1v8ie1
    }
}
#[doc = "Field `HVD1V8IE` writer - vdd1v8 High-Voltage Detector Interrupt Enable"]
pub type Hvd1v8ieW<'a, REG> = crate::BitWriter<'a, REG, Hvd1v8ie>;
impl<'a, REG> Hvd1v8ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vdd1v8 HVD interrupt disabled"]
    #[inline(always)]
    pub fn hvd1v8ie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8ie::Hvd1v8ie0)
    }
    #[doc = "vdd1v8 HVD causes interrupt and wakeup from deep sleep or deep power down mode"]
    #[inline(always)]
    pub fn hvd1v8ie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8ie::Hvd1v8ie1)
    }
}
#[doc = "vdd1v8 High-Voltage Detector Reset Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvd1v8re {
    #[doc = "0: vdd1v8 HVD reset disabled"]
    Hvd1v8re0 = 0,
    #[doc = "1: vdd1v8 HVD causes reset"]
    Hvd1v8re1 = 1,
}
impl From<Hvd1v8re> for bool {
    #[inline(always)]
    fn from(variant: Hvd1v8re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8RE` reader - vdd1v8 High-Voltage Detector Reset Enable"]
pub type Hvd1v8reR = crate::BitReader<Hvd1v8re>;
impl Hvd1v8reR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvd1v8re {
        match self.bits {
            false => Hvd1v8re::Hvd1v8re0,
            true => Hvd1v8re::Hvd1v8re1,
        }
    }
    #[doc = "vdd1v8 HVD reset disabled"]
    #[inline(always)]
    pub fn is_hvd1v8re_0(&self) -> bool {
        *self == Hvd1v8re::Hvd1v8re0
    }
    #[doc = "vdd1v8 HVD causes reset"]
    #[inline(always)]
    pub fn is_hvd1v8re_1(&self) -> bool {
        *self == Hvd1v8re::Hvd1v8re1
    }
}
#[doc = "Field `HVD1V8RE` writer - vdd1v8 High-Voltage Detector Reset Enable"]
pub type Hvd1v8reW<'a, REG> = crate::BitWriter<'a, REG, Hvd1v8re>;
impl<'a, REG> Hvd1v8reW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vdd1v8 HVD reset disabled"]
    #[inline(always)]
    pub fn hvd1v8re_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8re::Hvd1v8re0)
    }
    #[doc = "vdd1v8 HVD causes reset"]
    #[inline(always)]
    pub fn hvd1v8re_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8re::Hvd1v8re1)
    }
}
#[doc = "PMC automatic wakeup enable and interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autowken {
    #[doc = "0: Auto wakeup interrupt and counter disabled"]
    Autowken0 = 0,
    #[doc = "1: Auto wakeup interrupt generated when PMC sequencer finishes and AUTOWAKE counter = 0 after entering deep sleep mode (but not deep powerdown mode). Interrupt will wake up the M33."]
    Autowken1 = 1,
}
impl From<Autowken> for bool {
    #[inline(always)]
    fn from(variant: Autowken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOWKEN` reader - PMC automatic wakeup enable and interrupt enable"]
pub type AutowkenR = crate::BitReader<Autowken>;
impl AutowkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autowken {
        match self.bits {
            false => Autowken::Autowken0,
            true => Autowken::Autowken1,
        }
    }
    #[doc = "Auto wakeup interrupt and counter disabled"]
    #[inline(always)]
    pub fn is_autowken_0(&self) -> bool {
        *self == Autowken::Autowken0
    }
    #[doc = "Auto wakeup interrupt generated when PMC sequencer finishes and AUTOWAKE counter = 0 after entering deep sleep mode (but not deep powerdown mode). Interrupt will wake up the M33."]
    #[inline(always)]
    pub fn is_autowken_1(&self) -> bool {
        *self == Autowken::Autowken1
    }
}
#[doc = "Field `AUTOWKEN` writer - PMC automatic wakeup enable and interrupt enable"]
pub type AutowkenW<'a, REG> = crate::BitWriter<'a, REG, Autowken>;
impl<'a, REG> AutowkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto wakeup interrupt and counter disabled"]
    #[inline(always)]
    pub fn autowken_0(self) -> &'a mut crate::W<REG> {
        self.variant(Autowken::Autowken0)
    }
    #[doc = "Auto wakeup interrupt generated when PMC sequencer finishes and AUTOWAKE counter = 0 after entering deep sleep mode (but not deep powerdown mode). Interrupt will wake up the M33."]
    #[inline(always)]
    pub fn autowken_1(self) -> &'a mut crate::W<REG> {
        self.variant(Autowken::Autowken1)
    }
}
#[doc = "PMIC interrupt pin enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intrpaden {
    #[doc = "0: Interrupt pad low has no effect"]
    Intrpaden0 = 0,
    #[doc = "1: Interrupt pad low triggers an interrupt and deep sleep wakeup or deep powerdown wakeup event."]
    Intrpaden1 = 1,
}
impl From<Intrpaden> for bool {
    #[inline(always)]
    fn from(variant: Intrpaden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTRPADEN` reader - PMIC interrupt pin enable"]
pub type IntrpadenR = crate::BitReader<Intrpaden>;
impl IntrpadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intrpaden {
        match self.bits {
            false => Intrpaden::Intrpaden0,
            true => Intrpaden::Intrpaden1,
        }
    }
    #[doc = "Interrupt pad low has no effect"]
    #[inline(always)]
    pub fn is_intrpaden_0(&self) -> bool {
        *self == Intrpaden::Intrpaden0
    }
    #[doc = "Interrupt pad low triggers an interrupt and deep sleep wakeup or deep powerdown wakeup event."]
    #[inline(always)]
    pub fn is_intrpaden_1(&self) -> bool {
        *self == Intrpaden::Intrpaden1
    }
}
#[doc = "Field `INTRPADEN` writer - PMIC interrupt pin enable"]
pub type IntrpadenW<'a, REG> = crate::BitWriter<'a, REG, Intrpaden>;
impl<'a, REG> IntrpadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt pad low has no effect"]
    #[inline(always)]
    pub fn intrpaden_0(self) -> &'a mut crate::W<REG> {
        self.variant(Intrpaden::Intrpaden0)
    }
    #[doc = "Interrupt pad low triggers an interrupt and deep sleep wakeup or deep powerdown wakeup event."]
    #[inline(always)]
    pub fn intrpaden_1(self) -> &'a mut crate::W<REG> {
        self.variant(Intrpaden::Intrpaden1)
    }
}
impl R {
    #[doc = "Bit 0 - Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting"]
    #[inline(always)]
    pub fn applycfg(&self) -> ApplycfgR {
        ApplycfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable analog buffer for references or ATX2"]
    #[inline(always)]
    pub fn bufen(&self) -> BufenR {
        BufenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn lvdcoreie(&self) -> LvdcoreieR {
        LvdcoreieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - vddcore Low-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub fn lvdcorere(&self) -> LvdcorereR {
        LvdcorereR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn hvdcoreie(&self) -> HvdcoreieR {
        HvdcoreieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - vddcore High-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub fn hvdcorere(&self) -> HvdcorereR {
        HvdcorereR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub fn hvd1v8ie(&self) -> Hvd1v8ieR {
        Hvd1v8ieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - vdd1v8 High-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub fn hvd1v8re(&self) -> Hvd1v8reR {
        Hvd1v8reR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - PMC automatic wakeup enable and interrupt enable"]
    #[inline(always)]
    pub fn autowken(&self) -> AutowkenR {
        AutowkenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PMIC interrupt pin enable"]
    #[inline(always)]
    pub fn intrpaden(&self) -> IntrpadenR {
        IntrpadenR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("applycfg", &self.applycfg())
            .field("bufen", &self.bufen())
            .field("lvdcoreie", &self.lvdcoreie())
            .field("lvdcorere", &self.lvdcorere())
            .field("hvdcoreie", &self.hvdcoreie())
            .field("hvdcorere", &self.hvdcorere())
            .field("hvd1v8ie", &self.hvd1v8ie())
            .field("hvd1v8re", &self.hvd1v8re())
            .field("autowken", &self.autowken())
            .field("intrpaden", &self.intrpaden())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting"]
    #[inline(always)]
    #[must_use]
    pub fn applycfg(&mut self) -> ApplycfgW<CtrlSpec> {
        ApplycfgW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable analog buffer for references or ATX2"]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BufenW<CtrlSpec> {
        BufenW::new(self, 4)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcoreie(&mut self) -> LvdcoreieW<CtrlSpec> {
        LvdcoreieW::new(self, 20)
    }
    #[doc = "Bit 21 - vddcore Low-Voltage Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcorere(&mut self) -> LvdcorereW<CtrlSpec> {
        LvdcorereW::new(self, 21)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcoreie(&mut self) -> HvdcoreieW<CtrlSpec> {
        HvdcoreieW::new(self, 22)
    }
    #[doc = "Bit 23 - vddcore High-Voltage Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcorere(&mut self) -> HvdcorereW<CtrlSpec> {
        HvdcorereW::new(self, 23)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8ie(&mut self) -> Hvd1v8ieW<CtrlSpec> {
        Hvd1v8ieW::new(self, 24)
    }
    #[doc = "Bit 25 - vdd1v8 High-Voltage Detector Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8re(&mut self) -> Hvd1v8reW<CtrlSpec> {
        Hvd1v8reW::new(self, 25)
    }
    #[doc = "Bit 28 - PMC automatic wakeup enable and interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn autowken(&mut self) -> AutowkenW<CtrlSpec> {
        AutowkenW::new(self, 28)
    }
    #[doc = "Bit 29 - PMIC interrupt pin enable"]
    #[inline(always)]
    #[must_use]
    pub fn intrpaden(&mut self) -> IntrpadenW<CtrlSpec> {
        IntrpadenW::new(self, 29)
    }
}
#[doc = "PMC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0020_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
