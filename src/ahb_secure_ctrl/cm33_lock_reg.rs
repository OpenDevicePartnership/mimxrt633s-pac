#[doc = "Register `CM33_LOCK_REG` reader"]
pub type R = crate::R<Cm33LockRegSpec>;
#[doc = "Register `CM33_LOCK_REG` writer"]
pub type W = crate::W<Cm33LockRegSpec>;
#[doc = "m33 LOCKNSVTOR write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockNsVtor {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(variant: LockNsVtor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockNsVtor {
    type Ux = u8;
}
impl crate::IsEnum for LockNsVtor {}
#[doc = "Field `LOCK_NS_VTOR` reader - m33 LOCKNSVTOR write-lock."]
pub type LockNsVtorR = crate::FieldReader<LockNsVtor>;
impl LockNsVtorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockNsVtor> {
        match self.bits {
            1 => Some(LockNsVtor::Blocked),
            2 => Some(LockNsVtor::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockNsVtor::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockNsVtor::Writable
    }
}
#[doc = "Field `LOCK_NS_VTOR` writer - m33 LOCKNSVTOR write-lock."]
pub type LockNsVtorW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockNsVtor>;
impl<'a, REG> LockNsVtorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsVtor::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsVtor::Writable)
    }
}
#[doc = "m33 LOCKNSMPU write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockNsMpu {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(variant: LockNsMpu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockNsMpu {
    type Ux = u8;
}
impl crate::IsEnum for LockNsMpu {}
#[doc = "Field `LOCK_NS_MPU` reader - m33 LOCKNSMPU write-lock."]
pub type LockNsMpuR = crate::FieldReader<LockNsMpu>;
impl LockNsMpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockNsMpu> {
        match self.bits {
            1 => Some(LockNsMpu::Blocked),
            2 => Some(LockNsMpu::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockNsMpu::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockNsMpu::Writable
    }
}
#[doc = "Field `LOCK_NS_MPU` writer - m33 LOCKNSMPU write-lock."]
pub type LockNsMpuW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockNsMpu>;
impl<'a, REG> LockNsMpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsMpu::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsMpu::Writable)
    }
}
#[doc = "m33 LOCKSVTOR write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockSVtor {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockSVtor> for u8 {
    #[inline(always)]
    fn from(variant: LockSVtor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockSVtor {
    type Ux = u8;
}
impl crate::IsEnum for LockSVtor {}
#[doc = "Field `LOCK_S_VTOR` reader - m33 LOCKSVTOR write-lock."]
pub type LockSVtorR = crate::FieldReader<LockSVtor>;
impl LockSVtorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockSVtor> {
        match self.bits {
            1 => Some(LockSVtor::Blocked),
            2 => Some(LockSVtor::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockSVtor::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockSVtor::Writable
    }
}
#[doc = "Field `LOCK_S_VTOR` writer - m33 LOCKSVTOR write-lock."]
pub type LockSVtorW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockSVtor>;
impl<'a, REG> LockSVtorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockSVtor::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockSVtor::Writable)
    }
}
#[doc = "m33 LOCKSMPU write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockSMpu {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(variant: LockSMpu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockSMpu {
    type Ux = u8;
}
impl crate::IsEnum for LockSMpu {}
#[doc = "Field `LOCK_S_MPU` reader - m33 LOCKSMPU write-lock."]
pub type LockSMpuR = crate::FieldReader<LockSMpu>;
impl LockSMpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockSMpu> {
        match self.bits {
            1 => Some(LockSMpu::Blocked),
            2 => Some(LockSMpu::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockSMpu::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockSMpu::Writable
    }
}
#[doc = "Field `LOCK_S_MPU` writer - m33 LOCKSMPU write-lock."]
pub type LockSMpuW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockSMpu>;
impl<'a, REG> LockSMpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockSMpu::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockSMpu::Writable)
    }
}
#[doc = "m33 LOCKSAU write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockSau {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(variant: LockSau) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockSau {
    type Ux = u8;
}
impl crate::IsEnum for LockSau {}
#[doc = "Field `LOCK_SAU` reader - m33 LOCKSAU write-lock."]
pub type LockSauR = crate::FieldReader<LockSau>;
impl LockSauR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockSau> {
        match self.bits {
            1 => Some(LockSau::Blocked),
            2 => Some(LockSau::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockSau::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockSau::Writable
    }
}
#[doc = "Field `LOCK_SAU` writer - m33 LOCKSAU write-lock."]
pub type LockSauW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockSau>;
impl<'a, REG> LockSauW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockSau::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockSau::Writable)
    }
}
#[doc = "CM33_LOCK_REG_LOCK write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm33LockRegLock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<Cm33LockRegLock> for u8 {
    #[inline(always)]
    fn from(variant: Cm33LockRegLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm33LockRegLock {
    type Ux = u8;
}
impl crate::IsEnum for Cm33LockRegLock {}
#[doc = "Field `CM33_LOCK_REG_LOCK` reader - CM33_LOCK_REG_LOCK write-lock."]
pub type Cm33LockRegLockR = crate::FieldReader<Cm33LockRegLock>;
impl Cm33LockRegLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cm33LockRegLock> {
        match self.bits {
            1 => Some(Cm33LockRegLock::Blocked),
            2 => Some(Cm33LockRegLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Cm33LockRegLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == Cm33LockRegLock::Writable
    }
}
#[doc = "Field `CM33_LOCK_REG_LOCK` writer - CM33_LOCK_REG_LOCK write-lock."]
pub type Cm33LockRegLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm33LockRegLock>;
impl<'a, REG> Cm33LockRegLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Cm33LockRegLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(Cm33LockRegLock::Writable)
    }
}
impl R {
    #[doc = "Bits 0:1 - m33 LOCKNSVTOR write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LockNsVtorR {
        LockNsVtorR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - m33 LOCKNSMPU write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LockNsMpuR {
        LockNsMpuR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - m33 LOCKSVTOR write-lock."]
    #[inline(always)]
    pub fn lock_s_vtor(&self) -> LockSVtorR {
        LockSVtorR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - m33 LOCKSMPU write-lock."]
    #[inline(always)]
    pub fn lock_s_mpu(&self) -> LockSMpuR {
        LockSMpuR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - m33 LOCKSAU write-lock."]
    #[inline(always)]
    pub fn lock_sau(&self) -> LockSauR {
        LockSauR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31 - CM33_LOCK_REG_LOCK write-lock."]
    #[inline(always)]
    pub fn cm33_lock_reg_lock(&self) -> Cm33LockRegLockR {
        Cm33LockRegLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM33_LOCK_REG")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtor", &self.lock_s_vtor())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .field("cm33_lock_reg_lock", &self.cm33_lock_reg_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - m33 LOCKNSVTOR write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_ns_vtor(&mut self) -> LockNsVtorW<Cm33LockRegSpec> {
        LockNsVtorW::new(self, 0)
    }
    #[doc = "Bits 2:3 - m33 LOCKNSMPU write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_ns_mpu(&mut self) -> LockNsMpuW<Cm33LockRegSpec> {
        LockNsMpuW::new(self, 2)
    }
    #[doc = "Bits 4:5 - m33 LOCKSVTOR write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_s_vtor(&mut self) -> LockSVtorW<Cm33LockRegSpec> {
        LockSVtorW::new(self, 4)
    }
    #[doc = "Bits 6:7 - m33 LOCKSMPU write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_s_mpu(&mut self) -> LockSMpuW<Cm33LockRegSpec> {
        LockSMpuW::new(self, 6)
    }
    #[doc = "Bits 8:9 - m33 LOCKSAU write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_sau(&mut self) -> LockSauW<Cm33LockRegSpec> {
        LockSauW::new(self, 8)
    }
    #[doc = "Bits 30:31 - CM33_LOCK_REG_LOCK write-lock."]
    #[inline(always)]
    #[must_use]
    pub fn cm33_lock_reg_lock(&mut self) -> Cm33LockRegLockW<Cm33LockRegSpec> {
        Cm33LockRegLockW::new(self, 30)
    }
}
#[doc = "m33 lock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cm33_lock_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm33_lock_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm33LockRegSpec;
impl crate::RegisterSpec for Cm33LockRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm33_lock_reg::R`](R) reader structure"]
impl crate::Readable for Cm33LockRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cm33_lock_reg::W`](W) writer structure"]
impl crate::Writable for Cm33LockRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM33_LOCK_REG to value 0x8000_02aa"]
impl crate::Resettable for Cm33LockRegSpec {
    const RESET_VALUE: u32 = 0x8000_02aa;
}
