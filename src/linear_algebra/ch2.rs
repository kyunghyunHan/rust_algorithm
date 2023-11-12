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

    모든 벡터 vⅈ의 차원은 같다고 가정합니다.λ는 0을 포함한 임의의 실수가 될수 있습니다.
    벡터의 뺄셈 연산을 위해 다시 작성할수 있지만 λⅈ를음수로 두면 뺴는것가 같기때문에 기본적으로 선형가중결합을 합으로 다루는 것이 더 편합니다.

    선형 가중결합은 다음 코드처럼 구현하기 쉽습니다.

    

     */

    let l1 = 1.0;
    let l2= 2.0;
    let l3 = -3.0;

    let v1 = arr1(&[4.0, 5.0, 1.0]);
    let v2 = arr1(&[-4.0, 0.0, -4.0]);
    let v3 = arr1(&[1.0, 3.0, 2.0]);

    println!("{}",l1*v1+l2*v2+l3*v3);

    /*
    각 벡터와 계수를 별도에 변수에 저장하는 방법은 번거롭고 더 어려운 문제를 풀떄 확장성이 떨어집니다.

    선형 가중결합은 여러 방면에서 응용됩니다.
    - 통계모델로부터 예측된 데이터는 최소제곱 알고리즘을 통해 계산되는 회귀변수(독립변수)와 계수(스칼라)의 선형 가중 결합으로 생성됩니다.
    - 주성분 분석과 같은 차원 축소 과정에서 각 성분의 성분의 분산을 최대화 하는 가중치와 데이터 채널의 선형가중 결합으로 도출됩니다.
    - 인공 신경망에서는 두가지 연산,즉 입력데이터의 선형가중결합과 비선형 변환이 있습니다. 가중치는 비용 함수를 최소화 하도록 학습됩니다.비용함수는 일반적으로 모델예측과 실제 목표변수사이의 차이입니다.

    선형 가중결합 개념은 부분공간과 행렬공간을 생성하는 메커니즘이며 선형독립성의 핵심입니다.
    
    
     */

    /*
    벡터 집합에서 적어도 하나의 벡터를 집합의 다른 벡터들의 선형가중결합으로 나타낼수 있을 떄 벡터집합을 선형 종속적 이라고합니다.
    반대로 집합에 있는 벡터들의 선형가중 결합으로 집합의 아무런 벡터도 나타낼수 없다면 해당 벡터 집합은 선형 독립적 입니다.

    벡터 집합 V는 선형독립적 입니다.집합의 한 벡터를 집합의 다른 벡터의 선형 배수로 나타낼수 없습니다.집합내의 벡터들을 v₁과 v₂라고 했을떄 v₁= λv₂인 스칼라 λ가 존재하지 않습니다.

    S집합은 선형종속적입니다. 이러한 결합은 무한히 존재하며 그중 두가지는 s₁= .5*s₂와 s₂= 2*s₁입니다.
    
    T는 복잡하지만 결과적으로 선형종속적 집합입니다.( 예로 세번쨰합이 네번쨰 벡터의 두배와 같기떄문)
    데이터과학에서는 선형독립성을 결정하는 방법은 벡터집합으로 행렬을 만들고 행렬의 계수를 계산한 다음 행의 수와 열의 수 중에서 더 작은 값과 비교하는 것입니다.

    이식의 의미는 선형종속적 이라면 집합의 벡터들의 선형가중 결합으로 영벡터를 만들수 있는 것입니다.
    식을 참으로 만드는 λ를 찾을수 있다면 벡터집합은 선형종속적 이며 선형적으로 결합해서 영벡터를 생성할수 있는 방법이 없다면 선형 독립적 입니다.

    식을 0과 같다고 설정하면 접체집합이 종속적 또는 독립적이라는 원리는 강조할수 잇습니다.그렇다면 어떠한 개별 벡터도 종속벡터라는 특권을 가지지 않게 됩니다.독립에 대해 평등한 벡터 집합이 됩니다.

    하지만 선형대수학에서 0과 같은 단순한 해는 좋은 답이 아니기 떄문에 대부분 적어도 하나의 λ ≠ 0이라는 제약 조건을 달게 됩니다.

    0벡터가 포함된 모든 벡터 집합은 당연히 선형 종속적인 집합입니다. 
    0벡터에 스칼라값을 곱하면 여전히 영벡터입니다.따라서 선형 종속정 정의를 항상 만족합니다.

   λ₁ ≠ 0이며 자명하지 않는 해가 존재 한다면 그 집합은 선형 종속성 정의에 부합합니다.


   벡터 집합의 동일한 벡터들을 사용하지만 가중치 숫자를 사용해서 무한히 선형 결합하는 방식으로 벡터 부분공간을 만듭니다.그리고 가능한 모든 선형가중결합을 구성하는 메커니즘을 벡터 집합의 생성 이라고합니다.
    
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
