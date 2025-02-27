// Copyright (c) 2020-2021 Via Technology Ltd. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// cl_d3d10_device_source_khr
use super::Result;
#[allow(unused_imports)]
use cl3::d3d10;
#[allow(unused_imports)]
use cl3::d3d11;
use cl3::device;
#[allow(unused_imports)]
use cl3::dx9_media_sharing;
#[allow(unused_imports)]
use cl3::ext;
#[allow(unused_imports)]
use cl3::ffi::cl_d3d10::{cl_d3d10_device_set_khr, cl_d3d10_device_source_khr};
#[allow(unused_imports)]
use cl3::ffi::cl_d3d11::{cl_d3d11_device_set_khr, cl_d3d11_device_source_khr};
#[allow(unused_imports)]
use cl3::ffi::cl_dx9_media_sharing::{
    cl_dx9_device_set_intel, cl_dx9_device_source_intel, cl_dx9_media_adapter_set_khr,
    cl_dx9_media_adapter_type_khr,
};
use cl3::platform;
#[allow(unused_imports)]
use cl3::program;
#[allow(unused_imports)]
use cl3::types::{
    cl_device_id, cl_device_type, cl_name_version, cl_platform_id, cl_uint, cl_ulong, cl_version,
};
#[allow(unused_imports)]
use libc::{c_void, intptr_t};

/// An OpenCL platform id and methods to query it.  
/// The query methods calls clGetPlatformInfo with the relevant param_name, see:
/// [Platform Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#platform-queries-table).
#[derive(Copy, Clone, Debug)]
pub struct Platform {
    id: intptr_t,
}

unsafe impl Send for Platform {}
unsafe impl Sync for Platform {}

impl Platform {
    pub fn new(id: cl_platform_id) -> Platform {
        Platform { id: id as intptr_t }
    }

    /// Accessor for the underlying platform id.
    pub fn id(&self) -> cl_platform_id {
        self.id as cl_platform_id
    }

    /// Get the list of available devices of the given type on the Platform.
    /// # Examples
    /// ```
    /// use opencl3::platform::get_platforms;
    /// use opencl3::device::CL_DEVICE_TYPE_GPU;
    ///
    /// let platforms = get_platforms().unwrap();
    /// assert!(0 < platforms.len());
    ///
    /// // Choose a the first platform
    /// let platform = &platforms[0];
    /// let device_ids = platform.get_devices(CL_DEVICE_TYPE_GPU).unwrap();
    /// println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
    /// assert!(0 < device_ids.len());
    /// ```
    pub fn get_devices(&self, device_type: cl_device_type) -> Result<Vec<cl_device_id>> {
        Ok(device::get_device_ids(self.id(), device_type)?)
    }

    #[cfg(feature = "cl_khr_dx9_media_sharing")]
    pub fn get_devices_from_dx9_media_adapter_khr(
        &self,
        media_adapter_type: &[cl_dx9_media_adapter_type_khr],
        media_adapters: &[c_void],
        media_adapter_set: cl_dx9_media_adapter_set_khr,
    ) -> Result<Vec<cl_device_id>> {
        Ok(
            dx9_media_sharing::get_device_ids_from_dx9_media_adapter_khr(
                self.id(),
                media_adapters.len() as cl_uint,
                media_adapter_type.as_ptr() as *mut cl_dx9_media_adapter_type_khr,
                media_adapters.as_ptr() as *mut c_void,
                media_adapter_set,
            )?,
        )
    }

    #[cfg(feature = "cl_khr_dx9_media_sharing")]
    pub fn get_device_ids_from_dx9_intel(
        &self,
        dx9_device_source: cl_dx9_device_source_intel,
        dx9_object: *mut c_void,
        dx9_device_set: cl_dx9_device_set_intel,
    ) -> Result<Vec<cl_device_id>> {
        Ok(dx9_media_sharing::get_device_ids_from_dx9_intel(
            self.id(),
            dx9_device_source,
            dx9_object,
            dx9_device_set,
        )?)
    }

    #[cfg(feature = "cl_khr_d3d10_sharing")]
    pub fn get_devices_from_dx3d10_khr(
        &self,
        d3d_device_source: cl_d3d10_device_source_khr,
        d3d_object: *mut c_void,
        d3d_device_set: cl_d3d10_device_set_khr,
    ) -> Result<Vec<cl_device_id>> {
        Ok(d3d10::get_device_ids_from_dx3d10_khr(
            self.id(),
            d3d_device_source,
            d3d_object,
            d3d_device_set,
        )?)
    }

    #[cfg(feature = "cl_khr_d3d11_sharing")]
    pub fn get_devices_from_dx3d11_khr(
        &self,
        d3d_device_source: cl_d3d11_device_source_khr,
        d3d_object: *mut c_void,
        d3d_device_set: cl_d3d11_device_set_khr,
    ) -> Result<Vec<cl_device_id>> {
        Ok(d3d11::get_device_ids_from_dx3d11_khr(
            self.id(),
            d3d_device_source,
            d3d_object,
            d3d_device_set,
        )?)
    }

    /// The OpenCL profile supported by the Platform,
    /// it can be FULL_PROFILE or EMBEDDED_PROFILE.  
    pub fn profile(&self) -> Result<String> {
        Ok(
            platform::get_platform_info(self.id(), platform::PlatformInfo::CL_PLATFORM_PROFILE)?
                .to_string(),
        )
    }

    /// The OpenCL profile version supported by the Platform,
    /// e.g. OpenCL 1.2, OpenCL 2.0, OpenCL 2.1, etc.  
    pub fn version(&self) -> Result<String> {
        Ok(
            platform::get_platform_info(self.id(), platform::PlatformInfo::CL_PLATFORM_VERSION)?
                .to_string(),
        )
    }

    /// The OpenCL Platform name string.  
    pub fn name(&self) -> Result<String> {
        Ok(
            platform::get_platform_info(self.id(), platform::PlatformInfo::CL_PLATFORM_NAME)?
                .to_string(),
        )
    }

    /// The OpenCL Platform vendor string.  
    pub fn vendor(&self) -> Result<String> {
        Ok(
            platform::get_platform_info(self.id(), platform::PlatformInfo::CL_PLATFORM_VENDOR)?
                .to_string(),
        )
    }

    /// A space separated list of extension names supported by the Platform.  
    pub fn extensions(&self) -> Result<String> {
        Ok(
            platform::get_platform_info(self.id(), platform::PlatformInfo::CL_PLATFORM_EXTENSIONS)?
                .to_string(),
        )
    }

    /// The resolution of the host timer in nanoseconds as used by
    /// clGetDeviceAndHostTimer.  
    /// CL_VERSION_2_1
    pub fn host_timer_resolution(&self) -> Result<cl_ulong> {
        Ok(platform::get_platform_info(
            self.id(),
            platform::PlatformInfo::CL_PLATFORM_HOST_TIMER_RESOLUTION,
        )?
        .to_ulong())
    }

    /// The detailed (major, minor, patch) version supported by the platform.  
    /// CL_VERSION_3_0
    pub fn numeric_version(&self) -> Result<cl_version> {
        Ok(platform::get_platform_info(
            self.id(),
            platform::PlatformInfo::CL_PLATFORM_NUMERIC_VERSION,
        )?
        .to_uint())
    }

    /// An array of description (name and version) structures that lists all the
    /// extensions supported by the platform.  
    /// CL_VERSION_3_0
    pub fn extensions_with_version(&self) -> Result<Vec<cl_name_version>> {
        Ok(platform::get_platform_info(
            self.id(),
            platform::PlatformInfo::CL_PLATFORM_EXTENSIONS_WITH_VERSION,
        )?
        .to_vec_name_version())
    }

    /// Unload an OpenCL compiler for a platform.
    /// CL_VERSION_1_2
    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn unload_compiler(&self) -> Result<()> {
        Ok(program::unload_platform_compiler(self.id())?)
    }
}

/// Get the available OpenCL platforms.  
/// # Examples
/// ```
/// use opencl3::platform::get_platforms;
///
/// let platforms = get_platforms().unwrap();
/// println!("Number of OpenCL platforms: {}", platforms.len());
/// assert!(0 < platforms.len());
/// ```
/// returns a Result containing a vector of available Platforms
/// or the error code from the OpenCL C API function.
pub fn get_platforms() -> Result<Vec<Platform>> {
    let platform_ids = platform::get_platform_ids()?;
    Ok(platform_ids
        .iter()
        .map(|id| Platform::new(*id))
        .collect::<Vec<Platform>>())
}

#[cfg(feature = "cl_khr_icd")]
pub fn icd_get_platform_ids_khr() -> Result<Vec<Platform>> {
    let platform_ids = ext::icd_get_platform_ids_khr()?;
    Ok(platform_ids
        .iter()
        .map(|id| Platform::new(*id))
        .collect::<Vec<Platform>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platforms() {
        let platforms = get_platforms().unwrap();
        println!("Number of platforms: {}", platforms.len());
        assert!(0 < platforms.len());

        for platform in platforms {
            println!("Platform Debug Trait: {:?}", platform);
            println!("CL_PLATFORM_NAME: {}", platform.name().unwrap());
            println!("CL_PLATFORM_PROFILE: {}", platform.profile().unwrap());

            let value = platform.version().unwrap();
            println!("CL_PLATFORM_VERSION: {:?}", value);

            println!("CL_PLATFORM_VENDOR: {}", platform.vendor().unwrap());
            println!(
                "CL_PLATFORM_EXTENSIONS: {:?}",
                platform.extensions().unwrap()
            );

            // CL_VERSION_2_1 value, may not be supported
            match platform.host_timer_resolution() {
                Ok(value) => {
                    println!("CL_PLATFORM_HOST_TIMER_RESOLUTION: {}", value)
                }
                Err(e) => println!(
                    "OpenCL error, CL_PLATFORM_HOST_TIMER_RESOLUTION: {:?}, {}",
                    e, e
                ),
            };

            println!();
        }
    }
}
