#[doc = "Register `GISR` reader"]
pub type R = crate::R<GisrSpec>;
#[doc = "Field `NESTSTA` reader - NESTSTA"]
pub type NeststaR = crate::FieldReader;
#[doc = "Field `GACTSTA` reader - GACTSTA"]
pub type GactstaR = crate::BitReader;
#[doc = "Field `GPENDSTA` reader - GPENDSTA"]
pub type GpendstaR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - NESTSTA"]
    #[inline(always)]
    pub fn neststa(&self) -> NeststaR {
        NeststaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GACTSTA"]
    #[inline(always)]
    pub fn gactsta(&self) -> GactstaR {
        GactstaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPENDSTA"]
    #[inline(always)]
    pub fn gpendsta(&self) -> GpendstaR {
        GpendstaR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Global Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GisrSpec;
impl crate::RegisterSpec for GisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gisr::R`](R) reader structure"]
impl crate::Readable for GisrSpec {}
#[doc = "`reset()` method sets GISR to value 0"]
impl crate::Resettable for GisrSpec {}
