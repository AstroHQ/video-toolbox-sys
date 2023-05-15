use crate::core_foundation_sys::base::{CFAllocatorRef, CFTypeID, CFTypeRef, OSStatus};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::url::CFURLRef;
use crate::core_media_sys::{CMItemCount, CMSampleBufferRef, CMTimeRange};
use crate::libc::{c_float, c_void};

pub type VTFrameSiloRef = CFTypeRef;
pub type Float32 = c_float;

#[link(name = "VideoToolBox", kind = "framework")]
extern "C" {

    pub fn VTFrameSiloCreate(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        timeRange: CMTimeRange,
        options: CFDictionaryRef,
        multiPassStorageOut: *mut VTFrameSiloRef,
    ) -> OSStatus;
    pub fn VTFrameSiloAddSampleBuffer(
        silo: VTFrameSiloRef,
        sampleBuffer: CMSampleBufferRef,
    ) -> OSStatus;
    pub fn VTFrameSiloSetTimeRangesForNextPass(
        silo: VTFrameSiloRef,
        timeRangeCount: CMItemCount,
        timeRangeArray: *const CMTimeRange,
    ) -> OSStatus;

    pub fn VTFrameSiloCallBlockForEachSampleBuffer(
        silo: VTFrameSiloRef,
        timeRange: CMTimeRange,
        handler: extern "C" fn(sampleBuffer: CMSampleBufferRef) -> OSStatus,
    ) -> OSStatus;
    pub fn VTFrameSiloCallFunctionForEachSampleBuffer(
        silo: VTFrameSiloRef,
        timeRange: CMTimeRange,
        refcon: *mut c_void,
        callback: extern "C" fn(refcon: *mut c_void, sampleBuffer: CMSampleBufferRef) -> OSStatus,
    ) -> OSStatus;

    pub fn VTFrameSiloGetProgressOfCurrentPass(
        silo: VTFrameSiloRef,
        progressOut: *mut Float32,
    ) -> OSStatus;
    pub fn VTFrameSiloGetTypeID() -> CFTypeID;

}
