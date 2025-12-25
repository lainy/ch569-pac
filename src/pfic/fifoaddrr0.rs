#[doc = "Register `FIFOADDRR0` reader"]
pub type R = crate::R<Fifoaddrr0Spec>;
#[doc = "Register `FIFOADDRR0` writer"]
pub type W = crate::W<Fifoaddrr0Spec>;
#[doc = "Field `OFFADDR0` reader - OFFADDR0"]
pub type Offaddr0R = crate::FieldReader<u32>;
#[doc = "Field `OFFADDR0` writer - OFFADDR0"]
pub type Offaddr0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `IRQID0` reader - IRQID0"]
pub type Irqid0R = crate::FieldReader;
#[doc = "Field `IRQID0` writer - IRQID0"]
pub type Irqid0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&self) -> Offaddr0R {
        Offaddr0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - IRQID0"]
    #[inline(always)]
    pub fn irqid0(&self) -> Irqid0R {
        Irqid0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - OFFADDR0"]
    #[inline(always)]
    pub fn offaddr0(&mut self) -> Offaddr0W<'_, Fifoaddrr0Spec> {
        Offaddr0W::new(self, 0)
    }
    #[doc = "Bits 24:31 - IRQID0"]
    #[inline(always)]
    pub fn irqid0(&mut self) -> Irqid0W<'_, Fifoaddrr0Spec> {
        Irqid0W::new(self, 24)
    }
}
#[doc = "Interrupt 0 address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoaddrr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoaddrr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifoaddrr0Spec;
impl crate::RegisterSpec for Fifoaddrr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoaddrr0::R`](R) reader structure"]
impl crate::Readable for Fifoaddrr0Spec {}
#[doc = "`write(|w| ..)` method takes [`fifoaddrr0::W`](W) writer structure"]
impl crate::Writable for Fifoaddrr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFOADDRR0 to value 0"]
impl crate::Resettable for Fifoaddrr0Spec {}
