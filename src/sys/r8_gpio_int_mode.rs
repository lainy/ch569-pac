#[doc = "Register `R8_GPIO_INT_MODE` reader"]
pub type R = crate::R<R8GpioIntModeSpec>;
#[doc = "Register `R8_GPIO_INT_MODE` writer"]
pub type W = crate::W<R8GpioIntModeSpec>;
#[doc = "Field `RB_GPIO_PA2_IM` reader - PA2 pin interrupt mode"]
pub type RbGpioPa2ImR = crate::BitReader;
#[doc = "Field `RB_GPIO_PA2_IM` writer - PA2 pin interrupt mode"]
pub type RbGpioPa2ImW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_im(&self) -> RbGpioPa2ImR {
        RbGpioPa2ImR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_im(&mut self) -> RbGpioPa2ImW<'_, R8GpioIntModeSpec> {
        RbGpioPa2ImW::new(self, 0)
    }
}
#[doc = "GPIO interrupt mode\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GpioIntModeSpec;
impl crate::RegisterSpec for R8GpioIntModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_gpio_int_mode::R`](R) reader structure"]
impl crate::Readable for R8GpioIntModeSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_gpio_int_mode::W`](W) writer structure"]
impl crate::Writable for R8GpioIntModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_GPIO_INT_MODE to value 0"]
impl crate::Resettable for R8GpioIntModeSpec {}
