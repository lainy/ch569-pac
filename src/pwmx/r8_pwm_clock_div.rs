#[doc = "Register `R8_PWM_CLOCK_DIV` reader"]
pub type R = crate::R<R8PwmClockDivSpec>;
#[doc = "Register `R8_PWM_CLOCK_DIV` writer"]
pub type W = crate::W<R8PwmClockDivSpec>;
#[doc = "Field `R8_PWM_CLOCK_DIV` reader - PWM clock divisor"]
pub type R8PwmClockDivR = crate::FieldReader;
#[doc = "Field `R8_PWM_CLOCK_DIV` writer - PWM clock divisor"]
pub type R8PwmClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PWM clock divisor"]
    #[inline(always)]
    pub fn r8_pwm_clock_div(&self) -> R8PwmClockDivR {
        R8PwmClockDivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM clock divisor"]
    #[inline(always)]
    pub fn r8_pwm_clock_div(&mut self) -> R8PwmClockDivW<'_, R8PwmClockDivSpec> {
        R8PwmClockDivW::new(self, 0)
    }
}
#[doc = "PWM clock divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_pwm_clock_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_pwm_clock_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8PwmClockDivSpec;
impl crate::RegisterSpec for R8PwmClockDivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_pwm_clock_div::R`](R) reader structure"]
impl crate::Readable for R8PwmClockDivSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_pwm_clock_div::W`](W) writer structure"]
impl crate::Writable for R8PwmClockDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_PWM_CLOCK_DIV to value 0"]
impl crate::Resettable for R8PwmClockDivSpec {}
