
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


[save_temperature_service.rs](src/application/service/save_temperature_service.rs)

[SaveDevicePort](SaveDevicePort)


## Swagger
http://localhost:8080/swagger-ui/





domain = { path = "../domain", version = "0.1.0" }
