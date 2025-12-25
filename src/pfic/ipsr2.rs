#[doc = "Register `IPSR2` reader"]
pub type R = crate::R<Ipsr2Spec>;
#[doc = "Register `IPSR2` writer"]
pub type W = crate::W<Ipsr2Spec>;
#[doc = "Field `PENDSET` reader - PENDSET"]
pub type PendsetR = crate::FieldReader<u32>;
#[doc = "Field `PENDSET` writer - PENDSET"]
pub type PendsetW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&self) -> PendsetR {
        PendsetR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&mut self) -> PendsetW<'_, Ipsr2Spec> {
        PendsetW::new(self, 0)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipsr2Spec;
impl crate::RegisterSpec for Ipsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipsr2::R`](R) reader structure"]
impl crate::Readable for Ipsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ipsr2::W`](W) writer structure"]
impl crate::Writable for Ipsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPSR2 to value 0"]
impl crate::Resettable for Ipsr2Spec {}
