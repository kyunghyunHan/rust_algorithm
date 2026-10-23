// 정렬 정책의 공통 인터페이스
trait SortPolicy {
    fn sort(&self, numbers: &mut [i32]);
}

// 구체적인 정렬 알고리즘
struct BubbleSort;

impl SortPolicy for BubbleSort {
    fn sort(&self, numbers: &mut [i32]) {
        for end in (1..numbers.len()).rev() {
            let mut swapped = false;

            for i in 0..end {
                if numbers[i] > numbers[i + 1] {
                    numbers.swap(i, i + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }
}

// 요청할 정렬 종류
#[derive(Clone, Copy, PartialEq, Eq)]
enum SortType {
    Bubble,
}

// 정책 객체를 생성·소유하고 제공하는 팩토리
struct SortFactory {
    policies: Vec<(SortType, Box<dyn SortPolicy>)>,
}

impl SortFactory {
    fn new() -> Self {
        Self {
            policies: vec![
                (SortType::Bubble, Box::new(BubbleSort)),
            ],
        }
    }

    fn get_sort_policy(
        &self,
        sort_type: SortType,
    ) -> Option<&dyn SortPolicy> {
        self.policies
            .iter()
            .find(|(kind, _)| *kind == sort_type)
            .map(|(_, policy)| policy.as_ref())
    }
}

// 구체적인 정렬 알고리즘을 몰라도 정렬을 요청
struct User<'a> {
    factory: &'a SortFactory,
}

impl<'a> User<'a> {
    fn new(factory: &'a SortFactory) -> Self {
        Self { factory }
    }

    fn sort_numbers(
        &self,
        numbers: &mut [i32],
        sort_type: SortType,
    ) -> Result<(), &'static str> {
        let policy = self.factory
            .get_sort_policy(sort_type)
            .ok_or("지원하지 않는 정렬 종류입니다")?;

        // 실제 정책 객체의 sort()를 호출
        policy.sort(numbers);

        Ok(())
    }
}

fn main() {
    let factory = SortFactory::new();
    let user = User::new(&factory);

    let mut numbers = vec![5, 2, 9, 1, 3];

    println!("정렬 전: {:?}", numbers);

    match user.sort_numbers(&mut numbers, SortType::Bubble) {
        Ok(()) => println!("정렬 후: {:?}", numbers),
        Err(message) => eprintln!("오류: {message}"),
    }
}