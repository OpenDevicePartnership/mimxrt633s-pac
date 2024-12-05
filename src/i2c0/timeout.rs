#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TimeoutSpec>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TimeoutSpec>;
#[doc = "Field `TOMIN` reader - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
pub type TominR = crate::FieldReader;
#[doc = "Field `TOMIN` writer - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
pub type TominW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TO` reader - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
pub type ToR = crate::FieldReader<u16>;
#[doc = "Field `TO` writer - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub fn tomin(&self) -> TominR {
        TominR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("tomin", &self.tomin())
            .field("to", &self.to())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub fn tomin(&mut self) -> TominW<TimeoutSpec> {
        TominW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<TimeoutSpec> {
        ToW::new(self, 4)
    }
}
#[doc = "Time-out value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutSpec;
impl crate::RegisterSpec for TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0xffff"]
impl crate::Resettable for TimeoutSpec {
    const RESET_VALUE: u32 = 0xffff;
}
