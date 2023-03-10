use actix_web::{get, web, HttpResponse, Responder};

use bytes::Bytes;
use futures::stream::{self, Stream, StreamExt};

type BoxedError = Box<dyn std::error::Error + 'static>;

pub struct IndexPage {
    before: String,
    after: String,
}

impl IndexPage {
    async fn render(&self) -> impl Stream<Item = Result<Bytes, BoxedError>> + Send {
        let renderer = yew::ServerRenderer::<crate::ui::App>::new();
        let before = self.before.clone();
        let after = self.after.clone();

        stream::once(async move { before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { after }))
            .map(|m| Result::<_, BoxedError>::Ok(m.into()))
    }

    pub fn load(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let index_html = std::fs::read_to_string(path.as_ref())?;
        let (index_before, index_after) = index_html.split_once("<body>").unwrap().to_owned();
        let (mut index_before, index_after) = (index_before.to_owned(), index_after.to_owned());
        index_before.push_str("<body>");
        log::trace!("<body> before: {}", index_before);
        log::trace!("<body> after: {}", index_after);
        Ok(Self {
            before: index_before.to_owned(),
            after: index_after.to_owned(),
        })
    }
}

#[get("/")]
pub async fn index(page: web::Data<IndexPage>) -> impl Responder {
    super::get_hits::INDEX_HITS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    HttpResponse::Ok().streaming(page.render().await)
}
