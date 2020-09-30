//!
//! Command-line IPP utility to print a document or get printer status
//!

use std::{fs, io, path::PathBuf, time::Duration};

use structopt::StructOpt;

use ipp::{
    prelude::*,
    proto::{model::DelimiterTag, IppAttribute, IppOperationBuilder, IppPayload},
};

fn new_client(uri: Uri, params: &IppParams) -> IppClient {
    let mut builder = IppClientBuilder::new(uri).ignore_tls_errors(params.ignore_tls_errors);
    if let Some(timeout) = params.timeout {
        builder = builder.timeout(Duration::from_secs(timeout));
    }
    builder.build()
}

fn get_payload(cmd: &IppPrintCmd) -> io::Result<IppPayload> {
    let payload = match cmd.file {
        Some(ref filename) => futures::io::AllowStdIo::new(fs::File::open(filename)?).into(),
        None => futures::io::AllowStdIo::new(io::stdin()).into(),
    };
    Ok(payload)
}

async fn do_print(params: &IppParams, cmd: IppPrintCmd) -> Result<(), IppError> {
    let client = new_client(cmd.uri.parse()?, params);

    if !cmd.no_check_state {
        client.check_ready().await?;
    }

    let payload = get_payload(&cmd).map_err(IppError::from)?;

    let mut builder = IppOperationBuilder::print_job(payload);
    if let Some(jobname) = cmd.job_name {
        builder = builder.job_title(&jobname);
    }
    if let Some(username) = cmd.user_name {
        builder = builder.user_name(&username);
    }

    for arg in cmd.options {
        let mut kv = arg.split('=');
        if let Some(k) = kv.next() {
            if let Some(v) = kv.next() {
                builder = builder.attribute(IppAttribute::new(k, v.parse()?));
            }
        }
    }

    let attrs = client.send(builder.build()).await?;
    if let Some(group) = attrs.groups_of(DelimiterTag::JobAttributes).get(0) {
        for v in group.attributes().values() {
            println!("{}: {}", v.name(), v.value());
        }
    }
    Ok(())
}

async fn do_status(params: &IppParams, cmd: IppStatusCmd) -> Result<(), IppError> {
    let client = new_client(cmd.uri.parse()?, &params);

    let operation = IppOperationBuilder::get_printer_attributes()
        .attributes(&cmd.attributes)
        .build();

    let attrs = client.send(operation).await?;

    let mut values = attrs
        .groups_of(DelimiterTag::PrinterAttributes)
        .iter()
        .flat_map(|group| group.attributes().values())
        .collect::<Vec<_>>();

    values.sort_by_key(|&a| a.name());

    for v in values {
        println!("{}: {}", v.name(), v.value());
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(about = "IPP print utility", name = "ipputil", rename_all = "kebab-case")]
struct IppParams {
    #[structopt(
        long = "ignore-tls-errors",
        short = "i",
        global = true,
        help = "Ignore TLS handshake errors"
    )]
    ignore_tls_errors: bool,

    #[structopt(
        long = "timeout",
        short = "t",
        global = true,
        help = "Request timeout in seconds, default = no timeout"
    )]
    timeout: Option<u64>,

    #[structopt(subcommand)]
    command: IppCommand,
}

#[derive(StructOpt)]
enum IppCommand {
    #[structopt(name = "print", about = "Print file to an IPP printer")]
    Print(IppPrintCmd),
    #[structopt(name = "status", about = "Get status of an IPP printer")]
    Status(IppStatusCmd),
}

#[derive(StructOpt, Clone)]
#[structopt(rename_all = "kebab-case")]
struct IppPrintCmd {
    #[structopt(help = "Printer URI")]
    uri: String,

    #[structopt(
        long = "no-check-state",
        short = "n",
        help = "Do not check printer state before printing"
    )]
    no_check_state: bool,

    #[structopt(
        long = "file",
        short = "f",
        help = "Input file name to print [default: standard input]"
    )]
    file: Option<PathBuf>,

    #[structopt(long = "job-name", short = "j", help = "Job name to send as job-name attribute")]
    job_name: Option<String>,

    #[structopt(
        long = "user-name",
        short = "u",
        help = "User name to send as requesting-user-name attribute"
    )]
    user_name: Option<String>,

    #[structopt(long = "option", short = "o", help = "Extra IPP job attributes in key=value format")]
    options: Vec<String>,
}

#[derive(StructOpt, Clone)]
#[structopt(rename_all = "kebab-case")]
struct IppStatusCmd {
    #[structopt(help = "Printer URI")]
    uri: String,

    #[structopt(long = "attribute", short = "a", help = "Attributes to query, default is to get all")]
    attributes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let params = IppParams::from_args();

    match params.command {
        IppCommand::Status(ref cmd) => do_status(&params, cmd.clone()).await?,
        IppCommand::Print(ref cmd) => do_print(&params, cmd.clone()).await?,
    }
    Ok(())
}
