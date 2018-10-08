use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

use actix::*;
use actix_web::ws::{handshake, WebsocketContext, WsStream};
use actix_web::{ws, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, Path};

use crate::db::AppState;
use crate::render::Failure;
use crate::utils::http_ok;

fn index_render() -> Result<String, Failure> {
    let toplinks = crate::menu::default_top_menu();
    let links = crate::menu::default_menu();
    let list = ructe_block_res!(crate::templates::email::build)?;
    // let meta = ructe_block_res!(crate::templates::email::build_meta)?;
    let meta = crate::modules::meta::Meta::new("Email Editor");
    ructe_page_res!(
        crate::templates::navigation::frame,
        meta,
        &toplinks,
        &links,
        &list
    )
}

pub fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    http_ok(index_render())
}

fn save_str_file(file: String, json: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    write!(file, "{}", json)?;
    Ok(())
}

/// Do websocket handshake and start websocket actor
pub fn ws_index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let params = Path::<(String, String)>::extract(req).unwrap();
    let mut resp = handshake(req)?;
    let stream = WsStream::new(req.payload()).max_size(8_388_608);

    let body = WebsocketContext::create(req.clone(), GrapeWs::new(params), stream);
    Ok(resp.body(body))

    // ws::start(req, GrapeWs::new())
}
/// websocket connection is long running connection, it easier
/// to handle with an actor
struct GrapeWs {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    ///
    params: Path<(String, String)>,
}

impl Actor for GrapeWs {
    type Context = ws::WebsocketContext<Self, AppState>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}
/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for GrapeWs {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        //ctx
        // process websocket messages
        debug!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let org = &self.params.0;
                let tid = self.params.1.parse::<i64>().unwrap();
                if "load".eq(&text) {
                    let res = load_file(format!("{}_{}.json", org, tid));
                    let restr = match res {
                        Ok(data) => data,
                        Err(err) => err.to_string(),
                    };
                    ctx.text(restr);
                } else {
                    let res = save_str_file(format!("{}_{}.json", org, tid), &text);
                    let restr = match res {
                        Ok(_) => String::new(),
                        Err(err) => err.to_string(),
                    };
                    ctx.text(restr);
                }
                // ctx.text(text);
            }
            ws::Message::Binary(_bin) => {
                // ctx.binary(bin);
            }
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

impl GrapeWs {
    fn new(par: Path<(String, String)>) -> Self {
        Self {
            hb: Instant::now(),
            params: par,
        }
    }

    /// helper method that sends ping to client every second.
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                debug!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping("");
        });
    }
}

pub fn load_file(file: String) -> Result<String, std::io::Error> {
    let file = File::open(file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
