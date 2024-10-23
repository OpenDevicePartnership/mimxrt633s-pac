#[doc = "Register `LENGTH` reader"]
pub type R = crate::R<LengthSpec>;
#[doc = "Register `LENGTH` writer"]
pub type W = crate::W<LengthSpec>;
#[doc = "Field `inst_length` reader - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
pub type InstLengthR = crate::FieldReader<u32>;
#[doc = "Field `inst_length` writer - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
pub type InstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    pub fn inst_length(&self) -> InstLengthR {
        InstLengthR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LENGTH")
            .field("inst_length", &self.inst_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Length register. When FIR : fir_xlength = inst_length\\[15:0\\]
, fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\]
, cols_a = inst_length\\[12:8\\]
, cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn inst_length(&mut self) -> InstLengthW<LengthSpec> {
        InstLengthW::new(self, 0)
    }
}
#[doc = "Length register\n\nYou can [`read`](crate::Reg::read) this register and get [`length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LengthSpec;
impl crate::RegisterSpec for LengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`length::R`](R) reader structure"]
impl crate::Readable for LengthSpec {}
#[doc = "`write(|w| ..)` method takes [`length::W`](W) writer structure"]
impl crate::Writable for LengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LengthSpec {
    const RESET_VALUE: u32 = 0;
}
