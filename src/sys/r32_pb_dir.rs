#[doc = "Register `R32_PB_DIR` reader"]
pub type R = crate::R<R32PbDirSpec>;
#[doc = "Register `R32_PB_DIR` writer"]
pub type W = crate::W<R32PbDirSpec>;
#[doc = "Field `R32_PB_DIR` reader - GPIO PB I/O direction"]
pub type R32PbDirR = crate::FieldReader<u32>;
#[doc = "Field `R32_PB_DIR` writer - GPIO PB I/O direction"]
pub type R32PbDirW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO PB I/O direction"]
    #[inline(always)]
    pub fn r32_pb_dir(&self) -> R32PbDirR {
        R32PbDirR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - GPIO PB I/O direction"]
    #[inline(always)]
    pub fn r32_pb_dir(&mut self) -> R32PbDirW<'_, R32PbDirSpec> {
        R32PbDirW::new(self, 0)
    }
}
#[doc = "GPIO PB I/O direction\n\nYou can [`read`](crate::Reg::read) this register and get [`r32_pb_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r32_pb_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R32PbDirSpec;
impl crate::RegisterSpec for R32PbDirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r32_pb_dir::R`](R) reader structure"]
impl crate::Readable for R32PbDirSpec {}
#[doc = "`write(|w| ..)` method takes [`r32_pb_dir::W`](W) writer structure"]
impl crate::Writable for R32PbDirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R32_PB_DIR to value 0"]
impl crate::Resettable for R32PbDirSpec {}
