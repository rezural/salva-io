extern crate ply_rs;

use ply_rs::ply::{
    Addable, DefaultElement, ElementDef, Encoding, Ply, Property, PropertyDef, PropertyType,
    ScalarType,
};
use ply_rs::writer;
use salva3d::object::Fluid;

use std::io::Write;
use std::{fs::File, io::BufWriter};

pub enum WriteError {}
pub struct Writer {}

impl Writer {
    fn create_ply() -> Ply<DefaultElement> {
        let mut ply = Ply::<DefaultElement>::new();
        ply.header.encoding = Encoding::BinaryBigEndian;
        ply.header.comments.push("A beautiful comment!".to_string());
        let mut point_element = ElementDef::new("vertex".to_string());

        let p = PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("z".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);

        let p = PropertyDef::new("nx".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("ny".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);
        let p = PropertyDef::new("nz".to_string(), PropertyType::Scalar(ScalarType::Float));
        point_element.properties.add(p);

        ply.header.elements.add(point_element);

        ply
    }
}

impl super::Writer for Writer {
    fn write_particles(
        &self,
        to: &std::path::PathBuf,
        fluid: &Fluid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file: File = File::create(to).unwrap();
        let mut buf = Vec::<u8>::new();

        let mut ply = Self::create_ply();

        // info!("particles.len({})", fluid.posisions.len());
        let points = fluid
            .positions
            .iter()
            .enumerate()
            .map(|(idx, particle)| {
                let mut point = DefaultElement::new();

                point.insert("x".to_string(), Property::Float(particle.x));
                point.insert("y".to_string(), Property::Float(particle.y));
                point.insert("z".to_string(), Property::Float(particle.z));

                let v = &fluid.velocities[idx];
                point.insert("nx".to_string(), Property::Float(v.x));
                point.insert("ny".to_string(), Property::Float(v.y));
                point.insert("nz".to_string(), Property::Float(v.z));

                point
            })
            .collect();

        ply.payload.insert("vertex".to_string(), points);

        let mut buf_writer = BufWriter::new(file);

        let w = writer::Writer::new();
        let _written = w.write_ply(&mut buf, &mut ply)?;

        buf_writer.write_all(&buf)?;

        Ok(())
    }
}
