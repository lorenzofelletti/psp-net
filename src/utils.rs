use alloc::{borrow::ToOwned, string::String};

/// Error type for net functions
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum NetError {
    /// Failed to load a net module
    LoadModuleFailed(String, i32),
    /// Failed to initialize
    InitFailed(String, i32),
    /// An error occurred when using a net function
    Error(String, i32),
}

impl NetError {
    pub fn load_module_failed(module: &str, error: i32) -> Self {
        NetError::LoadModuleFailed(module.to_owned(), error)
    }

    pub fn init_failed(fn_name: &str, error: i32) -> Self {
        NetError::InitFailed(fn_name.to_owned(), error)
    }

    pub fn error(fn_name: &str, error: i32) -> Self {
        NetError::Error(fn_name.to_owned(), error)
    }
}

/// Load net modules
///
/// # Errors
/// - [`NetError::LoadModuleFailed`] if the net module could not be loaded
#[allow(unused)]
#[inline]
pub fn load_net_modules() -> Result<(), NetError> {
    unsafe {
        let res = psp::sys::sceUtilityLoadNetModule(psp::sys::NetModule::NetCommon);
        if res != 0 {
            return Err(NetError::load_module_failed("", res));
        }

        let res = psp::sys::sceUtilityLoadNetModule(psp::sys::NetModule::NetInet);
        if res != 0 {
            return Err(NetError::load_module_failed("", res));
        }

        let res = psp::sys::sceUtilityLoadNetModule(psp::sys::NetModule::NetParseUri);
        if res != 0 {
            return Err(NetError::load_module_failed("", res));
        }

        let res = psp::sys::sceUtilityLoadNetModule(psp::sys::NetModule::NetHttp);
        if res != 0 {
            return Err(NetError::load_module_failed("", res));
        }

        Ok(())
    }
}

/// Initialize network
///
/// # Errors
/// - [`NetError::InitFailed`] if the net could not be initialized
#[allow(unused)]
#[inline]
pub fn net_init() -> Result<(), NetError> {
    unsafe {
        let res = psp::sys::sceNetInit(0x20000, 0x20, 0x1000, 0x20, 0x1000);
        if res != 0 {
            return Err(NetError::init_failed("sceNetInit", res));
        }

        let res = psp::sys::sceNetInetInit();
        if res != 0 {
            return Err(NetError::init_failed("sceNetInetInit", res));
        }

        let res = psp::sys::sceNetResolverInit();
        if res != 0 {
            return Err(NetError::init_failed("sceNetResolverInit", res));
        }

        let res = psp::sys::sceNetApctlInit(0x1600, 42);
        if res != 0 {
            return Err(NetError::init_failed("sceNetApctlInit", res));
        }
    }

    Ok(())
}

/// Select net config
///
/// # Errors
/// This function will return an error if selection fails.
/// The error, if any, will always be [`NetError::Error`].
///
/// # Notes
/// The netconfigs start from 1.
///
/// Remember that this function requires the [net modules](crate::utils::load_net_modules) to be loaded, and
/// [initialised](crate::utils::net_init) first.
#[allow(unused)]
#[inline]
pub fn select_netconfig(id: i32) -> Result<(), NetError> {
    unsafe {
        let res = psp::sys::sceUtilityCheckNetParam(id);
        if res != 0 {
            return Err(NetError::error("sceUtilityCheckNetParam", res));
        }
    }

    Ok(())
}

/// Select first net config
///
/// # Errors
/// This function will return an error if selection fails
#[allow(unused)]
#[inline]
pub fn select_first_netconfig() -> Result<(), NetError> {
    select_netconfig(1)
}
