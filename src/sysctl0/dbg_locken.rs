#[doc = "Register `DBG_LOCKEN` reader"]
pub type R = crate::R<DbgLockenSpec>;
#[doc = "Register `DBG_LOCKEN` writer"]
pub type W = crate::W<DbgLockenSpec>;
#[doc = "Field `DBG_LOCKEN` reader - Debug Write Lock the following registers: DBG_FEATURES DBG_FEATURES_DP CS_PROTTEST CS_PROTCPU0 CS_PROTCPU1 DBG_AUTH_SCRATCH 1010: Write Enabled (Unlocked) Any other value other than 1010, Write Disabled (Locked)"]
pub type DbgLockenR = crate::FieldReader;
#[doc = "Field `DBG_LOCKEN` writer - Debug Write Lock the following registers: DBG_FEATURES DBG_FEATURES_DP CS_PROTTEST CS_PROTCPU0 CS_PROTCPU1 DBG_AUTH_SCRATCH 1010: Write Enabled (Unlocked) Any other value other than 1010, Write Disabled (Locked)"]
pub type DbgLockenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Debug Write Lock the following registers: DBG_FEATURES DBG_FEATURES_DP CS_PROTTEST CS_PROTCPU0 CS_PROTCPU1 DBG_AUTH_SCRATCH 1010: Write Enabled (Unlocked) Any other value other than 1010, Write Disabled (Locked)"]
    #[inline(always)]
    pub fn dbg_locken(&self) -> DbgLockenR {
        DbgLockenR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_LOCKEN")
            .field("dbg_locken", &self.dbg_locken())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Debug Write Lock the following registers: DBG_FEATURES DBG_FEATURES_DP CS_PROTTEST CS_PROTCPU0 CS_PROTCPU1 DBG_AUTH_SCRATCH 1010: Write Enabled (Unlocked) Any other value other than 1010, Write Disabled (Locked)"]
    #[inline(always)]
    pub fn dbg_locken(&mut self) -> DbgLockenW<DbgLockenSpec> {
        DbgLockenW::new(self, 0)
    }
}
#[doc = "Debug Write Lock registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_locken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_locken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgLockenSpec;
impl crate::RegisterSpec for DbgLockenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_locken::R`](R) reader structure"]
impl crate::Readable for DbgLockenSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_locken::W`](W) writer structure"]
impl crate::Writable for DbgLockenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_LOCKEN to value 0x0a"]
impl crate::Resettable for DbgLockenSpec {
    const RESET_VALUE: u32 = 0x0a;
}
