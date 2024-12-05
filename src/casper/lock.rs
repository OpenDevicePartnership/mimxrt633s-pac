#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: unlock"]
    Unlock = 0,
    #[doc = "1: Lock to current security level"]
    Lock = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlock,
            true => Lock::Lock,
        }
    }
    #[doc = "unlock"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == Lock::Unlock
    }
    #[doc = "Lock to current security level"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Lock::Lock
    }
}
#[doc = "Field `LOCK` writer - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "unlock"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlock)
    }
    #[doc = "Lock to current security level"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Lock)
    }
}
#[doc = "Must be written as 0x73D to change the register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    #[doc = "1853: If set during write, will allow lock or unlock"]
    KwyValue = 1853,
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u16;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Must be written as 0x73D to change the register."]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            1853 => Some(Key::KwyValue),
            _ => None,
        }
    }
    #[doc = "If set during write, will allow lock or unlock"]
    #[inline(always)]
    pub fn is_kwy_value(&self) -> bool {
        *self == Key::KwyValue
    }
}
#[doc = "Field `KEY` writer - Must be written as 0x73D to change the register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 13, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "If set during write, will allow lock or unlock"]
    #[inline(always)]
    pub fn kwy_value(self) -> &'a mut crate::W<REG> {
        self.variant(Key::KwyValue)
    }
}
impl R {
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 4) & 0x1fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("lock", &self.lock())
            .field("key", &self.key())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reads back with security level locked to, or 0. Writes as 0 to unlock, 1 to lock."]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<LockSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 4:16 - Must be written as 0x73D to change the register."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<LockSpec> {
        KeyW::new(self, 4)
    }
}
#[doc = "Security lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {
    const RESET_VALUE: u32 = 0;
}
