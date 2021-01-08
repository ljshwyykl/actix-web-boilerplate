use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiSuccess<T> {
    pub code: u16,
    pub msg: String,
    pub data: T,
}

// pub async fn handle_response<T: Serialize>(t: Result<T, ApiError>) -> impl Responder {
//     // let mut res_builder = actix_web::HttpResponse::Ok();

//     match t {
//         Ok(message) => HttpResponse::Ok().json(ApiSuccess::new(message)),
//         Err(err) => HttpResponse::Ok().json(err),
//     }
//     // HttpResponse::Ok().json(ApiSuccess::new(100, "String".to_string(), t))
//     // res_builder.json(
//     //     response.json::<serde_json::value::Value>().await.unwrap()
//     // )
// }

impl<T> ApiSuccess<T> {
    pub fn new(data: T) -> Self {
        ApiSuccess {
            code: 200,
            msg: "success".to_string(),
            data,
        }
        //  HttpResponse::Ok().json(ApiSuccess { code, msg, data })
    }

    // type Error = Error;
    // type Future = Ready<Result<HttpResponse, Error>>;

    // fn respond_to(self, _req: &HttpRequest) -> Self::Future {

    //     // let body = serde_json::to_string(&self).unwrap();

    //     // Create response and set content type
    //     ready(Ok(HttpResponse::Ok().json(ApiSuccess::new(
    //         100,
    //         "String".to_string(),
    //         "eee",
    //     ))))
    // }
}

/*
/*
pub struct ApiSuccess{
    // data: T
}




*/

pub struct ApiSuccess{}

impl<T, E> Responder for ApiSuccess<T, E>
where
    T: Responder,
    E: Into<Error>,
{
    type Error = ApiError;
    type Future = Ready<Result<HttpResponse, ApiError>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // let body = serde_json::to_string(&self).unwrap();

        // match self {
        //     Ok(val) => EitherFuture::Left(ResponseFuture::new(val.respond_to(req))),
        //     Err(e) => EitherFuture::Right(err(e.into())),
        // }
        // Create response and set content type
    //     ready(Ok(HttpResponse::Ok().json(ApiObj::new(
    //         100,
    //         "String".to_string(),
    //         "eee",
    //     ))))
    // }
}


// ApiSuccess<Vec<User>>

// impl<T> Vec for ApiSuccess<T>{

// }
// impl<T>  ApiSuccess<T> where T:Responder {
//     type Error = ApiError;
//     type Future = Ready<Result<HttpResponse, ApiError>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         // let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok().json(ApiSuccess::new(
//             100,
//             "String".to_string(),
//             "eee",
//         ))))
//     }
// }


*/
