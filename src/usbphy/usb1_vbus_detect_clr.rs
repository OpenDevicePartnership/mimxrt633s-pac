#[doc = "Register `USB1_VBUS_DETECT_CLR` reader"]
pub type R = crate::R<Usb1VbusDetectClrSpec>;
#[doc = "Register `USB1_VBUS_DETECT_CLR` writer"]
pub type W = crate::W<Usb1VbusDetectClrSpec>;
#[doc = "Sets the threshold for the VBUSVALID comparator\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VbusvalidThresh {
    #[doc = "0: 4.0V"]
    VbusvalidThresh0 = 0,
    #[doc = "1: 4.1V"]
    VbusvalidThresh1 = 1,
    #[doc = "2: 4.2V"]
    VbusvalidThresh2 = 2,
    #[doc = "3: 4.3V"]
    VbusvalidThresh3 = 3,
    #[doc = "4: 4.4V(Default)"]
    VbusvalidThresh4 = 4,
    #[doc = "5: 4.5V"]
    VbusvalidThresh5 = 5,
    #[doc = "6: 4.6V"]
    VbusvalidThresh6 = 6,
    #[doc = "7: 4.7V"]
    VbusvalidThresh7 = 7,
}
impl From<VbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(variant: VbusvalidThresh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VbusvalidThresh {
    type Ux = u8;
}
impl crate::IsEnum for VbusvalidThresh {}
#[doc = "Field `VBUSVALID_THRESH` reader - Sets the threshold for the VBUSVALID comparator"]
pub type VbusvalidThreshR = crate::FieldReader<VbusvalidThresh>;
impl VbusvalidThreshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusvalidThresh {
        match self.bits {
            0 => VbusvalidThresh::VbusvalidThresh0,
            1 => VbusvalidThresh::VbusvalidThresh1,
            2 => VbusvalidThresh::VbusvalidThresh2,
            3 => VbusvalidThresh::VbusvalidThresh3,
            4 => VbusvalidThresh::VbusvalidThresh4,
            5 => VbusvalidThresh::VbusvalidThresh5,
            6 => VbusvalidThresh::VbusvalidThresh6,
            7 => VbusvalidThresh::VbusvalidThresh7,
            _ => unreachable!(),
        }
    }
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_0(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh0
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_1(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh1
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_2(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh2
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_3(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh3
    }
    #[doc = "4.4V(Default)"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_4(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh4
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_5(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh5
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_6(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh6
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn is_vbusvalid_thresh_7(&self) -> bool {
        *self == VbusvalidThresh::VbusvalidThresh7
    }
}
#[doc = "Field `VBUSVALID_THRESH` writer - Sets the threshold for the VBUSVALID comparator"]
pub type VbusvalidThreshW<'a, REG> = crate::FieldWriter<'a, REG, 3, VbusvalidThresh, crate::Safe>;
impl<'a, REG> VbusvalidThreshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh0)
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh1)
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_2(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh2)
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_3(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh3)
    }
    #[doc = "4.4V(Default)"]
    #[inline(always)]
    pub fn vbusvalid_thresh_4(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh4)
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_5(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh5)
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_6(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh6)
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn vbusvalid_thresh_7(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidThresh::VbusvalidThresh7)
    }
}
#[doc = "VBUS detect signal override enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusOverrideEn {
    #[doc = "0: Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VbusOverrideEn0 = 0,
    #[doc = "1: Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VbusOverrideEn1 = 1,
}
impl From<VbusOverrideEn> for bool {
    #[inline(always)]
    fn from(variant: VbusOverrideEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` reader - VBUS detect signal override enable"]
pub type VbusOverrideEnR = crate::BitReader<VbusOverrideEn>;
impl VbusOverrideEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusOverrideEn {
        match self.bits {
            false => VbusOverrideEn::VbusOverrideEn0,
            true => VbusOverrideEn::VbusOverrideEn1,
        }
    }
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn is_vbus_override_en_0(&self) -> bool {
        *self == VbusOverrideEn::VbusOverrideEn0
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn is_vbus_override_en_1(&self) -> bool {
        *self == VbusOverrideEn::VbusOverrideEn1
    }
}
#[doc = "Field `VBUS_OVERRIDE_EN` writer - VBUS detect signal override enable"]
pub type VbusOverrideEnW<'a, REG> = crate::BitWriter<'a, REG, VbusOverrideEn>;
impl<'a, REG> VbusOverrideEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    #[inline(always)]
    pub fn vbus_override_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOverrideEn::VbusOverrideEn0)
    }
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    #[inline(always)]
    pub fn vbus_override_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOverrideEn::VbusOverrideEn1)
    }
}
#[doc = "Field `SESSEND_OVERRIDE` reader - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub type SessendOverrideR = crate::BitReader;
#[doc = "Field `SESSEND_OVERRIDE` writer - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub type SessendOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALID_OVERRIDE` reader - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub type BvalidOverrideR = crate::BitReader;
#[doc = "Field `BVALID_OVERRIDE` writer - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub type BvalidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALID_OVERRIDE` reader - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub type AvalidOverrideR = crate::BitReader;
#[doc = "Field `AVALID_OVERRIDE` writer - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
pub type AvalidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSVALID_OVERRIDE` reader - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
pub type VbusvalidOverrideR = crate::BitReader;
#[doc = "Field `VBUSVALID_OVERRIDE` writer - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
pub type VbusvalidOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusvalidSel {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VbusvalidSel0 = 0,
    #[doc = "1: Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VbusvalidSel1 = 1,
}
impl From<VbusvalidSel> for bool {
    #[inline(always)]
    fn from(variant: VbusvalidSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVALID_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusvalidSelR = crate::BitReader<VbusvalidSel>;
impl VbusvalidSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusvalidSel {
        match self.bits {
            false => VbusvalidSel::VbusvalidSel0,
            true => VbusvalidSel::VbusvalidSel1,
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn is_vbusvalid_sel_0(&self) -> bool {
        *self == VbusvalidSel::VbusvalidSel0
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn is_vbusvalid_sel_1(&self) -> bool {
        *self == VbusvalidSel::VbusvalidSel1
    }
}
#[doc = "Field `VBUSVALID_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusvalidSelW<'a, REG> = crate::BitWriter<'a, REG, VbusvalidSel>;
impl<'a, REG> VbusvalidSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn vbusvalid_sel_0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidSel::VbusvalidSel0)
    }
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel_1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidSel::VbusvalidSel1)
    }
}
#[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VbusSourceSel {
    #[doc = "0: Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VbusSourceSel0 = 0,
    #[doc = "1: Use the Session Valid comparator results for signal reported to the USB controller"]
    VbusSourceSel1 = 1,
    #[doc = "2: Use the Session Valid comparator results for signal reported to the USB controller"]
    VbusSourceSel2 = 2,
}
impl From<VbusSourceSel> for u8 {
    #[inline(always)]
    fn from(variant: VbusSourceSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VbusSourceSel {
    type Ux = u8;
}
impl crate::IsEnum for VbusSourceSel {}
#[doc = "Field `VBUS_SOURCE_SEL` reader - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusSourceSelR = crate::FieldReader<VbusSourceSel>;
impl VbusSourceSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VbusSourceSel> {
        match self.bits {
            0 => Some(VbusSourceSel::VbusSourceSel0),
            1 => Some(VbusSourceSel::VbusSourceSel1),
            2 => Some(VbusSourceSel::VbusSourceSel2),
            _ => None,
        }
    }
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn is_vbus_source_sel_0(&self) -> bool {
        *self == VbusSourceSel::VbusSourceSel0
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn is_vbus_source_sel_1(&self) -> bool {
        *self == VbusSourceSel::VbusSourceSel1
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn is_vbus_source_sel_2(&self) -> bool {
        *self == VbusSourceSel::VbusSourceSel2
    }
}
#[doc = "Field `VBUS_SOURCE_SEL` writer - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
pub type VbusSourceSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, VbusSourceSel>;
impl<'a, REG> VbusSourceSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    #[inline(always)]
    pub fn vbus_source_sel_0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSourceSel::VbusSourceSel0)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel_1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSourceSel::VbusSourceSel1)
    }
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel_2(self) -> &'a mut crate::W<REG> {
        self.variant(VbusSourceSel::VbusSourceSel2)
    }
}
#[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusvalidToSessvalid {
    #[doc = "0: Use the VBUS_VALID comparator for VBUS_VALID results"]
    VbusvalidToSessvalid0 = 0,
    #[doc = "1: Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VbusvalidToSessvalid1 = 1,
}
impl From<VbusvalidToSessvalid> for bool {
    #[inline(always)]
    fn from(variant: VbusvalidToSessvalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` reader - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
pub type VbusvalidToSessvalidR = crate::BitReader<VbusvalidToSessvalid>;
impl VbusvalidToSessvalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusvalidToSessvalid {
        match self.bits {
            false => VbusvalidToSessvalid::VbusvalidToSessvalid0,
            true => VbusvalidToSessvalid::VbusvalidToSessvalid1,
        }
    }
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn is_vbusvalid_to_sessvalid_0(&self) -> bool {
        *self == VbusvalidToSessvalid::VbusvalidToSessvalid0
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn is_vbusvalid_to_sessvalid_1(&self) -> bool {
        *self == VbusvalidToSessvalid::VbusvalidToSessvalid1
    }
}
#[doc = "Field `VBUSVALID_TO_SESSVALID` writer - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
pub type VbusvalidToSessvalidW<'a, REG> = crate::BitWriter<'a, REG, VbusvalidToSessvalid>;
impl<'a, REG> VbusvalidToSessvalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid_0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidToSessvalid::VbusvalidToSessvalid0)
    }
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid_1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusvalidToSessvalid::VbusvalidToSessvalid1)
    }
}
#[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrupCmps {
    #[doc = "0: Powers down the VBUS_VALID comparator"]
    PwrupCmps0 = 0,
    #[doc = "1: Enables the VBUS_VALID comparator (default)"]
    PwrupCmps1 = 1,
}
impl From<PwrupCmps> for bool {
    #[inline(always)]
    fn from(variant: PwrupCmps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUP_CMPS` reader - Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
pub type PwrupCmpsR = crate::BitReader<PwrupCmps>;
impl PwrupCmpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrupCmps {
        match self.bits {
            false => PwrupCmps::PwrupCmps0,
            true => PwrupCmps::PwrupCmps1,
        }
    }
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn is_pwrup_cmps_0(&self) -> bool {
        *self == PwrupCmps::PwrupCmps0
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline(always)]
    pub fn is_pwrup_cmps_1(&self) -> bool {
        *self == PwrupCmps::PwrupCmps1
    }
}
#[doc = "Field `PWRUP_CMPS` writer - Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
pub type PwrupCmpsW<'a, REG> = crate::BitWriter<'a, REG, PwrupCmps>;
impl<'a, REG> PwrupCmpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Powers down the VBUS_VALID comparator"]
    #[inline(always)]
    pub fn pwrup_cmps_0(self) -> &'a mut crate::W<REG> {
        self.variant(PwrupCmps::PwrupCmps0)
    }
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    #[inline(always)]
    pub fn pwrup_cmps_1(self) -> &'a mut crate::W<REG> {
        self.variant(PwrupCmps::PwrupCmps1)
    }
}
#[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DischargeVbus {
    #[doc = "0: VBUS discharge resistor is disabled (Default)"]
    DischargeVbus0 = 0,
    #[doc = "1: VBUS discharge resistor is enabled"]
    DischargeVbus1 = 1,
}
impl From<DischargeVbus> for bool {
    #[inline(always)]
    fn from(variant: DischargeVbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCHARGE_VBUS` reader - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
pub type DischargeVbusR = crate::BitReader<DischargeVbus>;
impl DischargeVbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DischargeVbus {
        match self.bits {
            false => DischargeVbus::DischargeVbus0,
            true => DischargeVbus::DischargeVbus1,
        }
    }
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn is_discharge_vbus_0(&self) -> bool {
        *self == DischargeVbus::DischargeVbus0
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline(always)]
    pub fn is_discharge_vbus_1(&self) -> bool {
        *self == DischargeVbus::DischargeVbus1
    }
}
#[doc = "Field `DISCHARGE_VBUS` writer - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
pub type DischargeVbusW<'a, REG> = crate::BitWriter<'a, REG, DischargeVbus>;
impl<'a, REG> DischargeVbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    #[inline(always)]
    pub fn discharge_vbus_0(self) -> &'a mut crate::W<REG> {
        self.variant(DischargeVbus::DischargeVbus0)
    }
    #[doc = "VBUS discharge resistor is enabled"]
    #[inline(always)]
    pub fn discharge_vbus_1(self) -> &'a mut crate::W<REG> {
        self.variant(DischargeVbus::DischargeVbus1)
    }
}
#[doc = "Enables resistors used for an older method of resistive battery charger detection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnChargerResistor {
    #[doc = "0: Disable resistive charger detection resistors on USB_DP and USB_DP"]
    EnChargerResistor0 = 0,
    #[doc = "1: Enable resistive charger detection resistors on USB_DP and USB_DP"]
    EnChargerResistor1 = 1,
}
impl From<EnChargerResistor> for bool {
    #[inline(always)]
    fn from(variant: EnChargerResistor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN_CHARGER_RESISTOR` reader - Enables resistors used for an older method of resistive battery charger detection"]
pub type EnChargerResistorR = crate::BitReader<EnChargerResistor>;
impl EnChargerResistorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnChargerResistor {
        match self.bits {
            false => EnChargerResistor::EnChargerResistor0,
            true => EnChargerResistor::EnChargerResistor1,
        }
    }
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline(always)]
    pub fn is_en_charger_resistor_0(&self) -> bool {
        *self == EnChargerResistor::EnChargerResistor0
    }
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline(always)]
    pub fn is_en_charger_resistor_1(&self) -> bool {
        *self == EnChargerResistor::EnChargerResistor1
    }
}
#[doc = "Field `EN_CHARGER_RESISTOR` writer - Enables resistors used for an older method of resistive battery charger detection"]
pub type EnChargerResistorW<'a, REG> = crate::BitWriter<'a, REG, EnChargerResistor>;
impl<'a, REG> EnChargerResistorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline(always)]
    pub fn en_charger_resistor_0(self) -> &'a mut crate::W<REG> {
        self.variant(EnChargerResistor::EnChargerResistor0)
    }
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    #[inline(always)]
    pub fn en_charger_resistor_1(self) -> &'a mut crate::W<REG> {
        self.variant(EnChargerResistor::EnChargerResistor1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VbusvalidThreshR {
        VbusvalidThreshR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&self) -> VbusOverrideEnR {
        VbusOverrideEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn sessend_override(&self) -> SessendOverrideR {
        SessendOverrideR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn bvalid_override(&self) -> BvalidOverrideR {
        BvalidOverrideR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn avalid_override(&self) -> AvalidOverrideR {
        AvalidOverrideR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
    #[inline(always)]
    pub fn vbusvalid_override(&self) -> VbusvalidOverrideR {
        VbusvalidOverrideR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&self) -> VbusvalidSelR {
        VbusvalidSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&self) -> VbusSourceSelR {
        VbusSourceSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&self) -> VbusvalidToSessvalidR {
        VbusvalidToSessvalidR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub fn pwrup_cmps(&self) -> PwrupCmpsR {
        PwrupCmpsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DischargeVbusR {
        DischargeVbusR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub fn en_charger_resistor(&self) -> EnChargerResistorR {
        EnChargerResistorR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_CLR")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("en_charger_resistor", &self.en_charger_resistor())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&mut self) -> VbusvalidThreshW<Usb1VbusDetectClrSpec> {
        VbusvalidThreshW::new(self, 0)
    }
    #[doc = "Bit 3 - VBUS detect signal override enable"]
    #[inline(always)]
    pub fn vbus_override_en(&mut self) -> VbusOverrideEnW<Usb1VbusDetectClrSpec> {
        VbusOverrideEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn sessend_override(&mut self) -> SessendOverrideW<Usb1VbusDetectClrSpec> {
        SessendOverrideW::new(self, 4)
    }
    #[doc = "Bit 5 - Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn bvalid_override(&mut self) -> BvalidOverrideW<Usb1VbusDetectClrSpec> {
        BvalidOverrideW::new(self, 5)
    }
    #[doc = "Bit 6 - Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\]
if USB_VBUS_DETECT\\[3\\]
is set to value 1'b1"]
    #[inline(always)]
    pub fn avalid_override(&mut self) -> AvalidOverrideW<Usb1VbusDetectClrSpec> {
        AvalidOverrideW::new(self, 6)
    }
    #[doc = "Bit 7 - Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\]
is set to 1'b1"]
    #[inline(always)]
    pub fn vbusvalid_override(&mut self) -> VbusvalidOverrideW<Usb1VbusDetectClrSpec> {
        VbusvalidOverrideW::new(self, 7)
    }
    #[doc = "Bit 8 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbusvalid_sel(&mut self) -> VbusvalidSelW<Usb1VbusDetectClrSpec> {
        VbusvalidSelW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub fn vbus_source_sel(&mut self) -> VbusSourceSelW<Usb1VbusDetectClrSpec> {
        VbusSourceSelW::new(self, 9)
    }
    #[doc = "Bit 18 - Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\]
between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub fn vbusvalid_to_sessvalid(&mut self) -> VbusvalidToSessvalidW<Usb1VbusDetectClrSpec> {
        VbusvalidToSessvalidW::new(self, 18)
    }
    #[doc = "Bit 20 - Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub fn pwrup_cmps(&mut self) -> PwrupCmpsW<Usb1VbusDetectClrSpec> {
        PwrupCmpsW::new(self, 20)
    }
    #[doc = "Bit 26 - Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub fn discharge_vbus(&mut self) -> DischargeVbusW<Usb1VbusDetectClrSpec> {
        DischargeVbusW::new(self, 26)
    }
    #[doc = "Bit 31 - Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub fn en_charger_resistor(&mut self) -> EnChargerResistorW<Usb1VbusDetectClrSpec> {
        EnChargerResistorW::new(self, 31)
    }
}
#[doc = "USB PHY VBUS Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_vbus_detect_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_vbus_detect_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1VbusDetectClrSpec;
impl crate::RegisterSpec for Usb1VbusDetectClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_vbus_detect_clr::R`](R) reader structure"]
impl crate::Readable for Usb1VbusDetectClrSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1_vbus_detect_clr::W`](W) writer structure"]
impl crate::Writable for Usb1VbusDetectClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT_CLR to value 0x0070_0004"]
impl crate::Resettable for Usb1VbusDetectClrSpec {
    const RESET_VALUE: u32 = 0x0070_0004;
}
