#[doc = "Register `R32_PA_DRV` reader"]
pub type R = crate::R<R32PaDrvSpec>;
#[doc = "Register `R32_PA_DRV` writer"]
pub type W = crate::W<R32PaDrvSpec>;
#[doc = "Field `R32_PA_DRV` reader - GPIO PA driving capability"]
pub type R32PaDrvR = crate::FieldReader<u32>;
#[doc = "Field `R32_PA_DRV` writer - GPIO PA driving capability"]
pub type R32PaDrvW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn r32_pa_drv(&self) -> R32PaDrvR {
        R32PaDrvR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPIO PA driving capability"]
    #[inline(always)]
    pub fn r32_pa_drv(&mut self) -> R32PaDrvW<'_, R32PaDrvSpec> {
        R32PaDrvW::new(self, 0)
    }
}
#[doc = "GPIO PA driving capability\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_drv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_drv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaDrvSpec;
impl crate::RegisterSpec for R32PaDrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_drv::R`](R) reader structure"]
impl crate::Readable for R32PaDrvSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_drv::W`](W) writer structure"]
impl crate::Writable for R32PaDrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_DRV to value 0"]
impl crate::Resettable for R32PaDrvSpec {}
