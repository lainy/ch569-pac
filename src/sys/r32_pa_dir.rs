#[doc = "Register `R32_PA_DIR` reader"]
pub type R = crate::R<R32PaDirSpec>;
#[doc = "Register `R32_PA_DIR` writer"]
pub type W = crate::W<R32PaDirSpec>;
#[doc = "Field `R32_PA_DIR` reader - GPIO PA I/O direction"]
pub type R32PaDirR = crate::FieldReader<u32>;
#[doc = "Field `R32_PA_DIR` writer - GPIO PA I/O direction"]
pub type R32PaDirW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPIO PA I/O direction"]
    #[inline(always)]
    pub fn r32_pa_dir(&self) -> R32PaDirR {
        R32PaDirR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPIO PA I/O direction"]
    #[inline(always)]
    pub fn r32_pa_dir(&mut self) -> R32PaDirW<'_, R32PaDirSpec> {
        R32PaDirW::new(self, 0)
    }
}
#[doc = "GPIO PA I/O direction\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pa_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pa_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PaDirSpec;
impl crate::RegisterSpec for R32PaDirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pa_dir::R`](R) reader structure"]
impl crate::Readable for R32PaDirSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pa_dir::W`](W) writer structure"]
impl crate::Writable for R32PaDirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PA_DIR to value 0"]
impl crate::Resettable for R32PaDirSpec {}
