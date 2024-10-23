#[doc = "Register `HWVADHPFS` reader"]
pub type R = crate::R<HwvadhpfsSpec>;
#[doc = "Register `HWVADHPFS` writer"]
pub type W = crate::W<HwvadhpfsSpec>;
#[doc = "This field chooses the High Pass filter in first part of HWVAD\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hpfs {
    #[doc = "0: First filter by-pass."]
    Bypass = 0,
    #[doc = "1: High pass filter with -3dB cut-off at 1750Hz."]
    HighPass1750hz = 1,
    #[doc = "2: High pass filter with -3dB cut-off at 215Hz."]
    HighPass215hz = 2,
}
impl From<Hpfs> for u8 {
    #[inline(always)]
    fn from(variant: Hpfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hpfs {
    type Ux = u8;
}
impl crate::IsEnum for Hpfs {}
#[doc = "Field `HPFS` reader - This field chooses the High Pass filter in first part of HWVAD"]
pub type HpfsR = crate::FieldReader<Hpfs>;
impl HpfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hpfs> {
        match self.bits {
            0 => Some(Hpfs::Bypass),
            1 => Some(Hpfs::HighPass1750hz),
            2 => Some(Hpfs::HighPass215hz),
            _ => None,
        }
    }
    #[doc = "First filter by-pass."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Hpfs::Bypass
    }
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    #[inline(always)]
    pub fn is_high_pass_1750hz(&self) -> bool {
        *self == Hpfs::HighPass1750hz
    }
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    #[inline(always)]
    pub fn is_high_pass_215hz(&self) -> bool {
        *self == Hpfs::HighPass215hz
    }
}
#[doc = "Field `HPFS` writer - This field chooses the High Pass filter in first part of HWVAD"]
pub type HpfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hpfs>;
impl<'a, REG> HpfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "First filter by-pass."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Hpfs::Bypass)
    }
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    #[inline(always)]
    pub fn high_pass_1750hz(self) -> &'a mut crate::W<REG> {
        self.variant(Hpfs::HighPass1750hz)
    }
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    #[inline(always)]
    pub fn high_pass_215hz(self) -> &'a mut crate::W<REG> {
        self.variant(Hpfs::HighPass215hz)
    }
}
impl R {
    #[doc = "Bits 0:1 - This field chooses the High Pass filter in first part of HWVAD"]
    #[inline(always)]
    pub fn hpfs(&self) -> HpfsR {
        HpfsR::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVADHPFS")
            .field("hpfs", &self.hpfs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field chooses the High Pass filter in first part of HWVAD"]
    #[inline(always)]
    #[must_use]
    pub fn hpfs(&mut self) -> HpfsW<HwvadhpfsSpec> {
        HpfsW::new(self, 0)
    }
}
#[doc = "HWVAD filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvadhpfs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvadhpfs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwvadhpfsSpec;
impl crate::RegisterSpec for HwvadhpfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwvadhpfs::R`](R) reader structure"]
impl crate::Readable for HwvadhpfsSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvadhpfs::W`](W) writer structure"]
impl crate::Writable for HwvadhpfsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWVADHPFS to value 0x01"]
impl crate::Resettable for HwvadhpfsSpec {
    const RESET_VALUE: u32 = 0x01;
}
