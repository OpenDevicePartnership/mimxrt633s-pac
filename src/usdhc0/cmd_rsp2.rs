#[doc = "Register `CMD_RSP2` reader"]
pub type R = crate::R<CmdRsp2Spec>;
#[doc = "Field `CMDRSP2` reader - Command Response 2"]
pub type Cmdrsp2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdrsp2(&self) -> Cmdrsp2R {
        Cmdrsp2R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_RSP2")
            .field("cmdrsp2", &self.cmdrsp2())
            .finish()
    }
}
#[doc = "Command Response2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRsp2Spec;
impl crate::RegisterSpec for CmdRsp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_rsp2::R`](R) reader structure"]
impl crate::Readable for CmdRsp2Spec {}
#[doc = "`reset()` method sets CMD_RSP2 to value 0"]
impl crate::Resettable for CmdRsp2Spec {
    const RESET_VALUE: u32 = 0;
}
