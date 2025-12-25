#[doc = "Register `R8_GPIO_INT_POLAR` reader"]
pub type R = crate::R<R8GpioIntPolarSpec>;
#[doc = "Register `R8_GPIO_INT_POLAR` writer"]
pub type W = crate::W<R8GpioIntPolarSpec>;
#[doc = "Field `RB_GPIO_PA2_IP` reader - PA2 pin interrupt mode"]
pub type RbGpioPa2IpR = crate::BitReader;
#[doc = "Field `RB_GPIO_PA2_IP` writer - PA2 pin interrupt mode"]
pub type RbGpioPa2IpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_ip(&self) -> RbGpioPa2IpR {
        RbGpioPa2IpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PA2 pin interrupt mode"]
    #[inline(always)]
    pub fn rb_gpio_pa2_ip(&mut self) -> RbGpioPa2IpW<'_, R8GpioIntPolarSpec> {
        RbGpioPa2IpW::new(self, 0)
    }
}
#[doc = "GPIO interrupt polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_gpio_int_polar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_gpio_int_polar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GpioIntPolarSpec;
impl crate::RegisterSpec for R8GpioIntPolarSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_gpio_int_polar::R`](R) reader structure"]
impl crate::Readable for R8GpioIntPolarSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_gpio_int_polar::W`](W) writer structure"]
impl crate::Writable for R8GpioIntPolarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_GPIO_INT_POLAR to value 0"]
impl crate::Resettable for R8GpioIntPolarSpec {}
