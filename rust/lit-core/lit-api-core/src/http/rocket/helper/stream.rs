use std::fmt;
use std::io::Cursor;

use async_compression::tokio::bufread::GzipEncoder;
use log::as_error;
use rocket::futures::stream::{Stream, StreamExt};
use rocket::http::hyper::header::CONTENT_DISPOSITION;
use rocket::http::{ContentType, Header};
use rocket::response::{self, stream::ReaderStream, Responder};
use rocket::{Request, Response};
use tokio::process::Child;

use crate::error::{unexpected_err, ApiError};

pub struct GzipStream<S>(pub (S, String));

impl<'r, S: Stream<Item = std::io::Result<bytes::BytesMut>>> Responder<'r, 'r> for GzipStream<S>
where
    S: Send + 'r,
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let (stream, filename) = self.0;

        let content_disposition = Header::new(
            CONTENT_DISPOSITION.as_str(),
            format!("attachment; filename=\"{}\"", filename),
        );

        Response::build()
            .header(ContentType::GZIP)
            .header(content_disposition)
            .streamed_body(ReaderStream::from(
                tokio_util::io::ReaderStream::new(GzipEncoder::new(
                    tokio_util::io::StreamReader::new(stream),
                ))
                .filter_map(|v| async {
                    match v {
                        Ok(v) => Some(Cursor::new(v)),
                        Err(e) => {
                            warn!(error = as_error!(e); "unexpected error during gzip stream");
                            None
                        }
                    }
                }),
            ))
            .ok()
    }
}

impl<S: Stream<Item = std::io::Result<bytes::BytesMut>>> fmt::Debug for GzipStream<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("GzipStream");

        builder.finish()
    }
}

pub struct ChildStream(pub (Child, ContentType, String));

impl<'r> Responder<'r, 'r> for ChildStream {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'r> {
        let (child, content_type, filename) = self.0;

        let content_disposition = Header::new(
            CONTENT_DISPOSITION.as_str(),
            format!("attachment; filename=\"{}\"", filename),
        );

        match child.stdout {
            Some(stdout) => Response::build()
                .header(content_type)
                .header(content_disposition)
                .streamed_body(stdout)
                .ok(),
            None => unexpected_err("ChildStream: expected child process to contain stdout", None)
                .handle()
                .respond_to(req),
        }
    }
}

impl fmt::Debug for ChildStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("ChildStream");
        builder.field("content_type", &self.0 .1);
        builder.finish()
    }
}
