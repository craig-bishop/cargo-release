use std::io::Error as IOError;
use std::string::FromUtf8Error;
use semver::SemVerError;

quick_error! {
    #[derive(Debug)]
    pub enum FatalError {
        IOError(err: IOError) {
            from()
            cause(err)
        }
        InvalidCargoFileFormat {
            display("Invalid cargo file format")
            description("Invalid cargo file format")
        }
        InvalidCargoConfigKeys {
            display("Invalid cargo-release config item found")
            description("Invalid cargo-release config item found")
        }
        SemVerError(err: SemVerError) {
            from()
            cause(err)
        }
        FromUtf8Error(err: FromUtf8Error) {
            from()
            cause(err)
        }
        InvalidReleaseLevel(level: String) {
            display("Unsupported release level {}", level)
            description("Unsupported release level, only major, minor and patch are supported")
        }
        UnsupportedPrereleaseVersionScheme {
            display("This version scheme is not supported by cargo-release.")
            description("This version scheme is not supported by cargo-release. Use format like `pre`, `dev` or `alpha.1` for prerelease symbol")
        }
    }
}
