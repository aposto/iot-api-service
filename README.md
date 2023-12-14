
## 사용 프레임워크
* web: actix_web
* db: sqlx

## 구동
RUST_LOG=info ./main

## Architecture
* 클린아키텍쳐를 지향합니다.
* 헥사고날 아키텍처를 지행하여 도메인, 비즈니스로직, 어댑터, 인프라스트럭쳐를 분리합니다.
* 도메인에는 최대한 기술적 요소를 배제 합니다.
* 비즈니스로직이 중심 도메인에서 벗어나, 퍼시스턴스나, 어댑터, 인프라스트럭쳐에 존재 하지 않게 합니다.
* 순환참조를 배제하고 외부로직이 내부로 행하는 구조를 만듭니다.
* 내부에서는 의존성 역전으로 포트를 통한 인터페이싱을 합니다.
* REST API 명세를 지향합니다.


## Testing

### UnitTest
단위테스트는 소스코드내에 #[cfg(test)] 모듈로 하는 방향을 했습니다.
[save_temperature_service.rs](src/application/service/save_temperature_service.rs)

### IntegrateTest
tests/ 폴더에 구현해야 하나 시간 관계상 누락 했습니다.

## 완성부분
전체를 완성하지는 못했고, 등록(Command)과 온도 저장 부분만 구현했습니다.
아래 두부분이 그나마 제대로 구현 한 부분 같습니다. 

[save_temperature_service.rs](src/application/service/save_temperature_service.rs)

[SaveDevicePort](SaveDevicePort)


## Swagger
swagger 적용하려는 중이었습니다.

## 후기
업무강도로 인해 시간도 많치 않았지만, 오랜만에 Rust 프로그래밍을 하려니 좀 사소한 곳에서도 시간이 많이 걸렸습니다.
Dependency Injection 라이브러리가 마땅치 않은 상황에서 Hexagonal 베이스로 구현을 하다 보니 좀 여의치가 않았습니다.
어쩔수 없이 인스턴스가 상태 없이 trait을 구현하고 사용했습니다.
헥사고날 아키텍쳐가 스피링+코틀린 에서는 더할나위 없이 유용한데, 러스트에서 이런 아키텍쳐가 적절 한지 좀 고민해 보고 있습니다.
trait 을 선언 하면 해당 구현체를 자동으로 생성과 주입을 해줘야 하는데, 이런저런 방법과 트릭이 필요한 듯 합니다.
현업에서는 어떻게 처리하는지 궁금합니다.

Sqlx가 mysql pool을 필요로 하는데 이걸 ActicX의 app_data로 넣을수도 있지만, 이러면 
의존성 문제가 심각해져서, 퍼시스던스 계층에서만 접근 해야 하는데, 이걸 lazy하게 설정하는라 시간이 좀 걸렸습니다.
Diesel을 사용하려 했는데, 설치에 문제가 있어서 sqlx로 바꾸었습니다.

합불 여부를 떠나, 좋은 과제로 러스트 코딩을 할수 있어서 즐거 웠습니다.
감사합니다.