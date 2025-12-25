#[doc = "Register `IPSR1` reader"]
pub type R = crate::R<Ipsr1Spec>;
#[doc = "Register `IPSR1` writer"]
pub type W = crate::W<Ipsr1Spec>;
#[doc = "Field `PENDSET` reader - PENDSET"]
pub type PendsetR = crate::FieldReader<u32>;
#[doc = "Field `PENDSET` writer - PENDSET"]
pub type PendsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&self) -> PendsetR {
        PendsetR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&mut self) -> PendsetW<'_, Ipsr1Spec> {
        PendsetW::new(self, 12)
    }
}
#[doc = "Interrupt Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipsr1Spec;
impl crate::RegisterSpec for Ipsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipsr1::R`](R) reader structure"]
impl crate::Readable for Ipsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ipsr1::W`](W) writer structure"]
impl crate::Writable for Ipsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPSR1 to value 0"]
impl crate::Resettable for Ipsr1Spec {}
