#[doc = "Register `FIFOADDRR3` reader"]
pub type R = crate::R<Fifoaddrr3Spec>;
#[doc = "Register `FIFOADDRR3` writer"]
pub type W = crate::W<Fifoaddrr3Spec>;
#[doc = "Field `OFFADDR3` reader - OFFADDR3"]
pub type Offaddr3R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR3` writer - OFFADDR3"]
pub type Offaddr3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID3` reader - IRQID3"]
pub type Irqid3R = crate::FieldReader;
#[doc = "Field `IRQID3` writer - IRQID3"]
pub type Irqid3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR3"]
    #[inline(always)]
    pub fn offaddr3(&self) -> Offaddr3R {
        Offaddr3R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - IRQID3"]
    #[inline(always)]
    pub fn irqid3(&self) -> Irqid3R {
        Irqid3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR3"]
    #[inline(always)]
    pub fn offaddr3(&mut self) -> Offaddr3W<'_, Fifoaddrr3Spec> {
        Offaddr3W::new(self, 0)
    }
    #[doc = "Bits 24:31 - IRQID3"]
    #[inline(always)]
    pub fn irqid3(&mut self) -> Irqid3W<'_, Fifoaddrr3Spec> {
        Irqid3W::new(self, 24)
    }
}
#[doc = "Interrupt 3 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifoaddrr3Spec;
impl crate::RegisterSpec for Fifoaddrr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoaddrr3::R`](R) reader structure"]
impl crate::Readable for Fifoaddrr3Spec {}
#[doc = "`write(|w| ..)` method takes [`fifoaddrr3::W`](W) writer structure"]
impl crate::Writable for Fifoaddrr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOADDRR3 to value 0"]
impl crate::Resettable for Fifoaddrr3Spec {}
