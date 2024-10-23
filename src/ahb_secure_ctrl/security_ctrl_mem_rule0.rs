#[doc = "Register `SECURITY_CTRL_MEM_RULE0` reader"]
pub type R = crate::R<SecurityCtrlMemRule0Spec>;
#[doc = "Register `SECURITY_CTRL_MEM_RULE0` writer"]
pub type W = crate::W<SecurityCtrlMemRule0Spec>;
#[doc = "Field `RULE0` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule0R = crate::FieldReader;
#[doc = "Field `RULE0` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RULE1` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule1R = crate::FieldReader;
#[doc = "Field `RULE1` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RULE2` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule2R = crate::FieldReader;
#[doc = "Field `RULE2` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RULE3` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule3R = crate::FieldReader;
#[doc = "Field `RULE3` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&self) -> Rule0R {
        Rule0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&self) -> Rule1R {
        Rule1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule2(&self) -> Rule2R {
        Rule2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule3(&self) -> Rule3R {
        Rule3R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECURITY_CTRL_MEM_RULE0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule0(&mut self) -> Rule0W<SecurityCtrlMemRule0Spec> {
        Rule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule1(&mut self) -> Rule1W<SecurityCtrlMemRule0Spec> {
        Rule1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule2(&mut self) -> Rule2W<SecurityCtrlMemRule0Spec> {
        Rule2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule3(&mut self) -> Rule3W<SecurityCtrlMemRule0Spec> {
        Rule3W::new(self, 12)
    }
}
#[doc = "0x40148000--0x4014BFFF\n\nYou can [`read`](crate::Reg::read) this register and get [`security_ctrl_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`security_ctrl_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecurityCtrlMemRule0Spec;
impl crate::RegisterSpec for SecurityCtrlMemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`security_ctrl_mem_rule0::R`](R) reader structure"]
impl crate::Readable for SecurityCtrlMemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`security_ctrl_mem_rule0::W`](W) writer structure"]
impl crate::Writable for SecurityCtrlMemRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECURITY_CTRL_MEM_RULE0 to value 0"]
impl crate::Resettable for SecurityCtrlMemRule0Spec {
    const RESET_VALUE: u32 = 0;
}
