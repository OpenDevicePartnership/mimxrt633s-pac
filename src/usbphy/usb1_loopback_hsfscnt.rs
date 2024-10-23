#[doc = "Register `USB1_LOOPBACK_HSFSCNT` reader"]
pub type R = crate::R<Usb1LoopbackHsfscntSpec>;
#[doc = "Register `USB1_LOOPBACK_HSFSCNT` writer"]
pub type W = crate::W<Usb1LoopbackHsfscntSpec>;
#[doc = "Field `TSTI_HS_NUMBER` reader - High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
pub type TstiHsNumberR = crate::FieldReader<u16>;
#[doc = "Field `TSTI_HS_NUMBER` writer - High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
pub type TstiHsNumberW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSTI_FS_NUMBER` reader - Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
pub type TstiFsNumberR = crate::FieldReader<u16>;
#[doc = "Field `TSTI_FS_NUMBER` writer - Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
pub type TstiFsNumberW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
    #[inline(always)]
    pub fn tsti_hs_number(&self) -> TstiHsNumberR {
        TstiHsNumberR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
    #[inline(always)]
    pub fn tsti_fs_number(&self) -> TstiFsNumberR {
        TstiFsNumberR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_LOOPBACK_HSFSCNT")
            .field("tsti_hs_number", &self.tsti_hs_number())
            .field("tsti_fs_number", &self.tsti_fs_number())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_hs_number(&mut self) -> TstiHsNumberW<Usb1LoopbackHsfscntSpec> {
        TstiHsNumberW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\]
is set to value 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn tsti_fs_number(&mut self) -> TstiFsNumberW<Usb1LoopbackHsfscntSpec> {
        TstiFsNumberW::new(self, 16)
    }
}
#[doc = "USB PHY Loopback Packet Number Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1_loopback_hsfscnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1_loopback_hsfscnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1LoopbackHsfscntSpec;
impl crate::RegisterSpec for Usb1LoopbackHsfscntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1_loopback_hsfscnt::R`](R) reader structure"]
impl crate::Readable for Usb1LoopbackHsfscntSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1_loopback_hsfscnt::W`](W) writer structure"]
impl crate::Writable for Usb1LoopbackHsfscntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB1_LOOPBACK_HSFSCNT to value 0x0004_0010"]
impl crate::Resettable for Usb1LoopbackHsfscntSpec {
    const RESET_VALUE: u32 = 0x0004_0010;
}
