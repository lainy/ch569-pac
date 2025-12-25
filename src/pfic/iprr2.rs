#[doc = "Register `IPRR2` reader"]
pub type R = crate::R<Iprr2Spec>;
#[doc = "Register `IPRR2` writer"]
pub type W = crate::W<Iprr2Spec>;
#[doc = "Field `PENDRESET` reader - PENDRESET"]
pub type PendresetR = crate::FieldReader<u32>;
#[doc = "Field `PENDRESET` writer - PENDRESET"]
pub type PendresetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&self) -> PendresetR {
        PendresetR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&mut self) -> PendresetW<'_, Iprr2Spec> {
        PendresetW::new(self, 0)
    }
}
#[doc = "Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprr2Spec;
impl crate::RegisterSpec for Iprr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprr2::R`](R) reader structure"]
impl crate::Readable for Iprr2Spec {}
#[doc = "`write(|w| ..)` method takes [`iprr2::W`](W) writer structure"]
impl crate::Writable for Iprr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRR2 to value 0"]
impl crate::Resettable for Iprr2Spec {}
