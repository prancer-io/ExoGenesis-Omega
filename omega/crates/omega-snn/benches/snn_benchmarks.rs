//! SNN Benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use omega_snn::{NetworkConfig, SNNEngine, SpikingNetwork, Layer, LayerType};
use omega_snn::neuron::NeuronType;
use std::time::Duration;

fn bench_network_step(c: &mut Criterion) {
    let mut network = SpikingNetwork::new(NetworkConfig::default());

    // Create a small network
    network.add_layer(
        "input".to_string(),
        "Input".to_string(),
        LayerType::Input,
        100,
        NeuronType::Excitatory,
    );

    network.add_layer(
        "hidden".to_string(),
        "Hidden".to_string(),
        LayerType::Hidden,
        100,
        NeuronType::Excitatory,
    );

    network.connect_layers(&"input".to_string(), &"hidden".to_string(), 0.1, 0.5);

    c.bench_function("network_step_200_neurons", |b| {
        b.iter(|| {
            network.step(black_box(Duration::from_millis(1)))
        })
    });
}

fn bench_snn_engine_run(c: &mut Criterion) {
    let config = NetworkConfig::default();
    let mut engine = SNNEngine::new(config);

    // Add some neurons
    for i in 0..50 {
        engine.network_mut().add_neuron(format!("n{}", i), NeuronType::Excitatory);
    }

    c.bench_function("snn_engine_100ms", |b| {
        b.iter(|| {
            engine.run(black_box(Duration::from_millis(100)))
        })
    });
}

criterion_group!(benches, bench_network_step, bench_snn_engine_run);
criterion_main!(benches);
