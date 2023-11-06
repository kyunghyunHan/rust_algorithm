use plotters::prelude::*;
use ndarray::prelude::*;
use rand::Rng;

pub fn main() {
    let v1 = arr1(&[3.0, 5.0, 1.0]);
    let v2 = arr1(&[0.0, 2.0, 2.0]);

    let mut rng = rand::thread_rng();
    let xlim = (-4.0, 4.0);

    let mut points = Vec::new();

    for _ in 0..100 {
        let scalar1 = rng.gen_range(xlim.0..xlim.1);
        let scalar2 = rng.gen_range(xlim.0..xlim.1);
        let x = v1[0] * scalar1 + v2[0] * scalar2;
        let y = v1[1] * scalar1 + v2[2] * scalar2;
        let z = v1[2] * scalar1 + v2[2] * scalar2;
        points.push((x, y, z));
    }

    // Create a drawing area
    let root = BitMapBackend::new("ch2.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(-4.0..4.0, -4.0..4.0)
    .unwrap();
   chart
        .configure_mesh()
        .x_desc("Length")
        .y_desc("Weight")
        .draw()
        .unwrap();
    chart
    .draw_series(
        points
            .iter()
            .map(|(x, y, _)| Circle::new((*x, *y), 5, Into::<ShapeStyle>::into(&BLACK).filled())
        )
    )
    .unwrap();

}
