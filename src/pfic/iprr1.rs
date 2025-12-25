#[doc = "Register `IPRR1` reader"]
pub type R = crate::R<Iprr1Spec>;
#[doc = "Register `IPRR1` writer"]
pub type W = crate::W<Iprr1Spec>;
#[doc = "Field `PENDRESET` reader - PENDRESET"]
pub type PendresetR = crate::FieldReader<u32>;
#[doc = "Field `PENDRESET` writer - PENDRESET"]
pub type PendresetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&self) -> PendresetR {
        PendresetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&mut self) -> PendresetW<'_, Iprr1Spec> {
        PendresetW::new(self, 12)
    }
}
#[doc = "Interrupt Pending Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iprr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iprr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iprr1Spec;
impl crate::RegisterSpec for Iprr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iprr1::R`](R) reader structure"]
impl crate::Readable for Iprr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iprr1::W`](W) writer structure"]
impl crate::Writable for Iprr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPRR1 to value 0"]
impl crate::Resettable for Iprr1Spec {}
