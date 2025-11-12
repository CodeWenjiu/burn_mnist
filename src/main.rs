mod data;
mod model;
mod training;

use burn::backend::{
    Autodiff,
    wgpu::{Wgpu, WgpuDevice},
};

fn main() {
    let device = WgpuDevice::default();
    training::run::<Autodiff<Wgpu>>(device);
}
