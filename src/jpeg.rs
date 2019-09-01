#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec configuration register 0"]
    pub jpeg_confr0: JPEG_CONFR0,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub jpeg_confr1: JPEG_CONFR1,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub jpeg_confr2: JPEG_CONFR2,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub jpeg_confr3: JPEG_CONFR3,
    #[doc = "0x10 - JPEG codec configuration register 4"]
    pub jpeg_confr4: JPEG_CONFR4,
    #[doc = "0x14 - JPEG codec configuration register 5"]
    pub jpeg_confr5: JPEG_CONFR5,
    #[doc = "0x18 - JPEG codec configuration register 6"]
    pub jpeg_confr6: JPEG_CONFR6,
    #[doc = "0x1c - JPEG codec configuration register 7"]
    pub jpeg_confr7: JPEG_CONFR7,
    #[doc = "0x20 - JPEG control register"]
    pub jpeg_cr: JPEG_CR,
    #[doc = "0x24 - JPEG status register"]
    pub jpeg_sr: JPEG_SR,
    #[doc = "0x28 - JPEG clear flag register"]
    pub jpeg_cfr: JPEG_CFR,
    #[doc = "0x2c - JPEG data input register"]
    pub jpeg_dir: JPEG_DIR,
    #[doc = "0x30 - JPEG data output register"]
    pub jpeg_dor: JPEG_DOR,
}
#[doc = "JPEG codec configuration register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr0](jpeg_confr0) module"]
pub type JPEG_CONFR0 = crate::Reg<u32, _JPEG_CONFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR0;
#[doc = "`write(|w| ..)` method takes [jpeg_confr0::W](jpeg_confr0::W) writer structure"]
impl crate::Writable for JPEG_CONFR0 {}
#[doc = "JPEG codec configuration register 0"]
pub mod jpeg_confr0;
#[doc = "JPEG codec configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr1](jpeg_confr1) module"]
pub type JPEG_CONFR1 = crate::Reg<u32, _JPEG_CONFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR1;
#[doc = "`read()` method returns [jpeg_confr1::R](jpeg_confr1::R) reader structure"]
impl crate::Readable for JPEG_CONFR1 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr1::W](jpeg_confr1::W) writer structure"]
impl crate::Writable for JPEG_CONFR1 {}
#[doc = "JPEG codec configuration register 1"]
pub mod jpeg_confr1;
#[doc = "JPEG codec configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr2](jpeg_confr2) module"]
pub type JPEG_CONFR2 = crate::Reg<u32, _JPEG_CONFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR2;
#[doc = "`read()` method returns [jpeg_confr2::R](jpeg_confr2::R) reader structure"]
impl crate::Readable for JPEG_CONFR2 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr2::W](jpeg_confr2::W) writer structure"]
impl crate::Writable for JPEG_CONFR2 {}
#[doc = "JPEG codec configuration register 2"]
pub mod jpeg_confr2;
#[doc = "JPEG codec configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr3](jpeg_confr3) module"]
pub type JPEG_CONFR3 = crate::Reg<u32, _JPEG_CONFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR3;
#[doc = "`read()` method returns [jpeg_confr3::R](jpeg_confr3::R) reader structure"]
impl crate::Readable for JPEG_CONFR3 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr3::W](jpeg_confr3::W) writer structure"]
impl crate::Writable for JPEG_CONFR3 {}
#[doc = "JPEG codec configuration register 3"]
pub mod jpeg_confr3;
#[doc = "JPEG codec configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr4](jpeg_confr4) module"]
pub type JPEG_CONFR4 = crate::Reg<u32, _JPEG_CONFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR4;
#[doc = "`read()` method returns [jpeg_confr4::R](jpeg_confr4::R) reader structure"]
impl crate::Readable for JPEG_CONFR4 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr4::W](jpeg_confr4::W) writer structure"]
impl crate::Writable for JPEG_CONFR4 {}
#[doc = "JPEG codec configuration register 4"]
pub mod jpeg_confr4;
#[doc = "JPEG codec configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr5](jpeg_confr5) module"]
pub type JPEG_CONFR5 = crate::Reg<u32, _JPEG_CONFR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR5;
#[doc = "`read()` method returns [jpeg_confr5::R](jpeg_confr5::R) reader structure"]
impl crate::Readable for JPEG_CONFR5 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr5::W](jpeg_confr5::W) writer structure"]
impl crate::Writable for JPEG_CONFR5 {}
#[doc = "JPEG codec configuration register 5"]
pub mod jpeg_confr5;
#[doc = "JPEG codec configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr6](jpeg_confr6) module"]
pub type JPEG_CONFR6 = crate::Reg<u32, _JPEG_CONFR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR6;
#[doc = "`read()` method returns [jpeg_confr6::R](jpeg_confr6::R) reader structure"]
impl crate::Readable for JPEG_CONFR6 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr6::W](jpeg_confr6::W) writer structure"]
impl crate::Writable for JPEG_CONFR6 {}
#[doc = "JPEG codec configuration register 6"]
pub mod jpeg_confr6;
#[doc = "JPEG codec configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_confr7](jpeg_confr7) module"]
pub type JPEG_CONFR7 = crate::Reg<u32, _JPEG_CONFR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR7;
#[doc = "`read()` method returns [jpeg_confr7::R](jpeg_confr7::R) reader structure"]
impl crate::Readable for JPEG_CONFR7 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr7::W](jpeg_confr7::W) writer structure"]
impl crate::Writable for JPEG_CONFR7 {}
#[doc = "JPEG codec configuration register 7"]
pub mod jpeg_confr7;
#[doc = "JPEG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_cr](jpeg_cr) module"]
pub type JPEG_CR = crate::Reg<u32, _JPEG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CR;
#[doc = "`read()` method returns [jpeg_cr::R](jpeg_cr::R) reader structure"]
impl crate::Readable for JPEG_CR {}
#[doc = "`write(|w| ..)` method takes [jpeg_cr::W](jpeg_cr::W) writer structure"]
impl crate::Writable for JPEG_CR {}
#[doc = "JPEG control register"]
pub mod jpeg_cr;
#[doc = "JPEG status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_sr](jpeg_sr) module"]
pub type JPEG_SR = crate::Reg<u32, _JPEG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_SR;
#[doc = "`read()` method returns [jpeg_sr::R](jpeg_sr::R) reader structure"]
impl crate::Readable for JPEG_SR {}
#[doc = "JPEG status register"]
pub mod jpeg_sr;
#[doc = "JPEG clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_cfr](jpeg_cfr) module"]
pub type JPEG_CFR = crate::Reg<u32, _JPEG_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CFR;
#[doc = "`write(|w| ..)` method takes [jpeg_cfr::W](jpeg_cfr::W) writer structure"]
impl crate::Writable for JPEG_CFR {}
#[doc = "JPEG clear flag register"]
pub mod jpeg_cfr;
#[doc = "JPEG data input register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_dir](jpeg_dir) module"]
pub type JPEG_DIR = crate::Reg<u32, _JPEG_DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_DIR;
#[doc = "`write(|w| ..)` method takes [jpeg_dir::W](jpeg_dir::W) writer structure"]
impl crate::Writable for JPEG_DIR {}
#[doc = "JPEG data input register"]
pub mod jpeg_dir;
#[doc = "JPEG data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jpeg_dor](jpeg_dor) module"]
pub type JPEG_DOR = crate::Reg<u32, _JPEG_DOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_DOR;
#[doc = "`read()` method returns [jpeg_dor::R](jpeg_dor::R) reader structure"]
impl crate::Readable for JPEG_DOR {}
#[doc = "JPEG data output register"]
pub mod jpeg_dor;
