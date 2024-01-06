
## Repository
https://github.com/aposto/iot-api-service


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
http://localhost:8080/swagger-ui/


## 선택 구현
데이터베이스에 Lock 메카니즘을 사용해서, 동시 장비 등록을 막는 방법을 구현해주세요.
 * serial_number 가 유니크 이빈다.
 * 분산 락을 사용하여 락을 획득후 저장 합니다.

Unit Test는 구현해주세요.
  
온도에 데이터가 10만건 이상인 경우 평균값을 내는 최대한 빨리 나오게 하는 방법에 대해서 구현 또는 기술해주세요.
 * 시계열 DB사용
 * 구간이 일단위라면, 시간단위 집계 중간 데이터 저장
 * Flink등 사용 스트리밍으로 중간 집계 별도 수집
 * OLAPDB사용


domain = { path = "../domain", version = "0.1.0" }
