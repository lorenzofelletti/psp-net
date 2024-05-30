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
    /// Net config Do not exist
    NetConfigNotExist,
    /// Timeout
    Timeout,
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

/// Check existence of net config
///
/// # Errors
/// - [`NetError::NetConfigNotExist`] if the net config does not exist
///
/// # Notes
/// The netconfigs start from 1.
///
/// Remember that this function requires the [net modules](crate::utils::load_net_modules) to be loaded, and
/// [initialised](crate::utils::net_init) first.
#[allow(unused)]
#[inline]
pub fn check_netconfig_existence(id: i32) -> Result<(), NetError> {
    unsafe {
        let res = psp::sys::sceUtilityCheckNetParam(id);
        if res != 0 {
            return Err(NetError::NetConfigNotExist);
        }
    }

    Ok(())
}

/// Check existence of first net config
///
/// # Errors
/// Same as [`check_netconfig_existence`](crate::utils::check_netconfig_existence)
#[allow(unused)]
#[inline]
pub fn check_first_netconfig_existence() -> Result<(), NetError> {
    check_netconfig_existence(1)
}

/// Initialise a connection to an access point
///
/// # Parameters
/// - `connection_id`: The ID of the access point to connect to
///
/// # Errors
/// [`NetError::Error`] if the connection initialisation failed
///
/// # Notes
/// The access point ID is a number greater or equal to 1. It is the index of the
/// access point in the netconfig.
///
/// This function does not block until the connection is established.
/// Use [`block_until_connected`](crate::utils::block_until_connected) right after
/// this function to block until the connection is established.
pub fn init_connection_to_access_point(connection_id: i32) -> Result<(), NetError> {
    unsafe {
        let res = psp::sys::sceNetApctlConnect(connection_id);
        if res != 0 {
            return Err(NetError::error("sceNetApctlConnect", res));
        }
    }

    Ok(())
}

/// Block until connected
///
/// Polls the access point state until connection is established, `desist_after` times
/// is reached, or an error occurs.
/// The polling is done at 50ms intervals using [`sceKernelDelayThread`](psp::sys::sceKernelDelayThread).
///
/// # Parameters
/// - `desist_after`: The number of times to poll the state before desisting
///
/// # Errors
/// Same as [`init_connection_to_access_point`](crate::utils::init_connection_to_access_point)
#[inline]
pub fn block_until_connected(desist_after: usize) -> Result<(), NetError> {
    let mut state: psp::sys::ApctlState = unsafe { core::mem::zeroed() };
    for _ in 0..desist_after {
        unsafe {
            let err = psp::sys::sceNetApctlGetState(&mut state);
            if err != 0 {
                return Err(NetError::error("sceNetApctlGetState", err));
            }
        }
        if let psp::sys::ApctlState::GotIp = state {
            return Ok(());
        }
        unsafe { psp::sys::sceKernelDelayThread(50_000) };
    }

    Err(NetError::Timeout)
}
