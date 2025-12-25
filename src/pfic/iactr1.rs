#[doc = "Register `IACTR1` reader"]
pub type R = crate::R<Iactr1Spec>;
#[doc = "Register `IACTR1` writer"]
pub type W = crate::W<Iactr1Spec>;
#[doc = "Field `IACTS` reader - IACTS"]
pub type IactsR = crate::FieldReader<u32>;
#[doc = "Field `IACTS` writer - IACTS"]
pub type IactsW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - IACTS"]
    #[inline(always)]
    pub fn iacts(&self) -> IactsR {
        IactsR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - IACTS"]
    #[inline(always)]
    pub fn iacts(&mut self) -> IactsW<'_, Iactr1Spec> {
        IactsW::new(self, 12)
    }
}
#[doc = "Interrupt ACTIVE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iactr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iactr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iactr1Spec;
impl crate::RegisterSpec for Iactr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iactr1::R`](R) reader structure"]
impl crate::Readable for Iactr1Spec {}
#[doc = "`write(|w| ..)` method takes [`iactr1::W`](W) writer structure"]
impl crate::Writable for Iactr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IACTR1 to value 0"]
impl crate::Resettable for Iactr1Spec {}
