//! Provides the convolution functionality from the CUDA cuDNN API.
//!
//! Includes the convolution and filter functionality.

use ::{API, Error};
use ffi::*;

impl API {
    //
    // cuDNN Filter
    //

    /// Creates a generic CUDA cuDNN Filter Descriptor.
    pub fn create_filter_descriptor() -> Result<cudnnFilterDescriptor_t, Error> {
        unsafe { API::ffi_create_filter_descriptor() }
    }

    /// Destroys a CUDA cuDNN Filter Descriptor.
    ///
    /// Should be called when freeing a CUDA::Descriptor to not trash up the CUDA device.
    pub fn destroy_filter_descriptor(desc: cudnnFilterDescriptor_t) -> Result<(), Error> {
        unsafe { API::ffi_destroy_filter_descriptor(desc) }
    }

    /// Initializes a generic CUDA cuDNN Filter Descriptor with specific properties.
    pub fn set_pooling_descriptor(
        desc: cudnnFilterDescriptor_t,
        data_type: cudnnDataType_t,
        nb_dims: ::libc::c_int,
        filter_dim_a: *const ::libc::c_int,
    ) -> Result<(), Error> {
        unsafe { API::ffi_set_filter_nd_descriptor(desc, data_type, nb_dims, filter_dim_a) }
    }

    unsafe fn ffi_create_filter_descriptor() -> Result<cudnnFilterDescriptor_t, Error> {
        let mut desc: cudnnFilterDescriptor_t = ::std::ptr::null_mut();
        match cudnnCreateFilterDescriptor(&mut desc) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(desc),
            cudnnStatus_t::CUDNN_STATUS_ALLOC_FAILED => Err(Error::AllocFailed("The resources could not be allocated.")),
            _ => Err(Error::Unknown("Unable to create generic CUDA cuDNN Filter Descriptor.")),
        }
    }

    unsafe fn ffi_destroy_filter_descriptor(desc: cudnnFilterDescriptor_t) -> Result<(), Error> {
        match cudnnDestroyFilterDescriptor(desc) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            _ => Err(Error::Unknown("Unable to destroy CUDA cuDNN Filter Descriptor.")),
        }
    }

    unsafe fn ffi_set_filter_nd_descriptor(
        desc: cudnnFilterDescriptor_t,
        data_type: cudnnDataType_t,
        nb_dims: ::libc::c_int,
        filter_dim_a: *const ::libc::c_int,
    ) -> Result<(), Error> {
        match cudnnSetFilterNdDescriptor(desc, data_type, nb_dims, filter_dim_a) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            cudnnStatus_t::CUDNN_STATUS_BAD_PARAM => Err(Error::BadParam("`filter_dim_a` has a negative element or invalid `data_type` provided.")),
            cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED => Err(Error::NotSupported("`nb_dims` exceeds CUDNN_DIM_MAX.")),
            _ => Err(Error::Unknown("Unable to set CUDA cuDNN Filter Descriptor.")),
        }
    }

    //
    // cuDNN Convolution
    //

    /// Creates a generic CUDA cuDNN Convolution Descriptor.
    pub fn create_convolution_descriptor() -> Result<cudnnConvolutionDescriptor_t, Error> {
        unsafe { API::ffi_create_convolution_descriptor() }
    }

    /// Destroys a CUDA cuDNN Convolution Descriptor.
    ///
    /// Should be called when freeing a CUDA::Descriptor to not trash up the CUDA device.
    pub fn destroy_convolution_descriptor(desc: cudnnConvolutionDescriptor_t) -> Result<(), Error> {
        unsafe { API::ffi_destroy_convolution_descriptor(desc) }
    }

    /// Initializes a generic CUDA cuDNN Convolution Descriptor with specific properties.
    pub fn set_convolution_descriptor(
        desc: cudnnConvolutionDescriptor_t,
        data_type: cudnnDataType_t,
        mode: cudnnConvolutionMode_t,
        array_length: ::libc::c_int,
        pad_a: *const ::libc::c_int,
        filter_stride_a: *const ::libc::c_int,
        upscale_a: *const ::libc::c_int,
    ) -> Result<(), Error> {
        unsafe { API::ffi_set_convolution_nd_descriptor(desc, data_type, mode, array_length, pad_a, filter_stride_a, upscale_a) }
    }

    /// Computes a convolution forward function.
    pub fn convolution_forward(
        handle: cudnnHandle_t,
        algo: cudnnConvolutionFwdAlgo_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        work_space: *mut ::libc::c_void,
        work_size_in_bytes: ::libc::size_t,
        alpha: *const ::libc::c_void,
        src_desc: cudnnTensorDescriptor_t,
        src_data: *const ::libc::c_void,
        filter_desc: cudnnFilterDescriptor_t,
        filter_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        dest_desc: cudnnTensorDescriptor_t,
        dest_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        unsafe { API::ffi_convolution_forward(handle, algo, conv_desc, work_space, work_size_in_bytes, alpha, src_desc, src_data, filter_desc, filter_data, beta, dest_desc, dest_data) }
    }

    /// Computes a convolution backward function w.r.t the bias.
    pub fn convolution_backward_bias(
        handle: cudnnHandle_t,
        alpha: *const ::libc::c_void,
        src_desc: cudnnTensorDescriptor_t,
        src_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        dest_desc: cudnnTensorDescriptor_t,
        dest_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        unsafe { API::ffi_convolution_backward_bias(handle, alpha, src_desc, src_data, beta, dest_desc, dest_data) }
    }

    /// Computes a convolution backward function w.r.t filter coefficient.
    pub fn convolution_backward_filter(
        handle: cudnnHandle_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        work_space: *mut ::libc::c_void,
        work_size_in_bytes: ::libc::size_t,
        alpha: *const ::libc::c_void,
        src_desc: cudnnTensorDescriptor_t,
        src_data: *const ::libc::c_void,
        diff_desc: cudnnTensorDescriptor_t,
        diff_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        grad_desc: cudnnFilterDescriptor_t,
        grad_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        unsafe { API::ffi_convolution_backward_filter(handle, algo, conv_desc, work_space, work_size_in_bytes, alpha, src_desc, src_data, diff_desc, diff_data, beta, grad_desc, grad_data) }
    }

    /// Computes a convolution backward function w.r.t the output tensor.
    pub fn convolution_backward_data(
        handle: cudnnHandle_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        work_space: *mut ::libc::c_void,
        work_size_in_bytes: ::libc::size_t,
        alpha: *const ::libc::c_void,
        filter_desc: cudnnFilterDescriptor_t,
        filter_data: *const ::libc::c_void,
        diff_desc: cudnnTensorDescriptor_t,
        diff_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        grad_desc: cudnnTensorDescriptor_t,
        grad_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        unsafe { API::ffi_convolution_backward_data(handle, algo, conv_desc, work_space, work_size_in_bytes, alpha, filter_desc, filter_data, diff_desc, diff_data, beta, grad_desc, grad_data) }
    }

    unsafe fn ffi_create_convolution_descriptor() -> Result<cudnnConvolutionDescriptor_t, Error> {
        let mut desc: cudnnConvolutionDescriptor_t = ::std::ptr::null_mut();
        match cudnnCreateConvolutionDescriptor(&mut desc) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(desc),
            cudnnStatus_t::CUDNN_STATUS_ALLOC_FAILED => Err(Error::AllocFailed("The resources could not be allocated.")),
            _ => Err(Error::Unknown("Unable to create generic CUDA cuDNN Convolution Descriptor.")),
        }
    }

    unsafe fn ffi_destroy_convolution_descriptor(desc: cudnnConvolutionDescriptor_t) -> Result<(), Error> {
        match cudnnDestroyConvolutionDescriptor(desc) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            _ => Err(Error::Unknown("Unable to destroy CUDA cuDNN Convolution Descriptor.")),
        }
    }

    unsafe fn ffi_set_convolution_nd_descriptor(
        desc: cudnnConvolutionDescriptor_t,
        data_type: cudnnDataType_t,
        mode: cudnnConvolutionMode_t,
        array_length: ::libc::c_int,
        pad_a: *const ::libc::c_int,
        filter_stride_a: *const ::libc::c_int,
        upscale_a: *const ::libc::c_int,
    ) -> Result<(), Error> {
        match cudnnSetConvolutionNdDescriptor_v3(desc, array_length, pad_a, filter_stride_a, upscale_a, mode, data_type) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            cudnnStatus_t::CUDNN_STATUS_BAD_PARAM => Err(Error::BadParam("At least one of the following conditions are met: `desc` is NULL. `array_length` is negative, `mode` or `data_type` is invalid, element of `pad_a` is negative, element of `stride_a` is negative or zero.")),
            cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED => Err(Error::NotSupported("At least one of the following conditions are met: `array_length` is greater than CUDNN_DIM_MAX. `upscale_a` contains an element different from 1.")),
            _ => Err(Error::Unknown("Unable to set CUDA cuDNN Convolution Descriptor.")),
        }
    }

    unsafe fn ffi_convolution_forward(
        handle: cudnnHandle_t,
        algo: cudnnConvolutionFwdAlgo_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        work_space: *mut ::libc::c_void,
        work_size_in_bytes: ::libc::size_t,
        alpha: *const ::libc::c_void,
        src_desc: cudnnTensorDescriptor_t,
        src_data: *const ::libc::c_void,
        filter_desc: cudnnFilterDescriptor_t,
        filter_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        dest_desc: cudnnTensorDescriptor_t,
        dest_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        match cudnnConvolutionForward(handle, alpha, src_desc, src_data, filter_desc, filter_data, conv_desc, algo, work_space, work_size_in_bytes, beta, dest_desc, dest_data) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            cudnnStatus_t::CUDNN_STATUS_BAD_PARAM => Err(Error::BadParam("At least one of the following conditions are met: At least one of the following is NULL: `handle`, `src_desc`, `filter_desc`, `conv_desc`, `dest_desc`, `src_data`, `alpha`, `beta`. `src_desc` and `dest_desc` have a non-matching number of dimensions. `src_desc` and `filter_desc` have a non-matching number of dimensions. `src_desc` has fewer than three number of dimensions. `src_desc`s number of dimensions is not equal to `conv_desc`s `array_length` + 2. `src_desc` and `filter_desc` have a non-matching number of input feature maps per image. `src_desc`, `filter_desc` and `dest_desc` have a non-matching data type. For some spatial dimension, `filter_desc` has a spatial size that is larger than the input spatial size (including zero-padding size).")),
            cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED => Err(Error::NotSupported("At least one of the following conditions are met: `src_desc` or `dest_desc` have negative tensor striding. `src_desc`, `filter_desc` or `dest_desc` has a number of dimensions that is not 4 or 5. The chosen algo does not support the parameters provided; see the reference for exhaustive list of parameter support for each algo")),
            _ => Err(Error::Unknown("Unable to compute CUDA cuDNN convolutional forward.")),
        }
    }

    unsafe fn ffi_convolution_backward_bias(
        handle: cudnnHandle_t,
        alpha: *const ::libc::c_void,
        src_desc: cudnnTensorDescriptor_t,
        src_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        dest_desc: cudnnTensorDescriptor_t,
        dest_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        match cudnnConvolutionBackwardBias(handle, alpha, src_desc, src_data, beta, dest_desc, dest_data) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            cudnnStatus_t::CUDNN_STATUS_BAD_PARAM => Err(Error::BadParam("At least one of the following conditions are met: One of the parameters  n,h,w of the output tensor is not 1. The numbers of feature maps of the input tensor and output tensor differ. The  dataType of the two tensor descriptors are different.")),
            _ => Err(Error::Unknown("Unable to compute CUDA cuDNN convolutional backward bias.")),
        }
    }

    unsafe fn ffi_convolution_backward_filter(
        handle: cudnnHandle_t,
        algo: cudnnConvolutionBwdFilterAlgo_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        work_space: *mut ::libc::c_void,
        work_size_in_bytes: ::libc::size_t,
        alpha: *const ::libc::c_void,
        src_desc: cudnnTensorDescriptor_t,
        src_data: *const ::libc::c_void,
        diff_desc: cudnnTensorDescriptor_t,
        diff_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        grad_desc: cudnnFilterDescriptor_t,
        grad_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        match cudnnConvolutionBackwardFilter_v3(handle, alpha, src_desc, src_data, diff_desc, diff_data, conv_desc, algo, work_space, work_size_in_bytes, beta, grad_desc, grad_data) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            cudnnStatus_t::CUDNN_STATUS_BAD_PARAM => Err(Error::BadParam("At least one of the following conditions are met: At least one of the following is NULL: `handle`, `src_desc`, `diff_desc`, `conv_desc`, `grad_desc`, `src_data`, `diff_data`, `grad_data`, `alpha`, `beta`. `src_desc` and `diff_desc` have a non-matching number of dimensions. `src_desc` and `grad_desc` have a non-matching number of dimensions. `src_desc` has fewer than three number of dimensions. `src_desc`, `diff_desc` and `grad_desc` have a non-matching data type. `src_desc` and `grad_desc` have a non-matching number of input feature maps per image.")),
            cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED => Err(Error::NotSupported("At least one of the following conditions are met: `src_desc` or `diff_desc` have negative tensor striding. `src_desc`, `diff_desc` or `grad_desc` has a number of dimensions that is not 4 or 5. The chosen algo does not support the parameters provided; see the reference for exhaustive list of parameter support for each algo")),
            cudnnStatus_t::CUDNN_STATUS_MAPPING_ERROR => Err(Error::MappingError("An error occurs during the texture binding of the filter data.")),
            cudnnStatus_t::CUDNN_STATUS_EXECUTION_FAILED => Err(Error::ExecutionFailed("Execution failed to launch on GPU.")),
            _ => Err(Error::Unknown("Unable to compute CUDA cuDNN convolutional backward filter.")),
        }
    }

    unsafe fn ffi_convolution_backward_data(
        handle: cudnnHandle_t,
        algo: cudnnConvolutionBwdDataAlgo_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        work_space: *mut ::libc::c_void,
        work_size_in_bytes: ::libc::size_t,
        alpha: *const ::libc::c_void,
        filter_desc: cudnnFilterDescriptor_t,
        filter_data: *const ::libc::c_void,
        diff_desc: cudnnTensorDescriptor_t,
        diff_data: *const ::libc::c_void,
        beta: *const ::libc::c_void,
        grad_desc: cudnnTensorDescriptor_t,
        grad_data: *mut ::libc::c_void,
    ) -> Result<(), Error> {
        match cudnnConvolutionBackwardData_v3(handle, alpha, filter_desc, filter_data, diff_desc, diff_data, conv_desc, algo, work_space, work_size_in_bytes, beta, grad_desc, grad_data) {
            cudnnStatus_t::CUDNN_STATUS_SUCCESS => Ok(()),
            cudnnStatus_t::CUDNN_STATUS_BAD_PARAM => Err(Error::BadParam("At least one of the following conditions are met: At least one of the following is NULL: `handle`, `diff_desc`, `filter_desc`, `conv_desc`, `grad_desc`, `diff_data`, `filter_data`, `grad_data`, `alpha`, `beta`. `filter_desc` and `diff_desc` have a non-matching number of dimensions. `filter_desc` and `grad_desc` have a non-matching number of dimensions. `filter_desc has fewer than three number of dimensions. `filter_desc`, `grad_desc` and `diff_desc` have a non-matching data type. `filter_desc` and `grad_desc` have a non-matching number of input feature maps per image. `diff_desc`s spatial sizes do not match with the expected size as determined by `cudnnGetConvolutionNdForwardOutputDim()`.")),
            cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED => Err(Error::NotSupported("At least one of the following conditions are met:  `diff_desc` or `grad_desc` have negative tensor striding. `diff_desc`, `filter_desc` or `grad_desc` has a number of dimensions that is not 4 or 5. The chosen algo does not support the parameters provided; see the reference for exhaustive list of parameter support for each algo")),
            cudnnStatus_t::CUDNN_STATUS_MAPPING_ERROR => Err(Error::MappingError("An error occurs during the texture binding of the filter data or the input differential tensor data.")),
            cudnnStatus_t::CUDNN_STATUS_EXECUTION_FAILED => Err(Error::ExecutionFailed("Execution failed to launch on GPU.")),
            _ => Err(Error::Unknown("Unable to compute CUDA cuDNN convolutional backward data.")),
        }
    }
}