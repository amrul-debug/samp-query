//! Benchmarks for the SAMP Query library.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use samp_query::{Client, QueryType};
use samp_query::packet::Packet;
use std::net::SocketAddr;
use tokio::runtime::Runtime;

fn bench_client_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();

    c.bench_function("client_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let client = Client::connect(black_box(addr)).await.unwrap();
                black_box(client)
            })
        })
    });
}

fn bench_packet_creation(c: &mut Criterion) {
    let addr: SocketAddr = "127.0.0.1:7777".parse().unwrap();

    c.bench_function("packet_creation", |b| {
        b.iter(|| {
            let packet = Packet::create_query(
                black_box(addr),
                black_box(QueryType::Information),
            )
            .unwrap();
            black_box(packet)
        })
    });
}

fn bench_response_parsing(c: &mut Criterion) {
    let data = [
        // SAMP signature
        b'S', b'A', b'M', b'P',
        // Server IP
        127, 0, 0, 1,
        // Server port
        0x41, 0x1E,
        // Query type
        b'i',
        // Password
        0,
        // Players
        0x05, 0x00,
        // Max players
        0x32, 0x00,
        // Hostname length
        0x0A, 0x00, 0x00, 0x00,
        // Hostname
        b'T', b'e', b's', b't', b' ', b'S', b'e', b'r', b'v', b'e', b'r',
        // Gamemode length
        0x08, 0x00, 0x00, 0x00,
        // Gamemode
        b'F', b'r', b'e', b'e', b'r', b'o', b'a', b'm',
        // Language length
        0x07, 0x00, 0x00, 0x00,
        // Language
        b'E', b'n', b'g', b'l', b'i', b's', b'h',
    ];

    let packet = Packet::from_bytes(&data);

    c.bench_function("response_parsing", |b| {
        b.iter(|| {
            let response = packet.parse_response(black_box(QueryType::Information)).unwrap();
            black_box(response)
        })
    });
}

criterion_group!(
    benches,
    bench_client_creation,
    bench_packet_creation,
    bench_response_parsing
);
criterion_main!(benches);
