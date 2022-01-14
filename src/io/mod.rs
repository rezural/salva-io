pub mod ply;

use std::path::PathBuf;

use salva3d::object::Fluid;

#[allow(unused)]
pub trait Writer {
    fn write_particles(
        &self,
        to: &PathBuf,
        fluid: &Fluid,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
