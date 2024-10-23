#[doc = "Register `PIF_HIFI4_X_MEM_RULE0` reader"]
pub type R = crate::R<PifHifi4XMemRule0Spec>;
#[doc = "Register `PIF_HIFI4_X_MEM_RULE0` writer"]
pub type W = crate::W<PifHifi4XMemRule0Spec>;
#[doc = "Field `RULE0` reader - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule0R = crate::FieldReader;
#[doc = "Field `RULE0` writer - secure control rule0. it can be set when check_reg's write_lock is '0'"]
pub type Rule0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RULE1` reader - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub type Rule1R = crate::FieldReader;
#[doc = "Field `RULE1` writer - secure control rule1. it can be set when check_reg's write_lock is '0'"]
pub type Rule1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RULE4` reader - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub type Rule4R = crate::FieldReader;
#[doc = "Field `RULE4` writer - secure control rule4. it can be set when check_reg's write_lock is '0'"]
pub type Rule4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RULE5` reader - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub type Rule5R = crate::FieldReader;
#[doc = "Field `RULE5` writer - secure control rule5. it can be set when check_reg's write_lock is '0'"]
pub type Rule5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule0(&self) -> Rule0R {
        Rule0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule1(&self) -> Rule1R {
        Rule1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule4(&self) -> Rule4R {
        Rule4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    pub fn rule5(&self) -> Rule5R {
        Rule5R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIF_HIFI4_X_MEM_RULE0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - secure control rule0. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule0(&mut self) -> Rule0W<PifHifi4XMemRule0Spec> {
        Rule0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - secure control rule1. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule1(&mut self) -> Rule1W<PifHifi4XMemRule0Spec> {
        Rule1W::new(self, 4)
    }
    #[doc = "Bits 16:17 - secure control rule4. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule4(&mut self) -> Rule4W<PifHifi4XMemRule0Spec> {
        Rule4W::new(self, 16)
    }
    #[doc = "Bits 20:21 - secure control rule5. it can be set when check_reg's write_lock is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn rule5(&mut self) -> Rule5W<PifHifi4XMemRule0Spec> {
        Rule5W::new(self, 20)
    }
}
#[doc = "Security access rules for HiFi 4 memory sectors (0x24000000--0x240FFFFF). Each sector is 32 Kbytes, there're 4 sectors in total.\n\nYou can [`read`](crate::Reg::read) this register and get [`pif_hifi4_x_mem_rule0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pif_hifi4_x_mem_rule0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PifHifi4XMemRule0Spec;
impl crate::RegisterSpec for PifHifi4XMemRule0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pif_hifi4_x_mem_rule0::R`](R) reader structure"]
impl crate::Readable for PifHifi4XMemRule0Spec {}
#[doc = "`write(|w| ..)` method takes [`pif_hifi4_x_mem_rule0::W`](W) writer structure"]
impl crate::Writable for PifHifi4XMemRule0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIF_HIFI4_X_MEM_RULE0 to value 0"]
impl crate::Resettable for PifHifi4XMemRule0Spec {
    const RESET_VALUE: u32 = 0;
}
