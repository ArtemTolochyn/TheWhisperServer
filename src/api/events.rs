use actix_web::{post, rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;


#[post("/ws")]
async fn get_events(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let auth_token: Option<String> = None;

    let app_state = match req.app_data::<web::Data<crate::AppState>>() {
        Some(state) => state,
        None => return Err(actix_web::error::ErrorInternalServerError("Internal server error")),
    };

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));


    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    Ok(res)
}