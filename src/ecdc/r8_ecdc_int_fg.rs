#[doc = "Register `R8_ECDC_INT_FG` reader"]
pub type R = crate::R<R8EcdcIntFgSpec>;
#[doc = "Register `R8_ECDC_INT_FG` writer"]
pub type W = crate::W<R8EcdcIntFgSpec>;
#[doc = "Field `RB_ECDC_IF_EKDONE` reader - Key extension completion interrupt flag"]
pub type RbEcdcIfEkdoneR = crate::BitReader;
#[doc = "Field `RB_ECDC_IF_EKDONE` writer - Key extension completion interrupt flag"]
pub type RbEcdcIfEkdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_IF_SINGLE` reader - Single encryption and decryption completion interrupt flag"]
pub type RbEcdcIfSingleR = crate::BitReader;
#[doc = "Field `RB_ECDC_IF_SINGLE` writer - Single encryption and decryption completion interrupt flag"]
pub type RbEcdcIfSingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RB_ECDC_IF_WRSRAM` reader - Memory to memory encryption and decryption completion interrupt flag"]
pub type RbEcdcIfWrsramR = crate::BitReader;
#[doc = "Field `RB_ECDC_IF_WRSRAM` writer - Memory to memory encryption and decryption completion interrupt flag"]
pub type RbEcdcIfWrsramW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Key extension completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_ekdone(&self) -> RbEcdcIfEkdoneR {
        RbEcdcIfEkdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_single(&self) -> RbEcdcIfSingleR {
        RbEcdcIfSingleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_wrsram(&self) -> RbEcdcIfWrsramR {
        RbEcdcIfWrsramR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key extension completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_ekdone(&mut self) -> RbEcdcIfEkdoneW<'_, R8EcdcIntFgSpec> {
        RbEcdcIfEkdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Single encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_single(&mut self) -> RbEcdcIfSingleW<'_, R8EcdcIntFgSpec> {
        RbEcdcIfSingleW::new(self, 1)
    }
    #[doc = "Bit 2 - Memory to memory encryption and decryption completion interrupt flag"]
    #[inline(always)]
    pub fn rb_ecdc_if_wrsram(&mut self) -> RbEcdcIfWrsramW<'_, R8EcdcIntFgSpec> {
        RbEcdcIfWrsramW::new(self, 2)
    }
}
#[doc = "Interupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`r8_ecdc_int_fg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8_ecdc_int_fg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R8EcdcIntFgSpec;
impl crate::RegisterSpec for R8EcdcIntFgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`r8_ecdc_int_fg::R`](R) reader structure"]
impl crate::Readable for R8EcdcIntFgSpec {}
#[doc = "`write(|w| ..)` method takes [`r8_ecdc_int_fg::W`](W) writer structure"]
impl crate::Writable for R8EcdcIntFgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R8_ECDC_INT_FG to value 0"]
impl crate::Resettable for R8EcdcIntFgSpec {}
