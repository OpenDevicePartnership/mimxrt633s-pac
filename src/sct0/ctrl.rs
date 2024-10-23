#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `DOWN_L` reader - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DownLR = crate::BitReader;
#[doc = "Field `DOWN_L` writer - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DownLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_L` reader - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
pub type StopLR = crate::BitReader;
#[doc = "Field `STOP_L` writer - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
pub type StopLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT_L` reader - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
pub type HaltLR = crate::BitReader;
#[doc = "Field `HALT_L` writer - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
pub type HaltLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCTR_L` reader - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
pub type ClrctrLR = crate::BitReader;
#[doc = "Field `CLRCTR_L` writer - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
pub type ClrctrLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "L or unified counter direction select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidirL {
    #[doc = "0: Up. The counter counts up to a limit condition, then is cleared to zero."]
    Up = 0,
    #[doc = "1: Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    UpDown = 1,
}
impl From<BidirL> for bool {
    #[inline(always)]
    fn from(variant: BidirL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIDIR_L` reader - L or unified counter direction select"]
pub type BidirLR = crate::BitReader<BidirL>;
impl BidirLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BidirL {
        match self.bits {
            false => BidirL::Up,
            true => BidirL::UpDown,
        }
    }
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BidirL::Up
    }
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BidirL::UpDown
    }
}
#[doc = "Field `BIDIR_L` writer - L or unified counter direction select"]
pub type BidirLW<'a, REG> = crate::BitWriter<'a, REG, BidirL>;
impl<'a, REG> BidirLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(BidirL::Up)
    }
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut crate::W<REG> {
        self.variant(BidirL::UpDown)
    }
}
#[doc = "Field `PRE_L` reader - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PreLR = crate::FieldReader;
#[doc = "Field `PRE_L` writer - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PreLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DOWN_H` reader - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DownHR = crate::BitReader;
#[doc = "Field `DOWN_H` writer - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DownHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_H` reader - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
pub type StopHR = crate::BitReader;
#[doc = "Field `STOP_H` writer - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
pub type StopHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT_H` reader - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
pub type HaltHR = crate::BitReader;
#[doc = "Field `HALT_H` writer - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
pub type HaltHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCTR_H` reader - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
pub type ClrctrHR = crate::BitReader;
#[doc = "Field `CLRCTR_H` writer - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
pub type ClrctrHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Direction select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidirH {
    #[doc = "0: The H counter counts up to its limit condition, then is cleared to zero."]
    Up = 0,
    #[doc = "1: The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UpDown = 1,
}
impl From<BidirH> for bool {
    #[inline(always)]
    fn from(variant: BidirH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIDIR_H` reader - Direction select"]
pub type BidirHR = crate::BitReader<BidirH>;
impl BidirHR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BidirH {
        match self.bits {
            false => BidirH::Up,
            true => BidirH::UpDown,
        }
    }
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BidirH::Up
    }
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BidirH::UpDown
    }
}
#[doc = "Field `BIDIR_H` writer - Direction select"]
pub type BidirHW<'a, REG> = crate::BitWriter<'a, REG, BidirH>;
impl<'a, REG> BidirHW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(BidirH::Up)
    }
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut crate::W<REG> {
        self.variant(BidirH::UpDown)
    }
}
#[doc = "Field `PRE_H` reader - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PreHR = crate::FieldReader;
#[doc = "Field `PRE_H` writer - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PreHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_l(&self) -> DownLR {
        DownLR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_l(&self) -> StopLR {
        StopLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_l(&self) -> HaltLR {
        HaltLR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_l(&self) -> ClrctrLR {
        ClrctrLR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    pub fn bidir_l(&self) -> BidirLR {
        BidirLR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_l(&self) -> PreLR {
        PreLR::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_h(&self) -> DownHR {
        DownHR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_h(&self) -> StopHR {
        StopHR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_h(&self) -> HaltHR {
        HaltHR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_h(&self) -> ClrctrHR {
        ClrctrHR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    pub fn bidir_h(&self) -> BidirHR {
        BidirHR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_h(&self) -> PreHR {
        PreHR::new(((self.bits >> 21) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("down_l", &self.down_l())
            .field("stop_l", &self.stop_l())
            .field("halt_l", &self.halt_l())
            .field("clrctr_l", &self.clrctr_l())
            .field("bidir_l", &self.bidir_l())
            .field("pre_l", &self.pre_l())
            .field("down_h", &self.down_h())
            .field("stop_h", &self.stop_h())
            .field("halt_h", &self.halt_h())
            .field("clrctr_h", &self.clrctr_h())
            .field("bidir_h", &self.bidir_h())
            .field("pre_h", &self.pre_h())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn down_l(&mut self) -> DownLW<CtrlSpec> {
        DownLW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    #[must_use]
    pub fn stop_l(&mut self) -> StopLW<CtrlSpec> {
        StopLW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    #[must_use]
    pub fn halt_l(&mut self) -> HaltLW<CtrlSpec> {
        HaltLW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn clrctr_l(&mut self) -> ClrctrLW<CtrlSpec> {
        ClrctrLW::new(self, 3)
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    #[must_use]
    pub fn bidir_l(&mut self) -> BidirLW<CtrlSpec> {
        BidirLW::new(self, 4)
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pre_l(&mut self) -> PreLW<CtrlSpec> {
        PreLW::new(self, 5)
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn down_h(&mut self) -> DownHW<CtrlSpec> {
        DownHW::new(self, 16)
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    #[must_use]
    pub fn stop_h(&mut self) -> StopHW<CtrlSpec> {
        StopHW::new(self, 17)
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    #[must_use]
    pub fn halt_h(&mut self) -> HaltHW<CtrlSpec> {
        HaltHW::new(self, 18)
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn clrctr_h(&mut self) -> ClrctrHW<CtrlSpec> {
        ClrctrHW::new(self, 19)
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    #[must_use]
    pub fn bidir_h(&mut self) -> BidirHW<CtrlSpec> {
        BidirHW::new(self, 20)
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pre_h(&mut self) -> PreHW<CtrlSpec> {
        PreHW::new(self, 21)
    }
}
#[doc = "SCT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0004_0004"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0004_0004;
}
