#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - SysTick reload value register"]
    pub rvr: RVR,
    #[doc = "0x08 - SysTick current value register"]
    pub cvr: CVR,
    #[doc = "0x0c - SysTick calibration value register"]
    pub calib: CALIB,
}
#[doc = "SysTick control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "SysTick control and status register"]
pub mod csr;
#[doc = "SysTick reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rvr](rvr) module"]
pub type RVR = crate::Reg<u32, _RVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RVR;
#[doc = "`read()` method returns [rvr::R](rvr::R) reader structure"]
impl crate::Readable for RVR {}
#[doc = "`write(|w| ..)` method takes [rvr::W](rvr::W) writer structure"]
impl crate::Writable for RVR {}
#[doc = "SysTick reload value register"]
pub mod rvr;
#[doc = "SysTick current value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cvr](cvr) module"]
pub type CVR = crate::Reg<u32, _CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVR;
#[doc = "`read()` method returns [cvr::R](cvr::R) reader structure"]
impl crate::Readable for CVR {}
#[doc = "`write(|w| ..)` method takes [cvr::W](cvr::W) writer structure"]
impl crate::Writable for CVR {}
#[doc = "SysTick current value register"]
pub mod cvr;
#[doc = "SysTick calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [calib](calib) module"]
pub type CALIB = crate::Reg<u32, _CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIB;
#[doc = "`read()` method returns [calib::R](calib::R) reader structure"]
impl crate::Readable for CALIB {}
#[doc = "`write(|w| ..)` method takes [calib::W](calib::W) writer structure"]
impl crate::Writable for CALIB {}
#[doc = "SysTick calibration value register"]
pub mod calib;
