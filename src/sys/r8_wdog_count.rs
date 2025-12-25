#[doc = "Register `R8_WDOG_COUNT` reader"]
pub type R = crate::R<R8WdogCountSpec>;
#[doc = "Register `R8_WDOG_COUNT` writer"]
pub type W = crate::W<R8WdogCountSpec>;
#[doc = "Field `R8_WDOG_COUNT` reader - watch-dog count"]
pub type R8WdogCountR = crate::FieldReader;
#[doc = "Field `R8_WDOG_COUNT` writer - watch-dog count"]
pub type R8WdogCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - watch-dog count"]
    #[inline(always)]
    pub fn r8_wdog_count(&self) -> R8WdogCountR {
        R8WdogCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - watch-dog count"]
    #[inline(always)]
    pub fn r8_wdog_count(&mut self) -> R8WdogCountW<'_, R8WdogCountSpec> {
        R8WdogCountW::new(self, 0)
    }
}
#[doc = "watch-dog count register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_wdog_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_wdog_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8WdogCountSpec;
impl crate::RegisterSpec for R8WdogCountSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_wdog_count::R`](R) reader structure"]
impl crate::Readable for R8WdogCountSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_wdog_count::W`](W) writer structure"]
impl crate::Writable for R8WdogCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_WDOG_COUNT to value 0"]
impl crate::Resettable for R8WdogCountSpec {}
