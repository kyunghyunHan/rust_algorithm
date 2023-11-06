use plotters::prelude::*;
use ndarray::prelude::*;
use rand::Rng;



pub fn main() {

    /* 
    벡터집합
    벡터들의 모음을 집합 이라합니다.
    벡터 집합은 S또는 V와 같이 대문자로 표시합니다.
    벡터집합은 유한 또는 무한한 수의 벡터를 가질수 있습니다.
    무한한 수의 벡터를 가진 벡터집합은 너무 추상적으로 느껴지기도 하지만 벡터 공간이 무한한 집합이고 이는 통계 모델을 데이터에 적합시킬떄 큰 영향을 미칩니다.
    벡터 집합이 비어있다면 V={}로 나타냅니다.
     */

    /*
    선형 가중 결합
    선형 가중결합은 어려 변수마다 가중치를 다르게 주어 정보를 혼합하는 방법입니다.
    이 기초 연산을 선형 혼합 또는 가중 결합 이라고도 합니다.
    선형 가중 결합은 단순하게 스칼라-벡터 곱셈을 한다음 합하는 것입니다.
    벡터 집합에서 각 벡터에 스칼라를 곱한 다음 이들을 더해 하나의 벡터를 만듭니다.

    v
     */
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

    
    let root = BitMapBackend::new("3d-env.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Empty 3D Figure", ("sans-serif", 40))
        .build_cartesian_3d(0.0..1.0, 0.0..1.0, 0.0..1.0)
        .unwrap();
    chart.configure_axes().draw().unwrap();
    chart.draw_series(SurfaceSeries::xoz(
        (-25..25).map(|v| v as f64 / 10.0), 
        (-25..25).map(|v| v as f64 / 10.0), 
        |x:f64,z:f64|(x * x + z * z).cos()).style(&BLUE.mix(0.2))
    ).unwrap();

}
