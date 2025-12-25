#[doc = "Register `R32_PWM_DATA` reader"]
pub type R = crate::R<R32PwmDataSpec>;
#[doc = "Register `R32_PWM_DATA` writer"]
pub type W = crate::W<R32PwmDataSpec>;
#[doc = "Field `R8_PWM0_DATA` reader - PWM0 data holding"]
pub type R8Pwm0DataR = crate::FieldReader;
#[doc = "Field `R8_PWM0_DATA` writer - PWM0 data holding"]
pub type R8Pwm0DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PWM1_DATA` reader - PWM1 data holding"]
pub type R8Pwm1DataR = crate::FieldReader;
#[doc = "Field `R8_PWM1_DATA` writer - PWM1 data holding"]
pub type R8Pwm1DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PWM2_DATA` reader - PWM2 data holding"]
pub type R8Pwm2DataR = crate::FieldReader;
#[doc = "Field `R8_PWM2_DATA` writer - PWM2 data holding"]
pub type R8Pwm2DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R8_PWM3_DATA` reader - PWM3 data holding"]
pub type R8Pwm3DataR = crate::FieldReader;
#[doc = "Field `R8_PWM3_DATA` writer - PWM3 data holding"]
pub type R8Pwm3DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PWM0 data holding"]
    #[inline(always)]
    pub fn r8_pwm0_data(&self) -> R8Pwm0DataR {
        R8Pwm0DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PWM1 data holding"]
    #[inline(always)]
    pub fn r8_pwm1_data(&self) -> R8Pwm1DataR {
        R8Pwm1DataR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PWM2 data holding"]
    #[inline(always)]
    pub fn r8_pwm2_data(&self) -> R8Pwm2DataR {
        R8Pwm2DataR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PWM3 data holding"]
    #[inline(always)]
    pub fn r8_pwm3_data(&self) -> R8Pwm3DataR {
        R8Pwm3DataR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PWM0 data holding"]
    #[inline(always)]
    pub fn r8_pwm0_data(&mut self) -> R8Pwm0DataW<'_, R32PwmDataSpec> {
        R8Pwm0DataW::new(self, 0)
    }
    #[doc = "Bits 8:15 - PWM1 data holding"]
    #[inline(always)]
    pub fn r8_pwm1_data(&mut self) -> R8Pwm1DataW<'_, R32PwmDataSpec> {
        R8Pwm1DataW::new(self, 8)
    }
    #[doc = "Bits 16:23 - PWM2 data holding"]
    #[inline(always)]
    pub fn r8_pwm2_data(&mut self) -> R8Pwm2DataW<'_, R32PwmDataSpec> {
        R8Pwm2DataW::new(self, 16)
    }
    #[doc = "Bits 24:31 - PWM3 data holding"]
    #[inline(always)]
    pub fn r8_pwm3_data(&mut self) -> R8Pwm3DataW<'_, R32PwmDataSpec> {
        R8Pwm3DataW::new(self, 24)
    }
}
#[doc = "PWM data holding\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pwm_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pwm_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PwmDataSpec;
impl crate::RegisterSpec for R32PwmDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pwm_data::R`](R) reader structure"]
impl crate::Readable for R32PwmDataSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pwm_data::W`](W) writer structure"]
impl crate::Writable for R32PwmDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PWM_DATA to value 0"]
impl crate::Resettable for R32PwmDataSpec {}
