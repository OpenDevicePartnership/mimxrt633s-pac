#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seclock {
    #[doc = "0: Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    Unlock = 0,
    #[doc = "1: Locks to the current security level. AHB Master will issue requests at this level."]
    Lock = 1,
}
impl From<Seclock> for u8 {
    #[inline(always)]
    fn from(variant: Seclock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seclock {
    type Ux = u8;
}
impl crate::IsEnum for Seclock {}
#[doc = "Field `SECLOCK` reader - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
pub type SeclockR = crate::FieldReader<Seclock>;
impl SeclockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seclock> {
        match self.bits {
            0 => Some(Seclock::Unlock),
            1 => Some(Seclock::Lock),
            _ => None,
        }
    }
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == Seclock::Unlock
    }
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == Seclock::Lock
    }
}
#[doc = "Field `SECLOCK` writer - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
pub type SeclockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Seclock>;
impl<'a, REG> SeclockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Seclock::Unlock)
    }
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(Seclock::Lock)
    }
}
#[doc = "Field `PATTERN` reader - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
pub type PatternR = crate::FieldReader<u16>;
#[doc = "Field `PATTERN` writer - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
pub type PatternW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub fn seclock(&self) -> SeclockR {
        SeclockR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    pub fn pattern(&self) -> PatternR {
        PatternR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("seclock", &self.seclock())
            .field("pattern", &self.pattern())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub fn seclock(&mut self) -> SeclockW<LockSpec> {
        SeclockW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    pub fn pattern(&mut self) -> PatternW<LockSpec> {
        PatternW::new(self, 4)
    }
}
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
