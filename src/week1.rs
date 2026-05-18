pub fn run_practice() {
    println!("--- 1주차 Rust 기초 스터디 시작 ---");

    // 불변 변수 테스트
    let x = 10;
    println!("기본 변수 x의 값: {}", x);

    // 가변 변수 테스트 (mut)
    let mut y = 10;
    println!("가변 변수 y의 초기값: {}", y);
    y = 20;
    println!("변경된 y의 값: {}", y);
}

// 2. 구조체 정의 (외부 의존성인 Serialize/Deserialize는 제외하고 기본 Debug만 주입)
#[derive(Debug)]
struct User {
    username: String,
    email: String,
}

// 3. 미사용 코드 경고 끄기 테스트
#[allow(dead_code)]
fn future_work() {
    // 아직 구현 안 함
}

// ==================== 테스트 영역 ====================

#[test]
fn test_week1() {
    println!("이것은 테스트 환경에서 실행되는 로그입니다!");

    let mut a = 5;
    a = a + 5;
    assert_eq!(a, 10); // a가 10이 맞는지 검증
}

#[test]
#[should_panic] // 이 함수 안에서 에러(Panic)가 나야 이 테스트는 '패스'가 됩니다.
fn test_divide_by_zero() {
    // 💡 윈도우/리눅스 환경에 따라 컴파일러가 0으로 나누는 상수를 미리 차단할 수 있습니다.
    // 컴파일러의 눈을 속여서 런타임에 0이 되도록 변수로 나누기를 실행합니다.
    let zero = 0;
    let _result = 10 / zero;
}
