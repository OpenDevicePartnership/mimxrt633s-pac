#[doc = "Register `CMD_RSP3` reader"]
pub type R = crate::R<CmdRsp3Spec>;
#[doc = "Field `CMDRSP3` reader - Command Response 3"]
pub type Cmdrsp3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdrsp3(&self) -> Cmdrsp3R {
        Cmdrsp3R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_RSP3")
            .field("cmdrsp3", &self.cmdrsp3())
            .finish()
    }
}
#[doc = "Command Response3\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRsp3Spec;
impl crate::RegisterSpec for CmdRsp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_rsp3::R`](R) reader structure"]
impl crate::Readable for CmdRsp3Spec {}
#[doc = "`reset()` method sets CMD_RSP3 to value 0"]
impl crate::Resettable for CmdRsp3Spec {
    const RESET_VALUE: u32 = 0;
}
