#[doc = "Register `FFROCTL1` reader"]
pub type R = crate::R<Ffroctl1Spec>;
#[doc = "Register `FFROCTL1` writer"]
pub type W = crate::W<Ffroctl1Spec>;
#[doc = "Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Update {
    #[doc = "0: Normal Mode."]
    NormalMode = 0,
    #[doc = "1: Update Safe Mode."]
    UpdateSafeMode = 1,
}
impl From<Update> for bool {
    #[inline(always)]
    fn from(variant: Update) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATE` reader - Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
pub type UpdateR = crate::BitReader<Update>;
impl UpdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Update {
        match self.bits {
            false => Update::NormalMode,
            true => Update::UpdateSafeMode,
        }
    }
    #[doc = "Normal Mode."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Update::NormalMode
    }
    #[doc = "Update Safe Mode."]
    #[inline(always)]
    pub fn is_update_safe_mode(&self) -> bool {
        *self == Update::UpdateSafeMode
    }
}
#[doc = "Field `UPDATE` writer - Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
pub type UpdateW<'a, REG> = crate::BitWriter<'a, REG, Update>;
impl<'a, REG> UpdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Update::NormalMode)
    }
    #[doc = "Update Safe Mode."]
    #[inline(always)]
    pub fn update_safe_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Update::UpdateSafeMode)
    }
}
impl R {
    #[doc = "Bit 0 - Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FFROCTL1")
            .field("update", &self.update())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
    #[inline(always)]
    pub fn update(&mut self) -> UpdateW<Ffroctl1Spec> {
        UpdateW::new(self, 0)
    }
}
#[doc = "FFRO control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ffroctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffroctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ffroctl1Spec;
impl crate::RegisterSpec for Ffroctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffroctl1::R`](R) reader structure"]
impl crate::Readable for Ffroctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ffroctl1::W`](W) writer structure"]
impl crate::Writable for Ffroctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFROCTL1 to value 0"]
impl crate::Resettable for Ffroctl1Spec {
    const RESET_VALUE: u32 = 0;
}
