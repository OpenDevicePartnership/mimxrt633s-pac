#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `decode_opcode` reader - opcode specific to decode_machine"]
pub type DecodeOpcodeR = crate::FieldReader;
#[doc = "Field `decode_opcode` writer - opcode specific to decode_machine"]
pub type DecodeOpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `decode_machine` reader - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
pub type DecodeMachineR = crate::FieldReader;
#[doc = "Field `decode_machine` writer - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
pub type DecodeMachineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `inst_busy` reader - Instruction busy signal when high indicates processing is on"]
pub type InstBusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - opcode specific to decode_machine"]
    #[inline(always)]
    pub fn decode_opcode(&self) -> DecodeOpcodeR {
        DecodeOpcodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub fn decode_machine(&self) -> DecodeMachineR {
        DecodeMachineR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Instruction busy signal when high indicates processing is on"]
    #[inline(always)]
    pub fn inst_busy(&self) -> InstBusyR {
        InstBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field("decode_opcode", &self.decode_opcode())
            .field("decode_machine", &self.decode_machine())
            .field("inst_busy", &self.inst_busy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - opcode specific to decode_machine"]
    #[inline(always)]
    pub fn decode_opcode(&mut self) -> DecodeOpcodeW<ControlSpec> {
        DecodeOpcodeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub fn decode_machine(&mut self) -> DecodeMachineW<ControlSpec> {
        DecodeMachineW::new(self, 4)
    }
}
#[doc = "PowerQuad Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
