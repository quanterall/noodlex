use std::{fs::File, io::BufReader};
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

use noodles_vcf as vcf;
use rustler::{Atom, Error as RustlerError};

mod atoms {
    rustler::atoms! {
        ok,
        error,
        not_found,
        permission_denied,
        broken_pipe,
        already_exists,
        unknown,
    }
}

fn io_error_to_term(err: &IoError) -> Atom {
    match err.kind() {
        IoErrorKind::NotFound => atoms::not_found(),
        IoErrorKind::PermissionDenied => atoms::permission_denied(),
        IoErrorKind::BrokenPipe => atoms::broken_pipe(),
        IoErrorKind::AlreadyExists => atoms::already_exists(),
        _ => atoms::unknown(),
    }
}

macro_rules! handle_io_error {
    ($e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(ref error) => return Err(RustlerError::Term(Box::new(io_error_to_term(error)))),
        }
    };
}

#[rustler::nif]
fn get_header(path: String) -> Result<String, RustlerError> {
    let file_result = File::open(path);
    let file = handle_io_error!(file_result);
    let mut reader = BufReader::new(file);
    let header_result = vcf::Reader::new(&mut reader).read_header();
    let header = handle_io_error!(header_result);
    Ok(header)
}

rustler::init!("Elixir.Noodlex", [get_header]);
