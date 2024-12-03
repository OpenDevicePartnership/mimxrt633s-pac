#[doc = "Register `CMD_ARG` reader"]
pub type R = crate::R<CmdArgSpec>;
#[doc = "Register `CMD_ARG` writer"]
pub type W = crate::W<CmdArgSpec>;
#[doc = "Field `CMDARG` reader - Command Argument"]
pub type CmdargR = crate::FieldReader<u32>;
#[doc = "Field `CMDARG` writer - Command Argument"]
pub type CmdargW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn cmdarg(&self) -> CmdargR {
        CmdargR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_ARG")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CmdargW<CmdArgSpec> {
        CmdargW::new(self, 0)
    }
}
#[doc = "Command Argument\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_arg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_arg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdArgSpec;
impl crate::RegisterSpec for CmdArgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_arg::R`](R) reader structure"]
impl crate::Readable for CmdArgSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_arg::W`](W) writer structure"]
impl crate::Writable for CmdArgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_ARG to value 0"]
impl crate::Resettable for CmdArgSpec {
    const RESET_VALUE: u32 = 0;
}
