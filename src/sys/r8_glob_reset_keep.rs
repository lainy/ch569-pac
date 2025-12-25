#[doc = "Register `R8_GLOB_RESET_KEEP` reader"]
pub type R = crate::R<R8GlobResetKeepSpec>;
#[doc = "Register `R8_GLOB_RESET_KEEP` writer"]
pub type W = crate::W<R8GlobResetKeepSpec>;
#[doc = "Field `R8_GLOB_RESET_KEEP` reader - value keeper during global reset"]
pub type R8GlobResetKeepR = crate::FieldReader;
#[doc = "Field `R8_GLOB_RESET_KEEP` writer - value keeper during global reset"]
pub type R8GlobResetKeepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - value keeper during global reset"]
    #[inline(always)]
    pub fn r8_glob_reset_keep(&self) -> R8GlobResetKeepR {
        R8GlobResetKeepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - value keeper during global reset"]
    #[inline(always)]
    pub fn r8_glob_reset_keep(&mut self) -> R8GlobResetKeepW<'_, R8GlobResetKeepSpec> {
        R8GlobResetKeepW::new(self, 0)
    }
}
#[doc = "value keeper during global reset\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_glob_reset_keep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_glob_reset_keep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8GlobResetKeepSpec;
impl crate::RegisterSpec for R8GlobResetKeepSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_glob_reset_keep::R`](R) reader structure"]
impl crate::Readable for R8GlobResetKeepSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_glob_reset_keep::W`](W) writer structure"]
impl crate::Writable for R8GlobResetKeepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_GLOB_RESET_KEEP to value 0"]
impl crate::Resettable for R8GlobResetKeepSpec {}
