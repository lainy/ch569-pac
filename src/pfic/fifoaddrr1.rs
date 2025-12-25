#[doc = "Register `FIFOADDRR1` reader"]
pub type R = crate::R<Fifoaddrr1Spec>;
#[doc = "Register `FIFOADDRR1` writer"]
pub type W = crate::W<Fifoaddrr1Spec>;
#[doc = "Field `OFFADDR1` reader - OFFADDR1"]
pub type Offaddr1R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR1` writer - OFFADDR1"]
pub type Offaddr1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID1` reader - IRQID1"]
pub type Irqid1R = crate::FieldReader;
#[doc = "Field `IRQID1` writer - IRQID1"]
pub type Irqid1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&self) -> Offaddr1R {
        Offaddr1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - IRQID1"]
    #[inline(always)]
    pub fn irqid1(&self) -> Irqid1R {
        Irqid1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR1"]
    #[inline(always)]
    pub fn offaddr1(&mut self) -> Offaddr1W<'_, Fifoaddrr1Spec> {
        Offaddr1W::new(self, 0)
    }
    #[doc = "Bits 24:31 - IRQID1"]
    #[inline(always)]
    pub fn irqid1(&mut self) -> Irqid1W<'_, Fifoaddrr1Spec> {
        Irqid1W::new(self, 24)
    }
}
#[doc = "Interrupt 1 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifoaddrr1Spec;
impl crate::RegisterSpec for Fifoaddrr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoaddrr1::R`](R) reader structure"]
impl crate::Readable for Fifoaddrr1Spec {}
#[doc = "`write(|w| ..)` method takes [`fifoaddrr1::W`](W) writer structure"]
impl crate::Writable for Fifoaddrr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOADDRR1 to value 0"]
impl crate::Resettable for Fifoaddrr1Spec {}
