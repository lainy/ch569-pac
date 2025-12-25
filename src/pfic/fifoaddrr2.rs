#[doc = "Register `FIFOADDRR2` reader"]
pub type R = crate::R<Fifoaddrr2Spec>;
#[doc = "Register `FIFOADDRR2` writer"]
pub type W = crate::W<Fifoaddrr2Spec>;
#[doc = "Field `OFFADDR2` reader - OFFADDR2"]
pub type Offaddr2R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR2` writer - OFFADDR2"]
pub type Offaddr2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID2` reader - IRQID2"]
pub type Irqid2R = crate::FieldReader;
#[doc = "Field `IRQID2` writer - IRQID2"]
pub type Irqid2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&self) -> Offaddr2R {
        Offaddr2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - IRQID2"]
    #[inline(always)]
    pub fn irqid2(&self) -> Irqid2R {
        Irqid2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR2"]
    #[inline(always)]
    pub fn offaddr2(&mut self) -> Offaddr2W<'_, Fifoaddrr2Spec> {
        Offaddr2W::new(self, 0)
    }
    #[doc = "Bits 24:31 - IRQID2"]
    #[inline(always)]
    pub fn irqid2(&mut self) -> Irqid2W<'_, Fifoaddrr2Spec> {
        Irqid2W::new(self, 24)
    }
}
#[doc = "Interrupt 2 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifoaddrr2Spec;
impl crate::RegisterSpec for Fifoaddrr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoaddrr2::R`](R) reader structure"]
impl crate::Readable for Fifoaddrr2Spec {}
#[doc = "`write(|w| ..)` method takes [`fifoaddrr2::W`](W) writer structure"]
impl crate::Writable for Fifoaddrr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOADDRR2 to value 0"]
impl crate::Resettable for Fifoaddrr2Spec {}
