use {crate::utils::os_error::OsError, std::io};

#[test]
fn protocol_error() {
    let err = io::Error::from_raw_os_error(libc::EPROTO);
    {
        let kind = err.kind();
        let err: io::Error = kind.into();
        assert!(err.raw_os_error().is_none());
    }
    {
        let os: OsError = err.into();
        let err: io::Error = os.into();
        assert!(err.raw_os_error().is_some());
    }
}
