#[doc = "Register `SEC_MASK_LOCK` reader"]
pub type R = crate::R<SecMaskLockSpec>;
#[doc = "Register `SEC_MASK_LOCK` writer"]
pub type W = crate::W<SecMaskLockSpec>;
#[doc = "SEC_GPIO_MASK0 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask0Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask0Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask0Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask0Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask0Lock {}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` reader - SEC_GPIO_MASK0 register write-lock."]
pub type SecGpioMask0LockR = crate::FieldReader<SecGpioMask0Lock>;
impl SecGpioMask0LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask0Lock> {
        match self.bits {
            1 => Some(SecGpioMask0Lock::Blocked),
            2 => Some(SecGpioMask0Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask0Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask0Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` writer - SEC_GPIO_MASK0 register write-lock."]
pub type SecGpioMask0LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask0Lock>;
impl<'a, REG> SecGpioMask0LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask0Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask0Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK1 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask1Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask1Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask1Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask1Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask1Lock {}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` reader - SEC_GPIO_MASK1 register write-lock."]
pub type SecGpioMask1LockR = crate::FieldReader<SecGpioMask1Lock>;
impl SecGpioMask1LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask1Lock> {
        match self.bits {
            1 => Some(SecGpioMask1Lock::Blocked),
            2 => Some(SecGpioMask1Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask1Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask1Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` writer - SEC_GPIO_MASK1 register write-lock."]
pub type SecGpioMask1LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask1Lock>;
impl<'a, REG> SecGpioMask1LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask1Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask1Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK2 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask2Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask2Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask2Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask2Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask2Lock {}
#[doc = "Field `SEC_GPIO_MASK2_LOCK` reader - SEC_GPIO_MASK2 register write-lock."]
pub type SecGpioMask2LockR = crate::FieldReader<SecGpioMask2Lock>;
impl SecGpioMask2LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask2Lock> {
        match self.bits {
            1 => Some(SecGpioMask2Lock::Blocked),
            2 => Some(SecGpioMask2Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask2Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask2Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK2_LOCK` writer - SEC_GPIO_MASK2 register write-lock."]
pub type SecGpioMask2LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask2Lock>;
impl<'a, REG> SecGpioMask2LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask2Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask2Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK3 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask3Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask3Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask3Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask3Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask3Lock {}
#[doc = "Field `SEC_GPIO_MASK3_LOCK` reader - SEC_GPIO_MASK3 register write-lock."]
pub type SecGpioMask3LockR = crate::FieldReader<SecGpioMask3Lock>;
impl SecGpioMask3LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask3Lock> {
        match self.bits {
            1 => Some(SecGpioMask3Lock::Blocked),
            2 => Some(SecGpioMask3Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask3Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask3Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK3_LOCK` writer - SEC_GPIO_MASK3 register write-lock."]
pub type SecGpioMask3LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask3Lock>;
impl<'a, REG> SecGpioMask3LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask3Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask3Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK4 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask4Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask4Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask4Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask4Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask4Lock {}
#[doc = "Field `SEC_GPIO_MASK4_LOCK` reader - SEC_GPIO_MASK4 register write-lock."]
pub type SecGpioMask4LockR = crate::FieldReader<SecGpioMask4Lock>;
impl SecGpioMask4LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask4Lock> {
        match self.bits {
            1 => Some(SecGpioMask4Lock::Blocked),
            2 => Some(SecGpioMask4Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask4Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask4Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK4_LOCK` writer - SEC_GPIO_MASK4 register write-lock."]
pub type SecGpioMask4LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask4Lock>;
impl<'a, REG> SecGpioMask4LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask4Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask4Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK5 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask5Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask5Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask5Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask5Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask5Lock {}
#[doc = "Field `SEC_GPIO_MASK5_LOCK` reader - SEC_GPIO_MASK5 register write-lock."]
pub type SecGpioMask5LockR = crate::FieldReader<SecGpioMask5Lock>;
impl SecGpioMask5LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask5Lock> {
        match self.bits {
            1 => Some(SecGpioMask5Lock::Blocked),
            2 => Some(SecGpioMask5Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask5Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask5Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK5_LOCK` writer - SEC_GPIO_MASK5 register write-lock."]
pub type SecGpioMask5LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask5Lock>;
impl<'a, REG> SecGpioMask5LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask5Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask5Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK6 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask6Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask6Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask6Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask6Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask6Lock {}
#[doc = "Field `SEC_GPIO_MASK6_LOCK` reader - SEC_GPIO_MASK6 register write-lock."]
pub type SecGpioMask6LockR = crate::FieldReader<SecGpioMask6Lock>;
impl SecGpioMask6LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask6Lock> {
        match self.bits {
            1 => Some(SecGpioMask6Lock::Blocked),
            2 => Some(SecGpioMask6Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask6Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask6Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK6_LOCK` writer - SEC_GPIO_MASK6 register write-lock."]
pub type SecGpioMask6LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask6Lock>;
impl<'a, REG> SecGpioMask6LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask6Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask6Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK7 register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask7Lock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask7Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask7Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask7Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask7Lock {}
#[doc = "Field `SEC_GPIO_MASK7_LOCK` reader - SEC_GPIO_MASK7 register write-lock."]
pub type SecGpioMask7LockR = crate::FieldReader<SecGpioMask7Lock>;
impl SecGpioMask7LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask7Lock> {
        match self.bits {
            1 => Some(SecGpioMask7Lock::Blocked),
            2 => Some(SecGpioMask7Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask7Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask7Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK7_LOCK` writer - SEC_GPIO_MASK7 register write-lock."]
pub type SecGpioMask7LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask7Lock>;
impl<'a, REG> SecGpioMask7LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask7Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask7Lock::Writable)
    }
}
#[doc = "SEC_DSP_INT_MASK register write-lock.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecDspIntLock {
    #[doc = "1: Restrictive mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecDspIntLock> for u8 {
    #[inline(always)]
    fn from(variant: SecDspIntLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecDspIntLock {
    type Ux = u8;
}
impl crate::IsEnum for SecDspIntLock {}
#[doc = "Field `SEC_DSP_INT_LOCK` reader - SEC_DSP_INT_MASK register write-lock."]
pub type SecDspIntLockR = crate::FieldReader<SecDspIntLock>;
impl SecDspIntLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecDspIntLock> {
        match self.bits {
            1 => Some(SecDspIntLock::Blocked),
            2 => Some(SecDspIntLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecDspIntLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecDspIntLock::Writable
    }
}
#[doc = "Field `SEC_DSP_INT_LOCK` writer - SEC_DSP_INT_MASK register write-lock."]
pub type SecDspIntLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecDspIntLock>;
impl<'a, REG> SecDspIntLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restrictive mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecDspIntLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecDspIntLock::Writable)
    }
}
impl R {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&self) -> SecGpioMask0LockR {
        SecGpioMask0LockR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&self) -> SecGpioMask1LockR {
        SecGpioMask1LockR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SEC_GPIO_MASK2 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask2_lock(&self) -> SecGpioMask2LockR {
        SecGpioMask2LockR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SEC_GPIO_MASK3 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask3_lock(&self) -> SecGpioMask3LockR {
        SecGpioMask3LockR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SEC_GPIO_MASK4 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask4_lock(&self) -> SecGpioMask4LockR {
        SecGpioMask4LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SEC_GPIO_MASK5 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask5_lock(&self) -> SecGpioMask5LockR {
        SecGpioMask5LockR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SEC_GPIO_MASK6 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask6_lock(&self) -> SecGpioMask6LockR {
        SecGpioMask6LockR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SEC_GPIO_MASK7 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask7_lock(&self) -> SecGpioMask7LockR {
        SecGpioMask7LockR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SEC_DSP_INT_MASK register write-lock."]
    #[inline(always)]
    pub fn sec_dsp_int_lock(&self) -> SecDspIntLockR {
        SecDspIntLockR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_MASK_LOCK")
            .field("sec_gpio_mask0_lock", &self.sec_gpio_mask0_lock())
            .field("sec_gpio_mask1_lock", &self.sec_gpio_mask1_lock())
            .field("sec_gpio_mask2_lock", &self.sec_gpio_mask2_lock())
            .field("sec_gpio_mask3_lock", &self.sec_gpio_mask3_lock())
            .field("sec_gpio_mask4_lock", &self.sec_gpio_mask4_lock())
            .field("sec_gpio_mask5_lock", &self.sec_gpio_mask5_lock())
            .field("sec_gpio_mask6_lock", &self.sec_gpio_mask6_lock())
            .field("sec_gpio_mask7_lock", &self.sec_gpio_mask7_lock())
            .field("sec_dsp_int_lock", &self.sec_dsp_int_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&mut self) -> SecGpioMask0LockW<SecMaskLockSpec> {
        SecGpioMask0LockW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&mut self) -> SecGpioMask1LockW<SecMaskLockSpec> {
        SecGpioMask1LockW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SEC_GPIO_MASK2 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask2_lock(&mut self) -> SecGpioMask2LockW<SecMaskLockSpec> {
        SecGpioMask2LockW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SEC_GPIO_MASK3 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask3_lock(&mut self) -> SecGpioMask3LockW<SecMaskLockSpec> {
        SecGpioMask3LockW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SEC_GPIO_MASK4 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask4_lock(&mut self) -> SecGpioMask4LockW<SecMaskLockSpec> {
        SecGpioMask4LockW::new(self, 8)
    }
    #[doc = "Bits 10:11 - SEC_GPIO_MASK5 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask5_lock(&mut self) -> SecGpioMask5LockW<SecMaskLockSpec> {
        SecGpioMask5LockW::new(self, 10)
    }
    #[doc = "Bits 12:13 - SEC_GPIO_MASK6 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask6_lock(&mut self) -> SecGpioMask6LockW<SecMaskLockSpec> {
        SecGpioMask6LockW::new(self, 12)
    }
    #[doc = "Bits 14:15 - SEC_GPIO_MASK7 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask7_lock(&mut self) -> SecGpioMask7LockW<SecMaskLockSpec> {
        SecGpioMask7LockW::new(self, 14)
    }
    #[doc = "Bits 16:17 - SEC_DSP_INT_MASK register write-lock."]
    #[inline(always)]
    pub fn sec_dsp_int_lock(&mut self) -> SecDspIntLockW<SecMaskLockSpec> {
        SecDspIntLockW::new(self, 16)
    }
}
#[doc = "sec_gp_reg write-lock bits\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_mask_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_mask_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecMaskLockSpec;
impl crate::RegisterSpec for SecMaskLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_mask_lock::R`](R) reader structure"]
impl crate::Readable for SecMaskLockSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_mask_lock::W`](W) writer structure"]
impl crate::Writable for SecMaskLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_MASK_LOCK to value 0x0002_aaaa"]
impl crate::Resettable for SecMaskLockSpec {
    const RESET_VALUE: u32 = 0x0002_aaaa;
}
