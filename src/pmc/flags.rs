#[doc = "Register `FLAGS` reader"]
pub type R = crate::R<FlagsSpec>;
#[doc = "Register `FLAGS` writer"]
pub type W = crate::W<FlagsSpec>;
#[doc = "vddcore POR Flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porcoref {
    #[doc = "0: vddcore POR was not tripped since the last cleared."]
    Porcoref0 = 0,
    #[doc = "1: POR triggered by the vddcore POR monitor. Write 1 to clear"]
    Porcoref1 = 1,
}
impl From<Porcoref> for bool {
    #[inline(always)]
    fn from(variant: Porcoref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORCOREF` reader - vddcore POR Flag"]
pub type PorcorefR = crate::BitReader<Porcoref>;
impl PorcorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porcoref {
        match self.bits {
            false => Porcoref::Porcoref0,
            true => Porcoref::Porcoref1,
        }
    }
    #[doc = "vddcore POR was not tripped since the last cleared."]
    #[inline(always)]
    pub fn is_porcoref_0(&self) -> bool {
        *self == Porcoref::Porcoref0
    }
    #[doc = "POR triggered by the vddcore POR monitor. Write 1 to clear"]
    #[inline(always)]
    pub fn is_porcoref_1(&self) -> bool {
        *self == Porcoref::Porcoref1
    }
}
#[doc = "Field `PORCOREF` writer - vddcore POR Flag"]
pub type PorcorefW<'a, REG> = crate::BitWriter<'a, REG, Porcoref>;
impl<'a, REG> PorcorefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore POR was not tripped since the last cleared."]
    #[inline(always)]
    pub fn porcoref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Porcoref::Porcoref0)
    }
    #[doc = "POR triggered by the vddcore POR monitor. Write 1 to clear"]
    #[inline(always)]
    pub fn porcoref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Porcoref::Porcoref1)
    }
}
#[doc = "vdd1v8 power on reset flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Por1v8f {
    #[doc = "0: No vdd1v8 power on event detected since last cleared."]
    Por1v8f0 = 0,
    #[doc = "1: vdd1v8 power on detect caused a reset or deep pd wakeup. Write 1 to clear."]
    Por1v8f1 = 1,
}
impl From<Por1v8f> for bool {
    #[inline(always)]
    fn from(variant: Por1v8f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR1V8F` reader - vdd1v8 power on reset flag"]
pub type Por1v8fR = crate::BitReader<Por1v8f>;
impl Por1v8fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Por1v8f {
        match self.bits {
            false => Por1v8f::Por1v8f0,
            true => Por1v8f::Por1v8f1,
        }
    }
    #[doc = "No vdd1v8 power on event detected since last cleared."]
    #[inline(always)]
    pub fn is_por1v8f_0(&self) -> bool {
        *self == Por1v8f::Por1v8f0
    }
    #[doc = "vdd1v8 power on detect caused a reset or deep pd wakeup. Write 1 to clear."]
    #[inline(always)]
    pub fn is_por1v8f_1(&self) -> bool {
        *self == Por1v8f::Por1v8f1
    }
}
#[doc = "Field `POR1V8F` writer - vdd1v8 power on reset flag"]
pub type Por1v8fW<'a, REG> = crate::BitWriter<'a, REG, Por1v8f>;
impl<'a, REG> Por1v8fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No vdd1v8 power on event detected since last cleared."]
    #[inline(always)]
    pub fn por1v8f_0(self) -> &'a mut crate::W<REG> {
        self.variant(Por1v8f::Por1v8f0)
    }
    #[doc = "vdd1v8 power on detect caused a reset or deep pd wakeup. Write 1 to clear."]
    #[inline(always)]
    pub fn por1v8f_1(self) -> &'a mut crate::W<REG> {
        self.variant(Por1v8f::Por1v8f1)
    }
}
#[doc = "vdd_ao18 power on reset flag\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porao18f {
    #[doc = "0: No vdd_ao18 power on event detected since last cleared."]
    Porao18f0 = 0,
    #[doc = "1: vdd_ao18 power on detect caused a reset. Write 1 to clear."]
    Porao18f1 = 1,
}
impl From<Porao18f> for bool {
    #[inline(always)]
    fn from(variant: Porao18f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORAO18F` reader - vdd_ao18 power on reset flag"]
pub type Porao18fR = crate::BitReader<Porao18f>;
impl Porao18fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porao18f {
        match self.bits {
            false => Porao18f::Porao18f0,
            true => Porao18f::Porao18f1,
        }
    }
    #[doc = "No vdd_ao18 power on event detected since last cleared."]
    #[inline(always)]
    pub fn is_porao18f_0(&self) -> bool {
        *self == Porao18f::Porao18f0
    }
    #[doc = "vdd_ao18 power on detect caused a reset. Write 1 to clear."]
    #[inline(always)]
    pub fn is_porao18f_1(&self) -> bool {
        *self == Porao18f::Porao18f1
    }
}
#[doc = "Field `PORAO18F` writer - vdd_ao18 power on reset flag"]
pub type Porao18fW<'a, REG> = crate::BitWriter<'a, REG, Porao18f>;
impl<'a, REG> Porao18fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No vdd_ao18 power on event detected since last cleared."]
    #[inline(always)]
    pub fn porao18f_0(self) -> &'a mut crate::W<REG> {
        self.variant(Porao18f::Porao18f0)
    }
    #[doc = "vdd_ao18 power on detect caused a reset. Write 1 to clear."]
    #[inline(always)]
    pub fn porao18f_1(self) -> &'a mut crate::W<REG> {
        self.variant(Porao18f::Porao18f1)
    }
}
#[doc = "vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvdcoref {
    #[doc = "0: vddcore LVD has not triggered an interrupt or reset since last clear"]
    Lvdcoref0 = 0,
    #[doc = "1: vddcore LVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    Lvdcoref1 = 1,
}
impl From<Lvdcoref> for bool {
    #[inline(always)]
    fn from(variant: Lvdcoref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCOREF` reader - vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset"]
pub type LvdcorefR = crate::BitReader<Lvdcoref>;
impl LvdcorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvdcoref {
        match self.bits {
            false => Lvdcoref::Lvdcoref0,
            true => Lvdcoref::Lvdcoref1,
        }
    }
    #[doc = "vddcore LVD has not triggered an interrupt or reset since last clear"]
    #[inline(always)]
    pub fn is_lvdcoref_0(&self) -> bool {
        *self == Lvdcoref::Lvdcoref0
    }
    #[doc = "vddcore LVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn is_lvdcoref_1(&self) -> bool {
        *self == Lvdcoref::Lvdcoref1
    }
}
#[doc = "Field `LVDCOREF` writer - vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset"]
pub type LvdcorefW<'a, REG> = crate::BitWriter<'a, REG, Lvdcoref>;
impl<'a, REG> LvdcorefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore LVD has not triggered an interrupt or reset since last clear"]
    #[inline(always)]
    pub fn lvdcoref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdcoref::Lvdcoref0)
    }
    #[doc = "vddcore LVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn lvdcoref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvdcoref::Lvdcoref1)
    }
}
#[doc = "vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvdcoref {
    #[doc = "0: vddcore HVD has not triggered an interrupt or reset since last clear"]
    Hvdcoref0 = 0,
    #[doc = "1: vddcore HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    Hvdcoref1 = 1,
}
impl From<Hvdcoref> for bool {
    #[inline(always)]
    fn from(variant: Hvdcoref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCOREF` reader - vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset"]
pub type HvdcorefR = crate::BitReader<Hvdcoref>;
impl HvdcorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvdcoref {
        match self.bits {
            false => Hvdcoref::Hvdcoref0,
            true => Hvdcoref::Hvdcoref1,
        }
    }
    #[doc = "vddcore HVD has not triggered an interrupt or reset since last clear"]
    #[inline(always)]
    pub fn is_hvdcoref_0(&self) -> bool {
        *self == Hvdcoref::Hvdcoref0
    }
    #[doc = "vddcore HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn is_hvdcoref_1(&self) -> bool {
        *self == Hvdcoref::Hvdcoref1
    }
}
#[doc = "Field `HVDCOREF` writer - vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset"]
pub type HvdcorefW<'a, REG> = crate::BitWriter<'a, REG, Hvdcoref>;
impl<'a, REG> HvdcorefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vddcore HVD has not triggered an interrupt or reset since last clear"]
    #[inline(always)]
    pub fn hvdcoref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvdcoref::Hvdcoref0)
    }
    #[doc = "vddcore HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn hvdcoref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hvdcoref::Hvdcoref1)
    }
}
#[doc = "vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvd1v8f {
    #[doc = "0: vdd1v8 HVD has not triggered an interrupt or reset since last clear"]
    Hvd1v8f0 = 0,
    #[doc = "1: vdd1v8 HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    Hvd1v8f1 = 1,
}
impl From<Hvd1v8f> for bool {
    #[inline(always)]
    fn from(variant: Hvd1v8f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8F` reader - vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset"]
pub type Hvd1v8fR = crate::BitReader<Hvd1v8f>;
impl Hvd1v8fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvd1v8f {
        match self.bits {
            false => Hvd1v8f::Hvd1v8f0,
            true => Hvd1v8f::Hvd1v8f1,
        }
    }
    #[doc = "vdd1v8 HVD has not triggered an interrupt or reset since last clear"]
    #[inline(always)]
    pub fn is_hvd1v8f_0(&self) -> bool {
        *self == Hvd1v8f::Hvd1v8f0
    }
    #[doc = "vdd1v8 HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn is_hvd1v8f_1(&self) -> bool {
        *self == Hvd1v8f::Hvd1v8f1
    }
}
#[doc = "Field `HVD1V8F` writer - vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset"]
pub type Hvd1v8fW<'a, REG> = crate::BitWriter<'a, REG, Hvd1v8f>;
impl<'a, REG> Hvd1v8fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "vdd1v8 HVD has not triggered an interrupt or reset since last clear"]
    #[inline(always)]
    pub fn hvd1v8f_0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8f::Hvd1v8f0)
    }
    #[doc = "vdd1v8 HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn hvd1v8f_1(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8f::Hvd1v8f1)
    }
}
#[doc = "RTC Wakeup from deep powerdown mode flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcf {
    #[doc = "0: No RTC wakeup detected since last time flag was cleared."]
    Rtcf0 = 0,
    #[doc = "1: RTC wakeup caused a deep powerdown wakeup. Write 1 to clear."]
    Rtcf1 = 1,
}
impl From<Rtcf> for bool {
    #[inline(always)]
    fn from(variant: Rtcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCF` reader - RTC Wakeup from deep powerdown mode flag."]
pub type RtcfR = crate::BitReader<Rtcf>;
impl RtcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcf {
        match self.bits {
            false => Rtcf::Rtcf0,
            true => Rtcf::Rtcf1,
        }
    }
    #[doc = "No RTC wakeup detected since last time flag was cleared."]
    #[inline(always)]
    pub fn is_rtcf_0(&self) -> bool {
        *self == Rtcf::Rtcf0
    }
    #[doc = "RTC wakeup caused a deep powerdown wakeup. Write 1 to clear."]
    #[inline(always)]
    pub fn is_rtcf_1(&self) -> bool {
        *self == Rtcf::Rtcf1
    }
}
#[doc = "Field `RTCF` writer - RTC Wakeup from deep powerdown mode flag."]
pub type RtcfW<'a, REG> = crate::BitWriter<'a, REG, Rtcf>;
impl<'a, REG> RtcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RTC wakeup detected since last time flag was cleared."]
    #[inline(always)]
    pub fn rtcf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcf::Rtcf0)
    }
    #[doc = "RTC wakeup caused a deep powerdown wakeup. Write 1 to clear."]
    #[inline(always)]
    pub fn rtcf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcf::Rtcf1)
    }
}
#[doc = "PMC Auto Wakeup Interrupt flag.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autowkf {
    #[doc = "0: No PMC Auto Wakeup Interrupt detected since last time cleared."]
    Autowkf0 = 0,
    #[doc = "1: PMC Auto wakeup caused a deep sleep wakeup and interrupt. Write 1 to clear."]
    Autowkf1 = 1,
}
impl From<Autowkf> for bool {
    #[inline(always)]
    fn from(variant: Autowkf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOWKF` reader - PMC Auto Wakeup Interrupt flag."]
pub type AutowkfR = crate::BitReader<Autowkf>;
impl AutowkfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autowkf {
        match self.bits {
            false => Autowkf::Autowkf0,
            true => Autowkf::Autowkf1,
        }
    }
    #[doc = "No PMC Auto Wakeup Interrupt detected since last time cleared."]
    #[inline(always)]
    pub fn is_autowkf_0(&self) -> bool {
        *self == Autowkf::Autowkf0
    }
    #[doc = "PMC Auto wakeup caused a deep sleep wakeup and interrupt. Write 1 to clear."]
    #[inline(always)]
    pub fn is_autowkf_1(&self) -> bool {
        *self == Autowkf::Autowkf1
    }
}
#[doc = "Field `AUTOWKF` writer - PMC Auto Wakeup Interrupt flag."]
pub type AutowkfW<'a, REG> = crate::BitWriter<'a, REG, Autowkf>;
impl<'a, REG> AutowkfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PMC Auto Wakeup Interrupt detected since last time cleared."]
    #[inline(always)]
    pub fn autowkf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Autowkf::Autowkf0)
    }
    #[doc = "PMC Auto wakeup caused a deep sleep wakeup and interrupt. Write 1 to clear."]
    #[inline(always)]
    pub fn autowkf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Autowkf::Autowkf1)
    }
}
#[doc = "PMIC interrupt pin flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intnpadf {
    #[doc = "0: No interrupt detected since flag last cleared."]
    Intnpadf0 = 0,
    #[doc = "1: Pad interrupt caused a wakeup or interrupt event since the last time this flag was cleared. Write 1 to clear."]
    Intnpadf1 = 1,
}
impl From<Intnpadf> for bool {
    #[inline(always)]
    fn from(variant: Intnpadf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTNPADF` reader - PMIC interrupt pin flag"]
pub type IntnpadfR = crate::BitReader<Intnpadf>;
impl IntnpadfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intnpadf {
        match self.bits {
            false => Intnpadf::Intnpadf0,
            true => Intnpadf::Intnpadf1,
        }
    }
    #[doc = "No interrupt detected since flag last cleared."]
    #[inline(always)]
    pub fn is_intnpadf_0(&self) -> bool {
        *self == Intnpadf::Intnpadf0
    }
    #[doc = "Pad interrupt caused a wakeup or interrupt event since the last time this flag was cleared. Write 1 to clear."]
    #[inline(always)]
    pub fn is_intnpadf_1(&self) -> bool {
        *self == Intnpadf::Intnpadf1
    }
}
#[doc = "Field `INTNPADF` writer - PMIC interrupt pin flag"]
pub type IntnpadfW<'a, REG> = crate::BitWriter<'a, REG, Intnpadf>;
impl<'a, REG> IntnpadfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt detected since flag last cleared."]
    #[inline(always)]
    pub fn intnpadf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Intnpadf::Intnpadf0)
    }
    #[doc = "Pad interrupt caused a wakeup or interrupt event since the last time this flag was cleared. Write 1 to clear."]
    #[inline(always)]
    pub fn intnpadf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Intnpadf::Intnpadf1)
    }
}
#[doc = "Reset pad flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetnpadf {
    #[doc = "0: No reset detected since last time this flag was cleared."]
    Resetnpadf0 = 0,
    #[doc = "1: Reset pad wakeup caused a wakeup or reset event since the last time this bit was cleared. Write 1 to clear."]
    Resetnpadf1 = 1,
}
impl From<Resetnpadf> for bool {
    #[inline(always)]
    fn from(variant: Resetnpadf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETNPADF` reader - Reset pad flag"]
pub type ResetnpadfR = crate::BitReader<Resetnpadf>;
impl ResetnpadfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetnpadf {
        match self.bits {
            false => Resetnpadf::Resetnpadf0,
            true => Resetnpadf::Resetnpadf1,
        }
    }
    #[doc = "No reset detected since last time this flag was cleared."]
    #[inline(always)]
    pub fn is_resetnpadf_0(&self) -> bool {
        *self == Resetnpadf::Resetnpadf0
    }
    #[doc = "Reset pad wakeup caused a wakeup or reset event since the last time this bit was cleared. Write 1 to clear."]
    #[inline(always)]
    pub fn is_resetnpadf_1(&self) -> bool {
        *self == Resetnpadf::Resetnpadf1
    }
}
#[doc = "Field `RESETNPADF` writer - Reset pad flag"]
pub type ResetnpadfW<'a, REG> = crate::BitWriter<'a, REG, Resetnpadf>;
impl<'a, REG> ResetnpadfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset detected since last time this flag was cleared."]
    #[inline(always)]
    pub fn resetnpadf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Resetnpadf::Resetnpadf0)
    }
    #[doc = "Reset pad wakeup caused a wakeup or reset event since the last time this bit was cleared. Write 1 to clear."]
    #[inline(always)]
    pub fn resetnpadf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Resetnpadf::Resetnpadf1)
    }
}
#[doc = "Deep powerdown wakeup flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deeppdf {
    #[doc = "0: No deep powerdown wakeup since last time flag was cleared."]
    Deeppdf0 = 0,
    #[doc = "1: Deep powerdown was entered since the last time this flag was cleared. Write 1 to clear"]
    Deeppdf1 = 1,
}
impl From<Deeppdf> for bool {
    #[inline(always)]
    fn from(variant: Deeppdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPPDF` reader - Deep powerdown wakeup flag"]
pub type DeeppdfR = crate::BitReader<Deeppdf>;
impl DeeppdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Deeppdf {
        match self.bits {
            false => Deeppdf::Deeppdf0,
            true => Deeppdf::Deeppdf1,
        }
    }
    #[doc = "No deep powerdown wakeup since last time flag was cleared."]
    #[inline(always)]
    pub fn is_deeppdf_0(&self) -> bool {
        *self == Deeppdf::Deeppdf0
    }
    #[doc = "Deep powerdown was entered since the last time this flag was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn is_deeppdf_1(&self) -> bool {
        *self == Deeppdf::Deeppdf1
    }
}
#[doc = "Field `DEEPPDF` writer - Deep powerdown wakeup flag"]
pub type DeeppdfW<'a, REG> = crate::BitWriter<'a, REG, Deeppdf>;
impl<'a, REG> DeeppdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No deep powerdown wakeup since last time flag was cleared."]
    #[inline(always)]
    pub fn deeppdf_0(self) -> &'a mut crate::W<REG> {
        self.variant(Deeppdf::Deeppdf0)
    }
    #[doc = "Deep powerdown was entered since the last time this flag was cleared. Write 1 to clear"]
    #[inline(always)]
    pub fn deeppdf_1(self) -> &'a mut crate::W<REG> {
        self.variant(Deeppdf::Deeppdf1)
    }
}
impl R {
    #[doc = "Bit 16 - vddcore POR Flag"]
    #[inline(always)]
    pub fn porcoref(&self) -> PorcorefR {
        PorcorefR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - vdd1v8 power on reset flag"]
    #[inline(always)]
    pub fn por1v8f(&self) -> Por1v8fR {
        Por1v8fR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - vdd_ao18 power on reset flag"]
    #[inline(always)]
    pub fn porao18f(&self) -> Porao18fR {
        Porao18fR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    pub fn lvdcoref(&self) -> LvdcorefR {
        LvdcorefR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    pub fn hvdcoref(&self) -> HvdcorefR {
        HvdcorefR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    pub fn hvd1v8f(&self) -> Hvd1v8fR {
        Hvd1v8fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC Wakeup from deep powerdown mode flag."]
    #[inline(always)]
    pub fn rtcf(&self) -> RtcfR {
        RtcfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PMC Auto Wakeup Interrupt flag."]
    #[inline(always)]
    pub fn autowkf(&self) -> AutowkfR {
        AutowkfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PMIC interrupt pin flag"]
    #[inline(always)]
    pub fn intnpadf(&self) -> IntnpadfR {
        IntnpadfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset pad flag"]
    #[inline(always)]
    pub fn resetnpadf(&self) -> ResetnpadfR {
        ResetnpadfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deep powerdown wakeup flag"]
    #[inline(always)]
    pub fn deeppdf(&self) -> DeeppdfR {
        DeeppdfR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLAGS")
            .field("porcoref", &self.porcoref())
            .field("por1v8f", &self.por1v8f())
            .field("porao18f", &self.porao18f())
            .field("lvdcoref", &self.lvdcoref())
            .field("hvdcoref", &self.hvdcoref())
            .field("hvd1v8f", &self.hvd1v8f())
            .field("rtcf", &self.rtcf())
            .field("autowkf", &self.autowkf())
            .field("intnpadf", &self.intnpadf())
            .field("resetnpadf", &self.resetnpadf())
            .field("deeppdf", &self.deeppdf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - vddcore POR Flag"]
    #[inline(always)]
    #[must_use]
    pub fn porcoref(&mut self) -> PorcorefW<FlagsSpec> {
        PorcorefW::new(self, 16)
    }
    #[doc = "Bit 17 - vdd1v8 power on reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn por1v8f(&mut self) -> Por1v8fW<FlagsSpec> {
        Por1v8fW::new(self, 17)
    }
    #[doc = "Bit 18 - vdd_ao18 power on reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porao18f(&mut self) -> Porao18fW<FlagsSpec> {
        Porao18fW::new(self, 18)
    }
    #[doc = "Bit 20 - vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcoref(&mut self) -> LvdcorefW<FlagsSpec> {
        LvdcorefW::new(self, 20)
    }
    #[doc = "Bit 22 - vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcoref(&mut self) -> HvdcorefW<FlagsSpec> {
        HvdcorefW::new(self, 22)
    }
    #[doc = "Bit 24 - vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8f(&mut self) -> Hvd1v8fW<FlagsSpec> {
        Hvd1v8fW::new(self, 24)
    }
    #[doc = "Bit 27 - RTC Wakeup from deep powerdown mode flag."]
    #[inline(always)]
    #[must_use]
    pub fn rtcf(&mut self) -> RtcfW<FlagsSpec> {
        RtcfW::new(self, 27)
    }
    #[doc = "Bit 28 - PMC Auto Wakeup Interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn autowkf(&mut self) -> AutowkfW<FlagsSpec> {
        AutowkfW::new(self, 28)
    }
    #[doc = "Bit 29 - PMIC interrupt pin flag"]
    #[inline(always)]
    #[must_use]
    pub fn intnpadf(&mut self) -> IntnpadfW<FlagsSpec> {
        IntnpadfW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset pad flag"]
    #[inline(always)]
    #[must_use]
    pub fn resetnpadf(&mut self) -> ResetnpadfW<FlagsSpec> {
        ResetnpadfW::new(self, 30)
    }
    #[doc = "Bit 31 - Deep powerdown wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn deeppdf(&mut self) -> DeeppdfW<FlagsSpec> {
        DeeppdfW::new(self, 31)
    }
}
#[doc = "Wakeup, interrupt, and reset flags\n\nYou can [`read`](crate::Reg::read) this register and get [`flags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagsSpec;
impl crate::RegisterSpec for FlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flags::R`](R) reader structure"]
impl crate::Readable for FlagsSpec {}
#[doc = "`write(|w| ..)` method takes [`flags::W`](W) writer structure"]
impl crate::Writable for FlagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLAGS to value 0x0017_0000"]
impl crate::Resettable for FlagsSpec {
    const RESET_VALUE: u32 = 0x0017_0000;
}
