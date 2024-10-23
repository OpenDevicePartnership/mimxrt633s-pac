#[doc = "Register `CMD_RSP1` reader"]
pub type R = crate::R<CmdRsp1Spec>;
#[doc = "Field `CMDRSP1` reader - Command Response 1"]
pub type Cmdrsp1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 1"]
    #[inline(always)]
    pub fn cmdrsp1(&self) -> Cmdrsp1R {
        Cmdrsp1R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_RSP1")
            .field("cmdrsp1", &self.cmdrsp1())
            .finish()
    }
}
#[doc = "Command Response1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_rsp1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRsp1Spec;
impl crate::RegisterSpec for CmdRsp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_rsp1::R`](R) reader structure"]
impl crate::Readable for CmdRsp1Spec {}
#[doc = "`reset()` method sets CMD_RSP1 to value 0"]
impl crate::Resettable for CmdRsp1Spec {
    const RESET_VALUE: u32 = 0;
}
