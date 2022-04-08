use actix_web::{http::StatusCode, App, HttpRequest, ResponseError};
use paperclip::actix::OpenApiExt;
use paperclip::actix::{api_v2_errors, api_v2_operation};
use paperclip::actix::{put, Apiv2Schema};
use paperclip_restful::{JsonError, PutResponse};

#[test]
fn test_put_response() {
    #[derive(Apiv2Schema, serde::Serialize)]
    struct Person {
        first_name: String,
        last_name: String,
    }

    #[api_v2_errors(
        code = 400,
        description = "Bad request",
        code = 404,
        description = "Not found"
    )]
    #[derive(Debug, thiserror::Error)]
    enum PersonError {
        #[error("unexpected error")]
        Unexpected,
    }

    impl ResponseError for PersonError {
        fn status_code(&self) -> StatusCode {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    #[api_v2_operation]
    #[put("/")]
    async fn put_person(_: HttpRequest) -> Result<PutResponse<Person>, JsonError<PersonError>> {
        Err(JsonError::from(PersonError::Unexpected))
    }

    let (tx, rx) = std::sync::mpsc::channel();
    App::new()
        .wrap_api()
        .service(put_person)
        .with_raw_json_spec(move |app, json| {
            let json_spec = format!("{}", json);
            tx.send(json_spec).unwrap();
            app
        });

    let json_spec = rx.recv().unwrap();
    assert_eq!(
        r##"{"definitions":{"Person":{"properties":{"first_name":{"type":"string"},"last_name":{"type":"string"}},"required":["first_name","last_name"],"type":"object"}},"info":{"title":"","version":""},"paths":{"/":{"put":{"responses":{"200":{"description":"OK","schema":{"$ref":"#/definitions/Person"}},"201":{"description":"Created","schema":{"$ref":"#/definitions/Person"}},"400":{"description":"Bad request"},"404":{"description":"Not found"}}}}},"swagger":"2.0"}"##,
        json_spec
    );
}
