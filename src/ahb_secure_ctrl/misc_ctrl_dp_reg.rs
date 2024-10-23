#[doc = "Register `MISC_CTRL_DP_REG` reader"]
pub type R = crate::R<MiscCtrlDpRegSpec>;
#[doc = "Register `MISC_CTRL_DP_REG` writer"]
pub type W = crate::W<MiscCtrlDpRegSpec>;
#[doc = "Write lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WriteLock {
    #[doc = "1: Restrictive mode."]
    Restricted = 1,
    #[doc = "2: Secure control registers can be written."]
    Accessible = 2,
}
impl From<WriteLock> for u8 {
    #[inline(always)]
    fn from(variant: WriteLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteLock {
    type Ux = u8;
}
impl crate::IsEnum for WriteLock {}
#[doc = "Field `WRITE_LOCK` reader - Write lock."]
pub type WriteLockR = crate::FieldReader<WriteLock>;
impl WriteLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WriteLock> {
        match self.bits {
            1 => Some(WriteLock::Restricted),
            2 => Some(WriteLock::Accessible),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == WriteLock::Restricted
    }
    #[doc = "Secure control registers can be written."]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == WriteLock::Accessible
    }
}
#[doc = "Field `WRITE_LOCK` writer - Write lock."]
pub type WriteLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, WriteLock>;
impl<'a, REG> WriteLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut crate::W<REG> {
        self.variant(WriteLock::Restricted)
    }
    #[doc = "Secure control registers can be written."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut crate::W<REG> {
        self.variant(WriteLock::Accessible)
    }
}
#[doc = "AHB bus matrix enable secure checking.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnableSecureChecking {
    #[doc = "1: Restrictive mode."]
    Enable = 1,
    #[doc = "2: Disable check."]
    Disable = 2,
}
impl From<EnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(variant: EnableSecureChecking) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EnableSecureChecking {
    type Ux = u8;
}
impl crate::IsEnum for EnableSecureChecking {}
#[doc = "Field `ENABLE_SECURE_CHECKING` reader - AHB bus matrix enable secure checking."]
pub type EnableSecureCheckingR = crate::FieldReader<EnableSecureChecking>;
impl EnableSecureCheckingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EnableSecureChecking> {
        match self.bits {
            1 => Some(EnableSecureChecking::Enable),
            2 => Some(EnableSecureChecking::Disable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnableSecureChecking::Enable
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnableSecureChecking::Disable
    }
}
#[doc = "Field `ENABLE_SECURE_CHECKING` writer - AHB bus matrix enable secure checking."]
pub type EnableSecureCheckingW<'a, REG> = crate::FieldWriter<'a, REG, 2, EnableSecureChecking>;
impl<'a, REG> EnableSecureCheckingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSecureChecking::Enable)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSecureChecking::Disable)
    }
}
#[doc = "AHB bus matrix enable secure privilege check.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnableSPrivCheck {
    #[doc = "1: Restrictive mode."]
    Enable = 1,
    #[doc = "2: Disable check."]
    Disable = 2,
}
impl From<EnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(variant: EnableSPrivCheck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EnableSPrivCheck {
    type Ux = u8;
}
impl crate::IsEnum for EnableSPrivCheck {}
#[doc = "Field `ENABLE_S_PRIV_CHECK` reader - AHB bus matrix enable secure privilege check."]
pub type EnableSPrivCheckR = crate::FieldReader<EnableSPrivCheck>;
impl EnableSPrivCheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EnableSPrivCheck> {
        match self.bits {
            1 => Some(EnableSPrivCheck::Enable),
            2 => Some(EnableSPrivCheck::Disable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnableSPrivCheck::Enable
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnableSPrivCheck::Disable
    }
}
#[doc = "Field `ENABLE_S_PRIV_CHECK` writer - AHB bus matrix enable secure privilege check."]
pub type EnableSPrivCheckW<'a, REG> = crate::FieldWriter<'a, REG, 2, EnableSPrivCheck>;
impl<'a, REG> EnableSPrivCheckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSPrivCheck::Enable)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableSPrivCheck::Disable)
    }
}
#[doc = "AHB bus matrix enable non-secure privilege check.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnableNsPrivCheck {
    #[doc = "1: Restrictive mode."]
    Enable = 1,
    #[doc = "2: Disable check."]
    Disable = 2,
}
impl From<EnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(variant: EnableNsPrivCheck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EnableNsPrivCheck {
    type Ux = u8;
}
impl crate::IsEnum for EnableNsPrivCheck {}
#[doc = "Field `ENABLE_NS_PRIV_CHECK` reader - AHB bus matrix enable non-secure privilege check."]
pub type EnableNsPrivCheckR = crate::FieldReader<EnableNsPrivCheck>;
impl EnableNsPrivCheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EnableNsPrivCheck> {
        match self.bits {
            1 => Some(EnableNsPrivCheck::Enable),
            2 => Some(EnableNsPrivCheck::Disable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnableNsPrivCheck::Enable
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnableNsPrivCheck::Disable
    }
}
#[doc = "Field `ENABLE_NS_PRIV_CHECK` writer - AHB bus matrix enable non-secure privilege check."]
pub type EnableNsPrivCheckW<'a, REG> = crate::FieldWriter<'a, REG, 2, EnableNsPrivCheck>;
impl<'a, REG> EnableNsPrivCheckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableNsPrivCheck::Enable)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnableNsPrivCheck::Disable)
    }
}
#[doc = "Disable secure violation abort.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DisableViolationAbort {
    #[doc = "1: Violation assert secure_violation_irq."]
    Disable = 1,
    #[doc = "2: Violation causes abort."]
    Enable = 2,
}
impl From<DisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(variant: DisableViolationAbort) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DisableViolationAbort {
    type Ux = u8;
}
impl crate::IsEnum for DisableViolationAbort {}
#[doc = "Field `DISABLE_VIOLATION_ABORT` reader - Disable secure violation abort."]
pub type DisableViolationAbortR = crate::FieldReader<DisableViolationAbort>;
impl DisableViolationAbortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DisableViolationAbort> {
        match self.bits {
            1 => Some(DisableViolationAbort::Disable),
            2 => Some(DisableViolationAbort::Enable),
            _ => None,
        }
    }
    #[doc = "Violation assert secure_violation_irq."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DisableViolationAbort::Disable
    }
    #[doc = "Violation causes abort."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DisableViolationAbort::Enable
    }
}
#[doc = "Field `DISABLE_VIOLATION_ABORT` writer - Disable secure violation abort."]
pub type DisableViolationAbortW<'a, REG> = crate::FieldWriter<'a, REG, 2, DisableViolationAbort>;
impl<'a, REG> DisableViolationAbortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Violation assert secure_violation_irq."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DisableViolationAbort::Disable)
    }
    #[doc = "Violation causes abort."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DisableViolationAbort::Enable)
    }
}
#[doc = "Disable simple master strict mode.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DisableSimpleMasterStrictMode {
    #[doc = "1: Simple master in tier mode."]
    TierMode = 1,
    #[doc = "2: Simple master in strict mode."]
    StrictMode = 2,
}
impl From<DisableSimpleMasterStrictMode> for u8 {
    #[inline(always)]
    fn from(variant: DisableSimpleMasterStrictMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DisableSimpleMasterStrictMode {
    type Ux = u8;
}
impl crate::IsEnum for DisableSimpleMasterStrictMode {}
#[doc = "Field `DISABLE_SIMPLE_MASTER_STRICT_MODE` reader - Disable simple master strict mode."]
pub type DisableSimpleMasterStrictModeR = crate::FieldReader<DisableSimpleMasterStrictMode>;
impl DisableSimpleMasterStrictModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DisableSimpleMasterStrictMode> {
        match self.bits {
            1 => Some(DisableSimpleMasterStrictMode::TierMode),
            2 => Some(DisableSimpleMasterStrictMode::StrictMode),
            _ => None,
        }
    }
    #[doc = "Simple master in tier mode."]
    #[inline(always)]
    pub fn is_tier_mode(&self) -> bool {
        *self == DisableSimpleMasterStrictMode::TierMode
    }
    #[doc = "Simple master in strict mode."]
    #[inline(always)]
    pub fn is_strict_mode(&self) -> bool {
        *self == DisableSimpleMasterStrictMode::StrictMode
    }
}
#[doc = "Field `DISABLE_SIMPLE_MASTER_STRICT_MODE` writer - Disable simple master strict mode."]
pub type DisableSimpleMasterStrictModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, DisableSimpleMasterStrictMode>;
impl<'a, REG> DisableSimpleMasterStrictModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Simple master in tier mode."]
    #[inline(always)]
    pub fn tier_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DisableSimpleMasterStrictMode::TierMode)
    }
    #[doc = "Simple master in strict mode."]
    #[inline(always)]
    pub fn strict_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DisableSimpleMasterStrictMode::StrictMode)
    }
}
#[doc = "Disable smart master strict mode.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DisableSmartMasterStrictMode {
    #[doc = "1: Smart master in tier mode."]
    TierMode = 1,
    #[doc = "2: Smart master in strict mode."]
    StrictMode = 2,
}
impl From<DisableSmartMasterStrictMode> for u8 {
    #[inline(always)]
    fn from(variant: DisableSmartMasterStrictMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DisableSmartMasterStrictMode {
    type Ux = u8;
}
impl crate::IsEnum for DisableSmartMasterStrictMode {}
#[doc = "Field `DISABLE_SMART_MASTER_STRICT_MODE` reader - Disable smart master strict mode."]
pub type DisableSmartMasterStrictModeR = crate::FieldReader<DisableSmartMasterStrictMode>;
impl DisableSmartMasterStrictModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DisableSmartMasterStrictMode> {
        match self.bits {
            1 => Some(DisableSmartMasterStrictMode::TierMode),
            2 => Some(DisableSmartMasterStrictMode::StrictMode),
            _ => None,
        }
    }
    #[doc = "Smart master in tier mode."]
    #[inline(always)]
    pub fn is_tier_mode(&self) -> bool {
        *self == DisableSmartMasterStrictMode::TierMode
    }
    #[doc = "Smart master in strict mode."]
    #[inline(always)]
    pub fn is_strict_mode(&self) -> bool {
        *self == DisableSmartMasterStrictMode::StrictMode
    }
}
#[doc = "Field `DISABLE_SMART_MASTER_STRICT_MODE` writer - Disable smart master strict mode."]
pub type DisableSmartMasterStrictModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, DisableSmartMasterStrictMode>;
impl<'a, REG> DisableSmartMasterStrictModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Smart master in tier mode."]
    #[inline(always)]
    pub fn tier_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DisableSmartMasterStrictMode::TierMode)
    }
    #[doc = "Smart master in strict mode."]
    #[inline(always)]
    pub fn strict_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DisableSmartMasterStrictMode::StrictMode)
    }
}
#[doc = "Disable IDAU.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IdauAllNs {
    #[doc = "1: IDAU is disabled."]
    Disable = 1,
    #[doc = "2: IDAU is enabled."]
    Enable = 2,
}
impl From<IdauAllNs> for u8 {
    #[inline(always)]
    fn from(variant: IdauAllNs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IdauAllNs {
    type Ux = u8;
}
impl crate::IsEnum for IdauAllNs {}
#[doc = "Field `IDAU_ALL_NS` reader - Disable IDAU."]
pub type IdauAllNsR = crate::FieldReader<IdauAllNs>;
impl IdauAllNsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IdauAllNs> {
        match self.bits {
            1 => Some(IdauAllNs::Disable),
            2 => Some(IdauAllNs::Enable),
            _ => None,
        }
    }
    #[doc = "IDAU is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IdauAllNs::Disable
    }
    #[doc = "IDAU is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IdauAllNs::Enable
    }
}
#[doc = "Field `IDAU_ALL_NS` writer - Disable IDAU."]
pub type IdauAllNsW<'a, REG> = crate::FieldWriter<'a, REG, 2, IdauAllNs>;
impl<'a, REG> IdauAllNsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IDAU is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IdauAllNs::Disable)
    }
    #[doc = "IDAU is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IdauAllNs::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Write lock."]
    #[inline(always)]
    pub fn write_lock(&self) -> WriteLockR {
        WriteLockR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AHB bus matrix enable secure checking."]
    #[inline(always)]
    pub fn enable_secure_checking(&self) -> EnableSecureCheckingR {
        EnableSecureCheckingR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - AHB bus matrix enable secure privilege check."]
    #[inline(always)]
    pub fn enable_s_priv_check(&self) -> EnableSPrivCheckR {
        EnableSPrivCheckR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - AHB bus matrix enable non-secure privilege check."]
    #[inline(always)]
    pub fn enable_ns_priv_check(&self) -> EnableNsPrivCheckR {
        EnableNsPrivCheckR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline(always)]
    pub fn disable_violation_abort(&self) -> DisableViolationAbortR {
        DisableViolationAbortR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline(always)]
    pub fn disable_simple_master_strict_mode(&self) -> DisableSimpleMasterStrictModeR {
        DisableSimpleMasterStrictModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline(always)]
    pub fn disable_smart_master_strict_mode(&self) -> DisableSmartMasterStrictModeR {
        DisableSmartMasterStrictModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline(always)]
    pub fn idau_all_ns(&self) -> IdauAllNsR {
        IdauAllNsR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CTRL_DP_REG")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field(
                "disable_simple_master_strict_mode",
                &self.disable_simple_master_strict_mode(),
            )
            .field(
                "disable_smart_master_strict_mode",
                &self.disable_smart_master_strict_mode(),
            )
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Write lock."]
    #[inline(always)]
    #[must_use]
    pub fn write_lock(&mut self) -> WriteLockW<MiscCtrlDpRegSpec> {
        WriteLockW::new(self, 0)
    }
    #[doc = "Bits 2:3 - AHB bus matrix enable secure checking."]
    #[inline(always)]
    #[must_use]
    pub fn enable_secure_checking(&mut self) -> EnableSecureCheckingW<MiscCtrlDpRegSpec> {
        EnableSecureCheckingW::new(self, 2)
    }
    #[doc = "Bits 4:5 - AHB bus matrix enable secure privilege check."]
    #[inline(always)]
    #[must_use]
    pub fn enable_s_priv_check(&mut self) -> EnableSPrivCheckW<MiscCtrlDpRegSpec> {
        EnableSPrivCheckW::new(self, 4)
    }
    #[doc = "Bits 6:7 - AHB bus matrix enable non-secure privilege check."]
    #[inline(always)]
    #[must_use]
    pub fn enable_ns_priv_check(&mut self) -> EnableNsPrivCheckW<MiscCtrlDpRegSpec> {
        EnableNsPrivCheckW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline(always)]
    #[must_use]
    pub fn disable_violation_abort(&mut self) -> DisableViolationAbortW<MiscCtrlDpRegSpec> {
        DisableViolationAbortW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline(always)]
    #[must_use]
    pub fn disable_simple_master_strict_mode(
        &mut self,
    ) -> DisableSimpleMasterStrictModeW<MiscCtrlDpRegSpec> {
        DisableSimpleMasterStrictModeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline(always)]
    #[must_use]
    pub fn disable_smart_master_strict_mode(
        &mut self,
    ) -> DisableSmartMasterStrictModeW<MiscCtrlDpRegSpec> {
        DisableSmartMasterStrictModeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline(always)]
    #[must_use]
    pub fn idau_all_ns(&mut self) -> IdauAllNsW<MiscCtrlDpRegSpec> {
        IdauAllNsW::new(self, 14)
    }
}
#[doc = "secure control duplicate register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_ctrl_dp_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_ctrl_dp_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscCtrlDpRegSpec;
impl crate::RegisterSpec for MiscCtrlDpRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_ctrl_dp_reg::R`](R) reader structure"]
impl crate::Readable for MiscCtrlDpRegSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_ctrl_dp_reg::W`](W) writer structure"]
impl crate::Writable for MiscCtrlDpRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_CTRL_DP_REG to value 0xaaaa"]
impl crate::Resettable for MiscCtrlDpRegSpec {
    const RESET_VALUE: u32 = 0xaaaa;
}
