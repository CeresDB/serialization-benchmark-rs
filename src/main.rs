#![allow(unused)]
mod util;
use clap::Parser;
use std::time::Duration;
use util::Serializable;
use util::{
    EnDecodeDuration, FlatBuffersObject, FuryObject, ProtobufObject, RawPerson, RawPet, TestContext,
};
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(default_value = "1000000", long, env)]
    batch_size: usize,

    #[arg(long, env, help = "use unsafe feature within flatbuffer")]
    enable_unsafe: bool,
}

fn main() {
    let args: Args = Args::parse();
    let mut table = Table::new();

    table.add_row(row![
        "name",
        "serialize time",
        "deserialize time",
        "cpu_utility"
    ]);
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    println!("Benchmark test, batch_size={}, result: \n", args.batch_size);
    let n: usize = args.batch_size;
    let enable_unsafe = args.enable_unsafe;
    let raw_person = RawPerson {
        name: "Mr' White".to_string(),
        age: 18,
        pets: vec![
            RawPet {
                name: "alice".to_string(),
            },
            RawPet {
                name: "bob".to_string(),
            },
        ],
    };

    // flatbuffer test
    {
        let mut cpu_trace =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        let mut flatbuffer_objects = Vec::new();
        (0..n).for_each(|_| flatbuffer_objects.push(FlatBuffersObject::new(&raw_person)));

        let mut context = TestContext {
            duration: (Duration::new(0, 0), Duration::new(0, 0)),
            enable_unsafe: args.enable_unsafe,
        };
        flatbuffer_objects
            .iter_mut()
            .for_each(|x| x.serialize_and_deserialize(&mut context));

        cpu_trace.refresh_cpu();
        table.add_row(Row::new(vec![
            Cell::new("flatbuffer"),
            Cell::new(format!("{:?}(s)", context.duration.0.as_secs_f32()).as_str()),
            Cell::new(format!("{:?}(s)", context.duration.1.as_secs_f32()).as_str()),
            Cell::new(
                format!(
                    "{:?}",
                    cpu_trace.cpus().iter().map(|x| x.cpu_usage()).sum::<f32>()
                        / cpu_trace.cpus().len() as f32
                )
                .as_str(),
            ),
        ]));
    }

    // fury test
    {
        let mut cpu_trace =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        let mut fury_objects = Vec::new();
        (0..n).for_each(|_| fury_objects.push(FuryObject::new(&raw_person)));
        let mut context = TestContext {
            duration: (Duration::new(0, 0), Duration::new(0, 0)),
            enable_unsafe: args.enable_unsafe,
        };
        fury_objects
            .iter_mut()
            .for_each(|x| x.serialize_and_deserialize(&mut context));
        cpu_trace.refresh_cpu();
        table.add_row(Row::new(vec![
            Cell::new("fury"),
            Cell::new(format!("{:?}(s)", context.duration.0.as_secs_f32()).as_str()),
            Cell::new(format!("{:?}(s)", context.duration.1.as_secs_f32()).as_str()),
            Cell::new(
                format!(
                    "{:?}",
                    cpu_trace.cpus().iter().map(|x| x.cpu_usage()).sum::<f32>()
                        / cpu_trace.cpus().len() as f32
                )
                .as_str(),
            ),
        ]));
    }

    // pb test
    {
        let mut cpu_trace =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        let mut protobuf_objects = Vec::new();
        (0..n).for_each(|_| protobuf_objects.push(ProtobufObject::new(&raw_person)));
        let mut context = TestContext {
            duration: (Duration::new(0, 0), Duration::new(0, 0)),
            enable_unsafe: args.enable_unsafe,
        };
        protobuf_objects
            .iter_mut()
            .for_each(|x| x.serialize_and_deserialize(&mut context));
        cpu_trace.refresh_cpu();
        table.add_row(Row::new(vec![
            Cell::new("protobuf"),
            Cell::new(format!("{:?}(s)", context.duration.0.as_secs_f32()).as_str()),
            Cell::new(format!("{:?}(s)", context.duration.1.as_secs_f32()).as_str()),
            Cell::new(
                format!(
                    "{:?}",
                    cpu_trace.cpus().iter().map(|x| x.cpu_usage()).sum::<f32>()
                        / cpu_trace.cpus().len() as f32
                )
                .as_str(),
            ),
        ]));
    }

    table.printstd();
}

