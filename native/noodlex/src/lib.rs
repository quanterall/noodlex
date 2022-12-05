use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::sync;
use std::{fs::File, io::BufReader};

use noodles_vcf as vcf;
use rustler::Env;
use rustler::Term;
use rustler::{Atom, Error as RustlerError, ResourceArc};

mod atoms {
    rustler::atoms! {
        ok,
        error,
        not_found,
        permission_denied,
        broken_pipe,
        already_exists,
        unknown,
        ancestral_allele,
        allele_count,
        total_read_depths,
        forward_strand_read_depths,
        reverse_strand_read_depths,
        allele_frequencies,
        total_allele_count,
        base_quality,
        cigar,
        is_in_db_snp,
        total_depth,
        is_in_hap_map2,
        is_in_hap_map3,
        mapping_quality,
        zero_mapping_quality_count,
        samples_with_data_count,
        strand_bias,
        is_somatic_mutation,
        is_validated,
        is_in_1000_genomes,
        is_imprecise,
        is_novel,
        end_position,
        sv_type,
        sv_lengths,
        position_confidence_intervals,
        end_confidence_intervals,
        microhomology_lengths,
        microhomology_sequences,
        breakpoint_ids,
        mobile_element_info,
        mobile_element_transduction_info,
        dbv_id,
        db_var_id,
        db_rip_id,
        mate_breakend_ids,
        partner_breakend_id,
        breakend_event_id,
        breakend_confidence_intervals,
        adjacent_read_depths,
        breakend_copy_number,
        adjacent_copy_number,
        copy_number_confidence_intervals,
        adjacent_copy_number_confidence_intervals,
        other,
        integer,
        float,
        flag,
        character,
        string,
        alternate_alleles,
        reference_and_alternate_alleles,
        genotypes,
        pass,
    }
}

struct VcfHandle {
    pub header: sync::Mutex<vcf::Header>,
    pub stream: sync::Mutex<vcf::Reader<BufReader<File>>>,
}

#[derive(rustler::NifStruct)]
#[module = "Noodlex.Vcf.Header.Info"]
struct VcfInfo<'a> {
    pub id: Atom,
    pub number: Term<'a>,
    pub type_: Atom,
    pub description: String,
}

#[derive(rustler::NifStruct)]
#[module = "Noodlex.Vcf.Header.Filter"]
struct VcfFilter {
    pub id: String,
    pub description: String,
}

#[derive(rustler::NifStruct)]
#[module = "Noodlex.Vcf.Header.FileFormat"]
struct FileFormat {
    major: u32,
    minor: u32,
}

#[derive(rustler::NifStruct)]
#[module = "Noodlex.Vcf.Header"]
struct VcfHeader<'a> {
    pub fileformat: FileFormat,
    pub infos: Term<'a>,
    pub filters: Term<'a>,
}

#[derive(rustler::NifStruct)]
#[module = "Noodlex.Vcf.Record"]
struct VcfRecord {
    pub chromosome: String,
    pub position: usize,
    pub ids: Vec<String>,
    pub reference_bases: String,
    pub alternate_bases: String,
    // pub quality_score: f32,
    pub filters: VcfRecordFilters,
    // pub info: Term<'a>,
    // pub format: Term<'a>,
    // pub genotypes: Term<'a>,
}

#[derive(rustler::NifTaggedEnum)]
enum VcfRecordFilters {
    None,
    Pass,
    Fail(Vec<String>),
}

fn load(env: rustler::Env, _info: rustler::Term) -> bool {
    rustler::resource!(VcfHandle, env);
    true
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
fn get_handle(path: String) -> Result<ResourceArc<VcfHandle>, RustlerError> {
    let file_result = File::open(path.clone());
    let file = handle_io_error!(file_result);
    let reader = BufReader::new(file);
    let mut vcf_reader = vcf::Reader::new(reader);
    let header_result = vcf_reader.read_header();
    let header = handle_io_error!(header_result);
    match header.parse::<vcf::header::Header>() {
        Ok(header) => {
            let mutex = sync::Mutex::new(vcf_reader);
            let header_mutex = sync::Mutex::new(header);
            let resource_arc = ResourceArc::new(VcfHandle {
                header: header_mutex,
                stream: mutex,
            });

            Ok(resource_arc)
        }
        Err(err) => {
            let error = format!("Error parsing header: {}", err);
            Err(RustlerError::Term(Box::new(error)))
        }
    }
}

#[rustler::nif]
fn get_header<'a>(
    env: Env<'a>,
    handle: ResourceArc<VcfHandle>,
) -> Result<VcfHeader<'a>, RustlerError> {
    let header = handle.header.lock().unwrap();
    let fileformat = FileFormat {
        major: header.file_format().major(),
        minor: header.file_format().minor(),
    };
    let mut infos_vector = Vec::new();
    for (key, value) in header.infos() {
        let id = Atom::from_str(env, &key.to_string()).unwrap();
        let number = match value.number() {
            vcf::header::Number::Count(_count) => atoms::unknown().to_term(env),
            vcf::header::Number::A => atoms::alternate_alleles().to_term(env),
            vcf::header::Number::R => atoms::reference_and_alternate_alleles().to_term(env),
            vcf::header::Number::G => atoms::genotypes().to_term(env),
            vcf::header::Number::Unknown => atoms::unknown().to_term(env),
        };
        let type_ = match value.ty() {
            vcf::header::info::ty::Type::Integer => atoms::integer(),
            vcf::header::info::ty::Type::Float => atoms::float(),
            vcf::header::info::ty::Type::Flag => atoms::flag(),
            vcf::header::info::ty::Type::Character => atoms::character(),
            vcf::header::info::ty::Type::String => atoms::string(),
        };
        let description = value.description().to_string();

        infos_vector.push((
            Atom::from_str(env, &key.to_string()).unwrap(),
            VcfInfo {
                id,
                number,
                type_,
                description,
            },
        ));
    }
    let mut filters_vector = Vec::new();
    for (key, value) in header.filters() {
        filters_vector.push((
            key.to_string(),
            VcfFilter {
                id: key.to_string(),
                description: value.description().to_string(),
            },
        ));
    }
    match (
        Term::map_from_pairs(env, &infos_vector),
        Term::map_from_pairs(env, &filters_vector),
    ) {
        (Ok(infos), Ok(filters)) => Ok(VcfHeader {
            fileformat,
            infos,
            filters,
        }),
        _ => Err(RustlerError::Term(Box::new(atoms::error()))),
    }
}

#[rustler::nif]
fn get_record<'a>(handle: ResourceArc<VcfHandle>) -> Result<VcfRecord, RustlerError> {
    let mut buf = String::new();
    let _record_result = handle.stream.lock().unwrap().read_record(&mut buf);
    let parsed_record = vcf::record::Record::try_from_str(&buf, &handle.header.lock().unwrap());
    match parsed_record {
        Ok(record) => {
            let chromosome = record.chromosome().to_string();
            let position = record.position().into();
            let ids = record.ids().iter().map(|id| id.to_string()).collect();
            let reference_bases = record.reference_bases().to_string();
            let alternate_bases = record.alternate_bases().to_string();
            // let quality_score = record
            //     .quality_score()
            //     .unwrap_or("0.0".parse().unwrap())
            //     .into();
            let filters = match record.filters() {
                Some(filters) => match filters {
                    vcf::record::filters::Filters::Pass => VcfRecordFilters::Pass,
                    vcf::record::filters::Filters::Fail(filters) => {
                        VcfRecordFilters::Fail(filters.iter().map(|f| f.to_string()).collect())
                    }
                },
                None => VcfRecordFilters::None,
            };

            return Ok(VcfRecord {
                chromosome,
                position,
                ids,
                reference_bases,
                alternate_bases,
                // quality_score,
                filters,
                // info,
                // format,
                // genotypes,
            });
        }
        Err(err) => Err(RustlerError::Term(Box::new(
            "hello".to_owned() + &err.to_string(),
        ))),
    }
}

rustler::init!(
    "Elixir.Noodlex.Vcf",
    [get_handle, get_header, get_record],
    load = load
);
