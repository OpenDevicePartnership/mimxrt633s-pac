#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `DUAL` reader - 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
pub type DualR = crate::BitReader;
#[doc = "Field `DMA` reader - 1 if DMA is connected"]
pub type DmaR = crate::BitReader;
#[doc = "Field `AHB` reader - 1 if AHB Master is enabled"]
pub type AhbR = crate::BitReader;
#[doc = "Field `AES` reader - 1 if AES 128 included"]
pub type AesR = crate::BitReader;
#[doc = "Field `AESKEY` reader - 1 if AES 192 and 256 also included"]
pub type AeskeyR = crate::BitReader;
#[doc = "Field `SECRET` reader - 1 if AES Secret key available"]
pub type SecretR = crate::BitReader;
#[doc = "Field `ICB` reader - 1 if ICB over AES included"]
pub type IcbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1 if 2 x 512 bit buffers, 0 if only 1 x 512 bit"]
    #[inline(always)]
    pub fn dual(&self) -> DualR {
        DualR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 if DMA is connected"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 if AHB Master is enabled"]
    #[inline(always)]
    pub fn ahb(&self) -> AhbR {
        AhbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 if AES 128 included"]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 if AES 192 and 256 also included"]
    #[inline(always)]
    pub fn aeskey(&self) -> AeskeyR {
        AeskeyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 if AES Secret key available"]
    #[inline(always)]
    pub fn secret(&self) -> SecretR {
        SecretR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - 1 if ICB over AES included"]
    #[inline(always)]
    pub fn icb(&self) -> IcbR {
        IcbR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("dual", &self.dual())
            .field("dma", &self.dma())
            .field("ahb", &self.ahb())
            .field("aes", &self.aes())
            .field("aeskey", &self.aeskey())
            .field("secret", &self.secret())
            .field("icb", &self.icb())
            .finish()
    }
}
impl W {}
#[doc = "Returns the configuration of this block in this chip - indicates what services are available.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
