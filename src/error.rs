#[derive(Debug)]
pub enum Error {
    CStringCreationError,
    DeviceOpenError,
    UnopenedDeviceError,
    DeviceNotReadyError,
    XclbinFileAllocError,
    XclbinLoadError,
    XclbinUUIDRetrievalError,
    KernelCreationError,
    KernelNotLoadedYetError,
    KernelArgRtrvError,
    BOCreationError,
    RunCreationError,
    RunNotCreatedYetError,
    SetRunArgError,
    BONotCreatedYet,
    BOWriteError,
    BOReadError,
    BOSyncError,

    // SIMPLE API ERRORS
    NoSuchKernelError,
    ArgumentNumberMismatchError,
    PassVecToScalarArgumentError,
    NoOpenRunsError,
    XclbinInvalidMagicString(String),
    XclbinByteReadingError(usize, usize), // Byte indices of start and end of given section
    XclbinNoBuildMetadataSection,
    XclbinJSONParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
