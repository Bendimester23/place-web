// //Code stolen shamelessly from a random website
// use actix::{Actor, ActorContext, ActorStreamExt, Addr, AsyncContext, Context, ContextFutureSpawner, fut, Handler, Running, StreamHandler, WrapStream};
// use actix_web::get;
// use std::collections::{HashMap, HashSet};
// use std::time::Duration;
// use actix_rt::time::Instant;
// use actix_web_actors::ws;
// use actix_web_actors::ws::Message::Text;
//
// pub struct WebSocketSession  {
//     room: String,
//     lobby_addr: Addr<Lobby>,
//     hb: Instant,
//     id: Uuid,
// }
//
// impl Actor for WebSocketSession  {
//     type Context = actix_web_actors::ws::WebsocketContext<Self>;
//
//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.hb(ctx);
//
//         let addr = ctx.address();
//         self.lobby_addr
//             .send(Connect {
//                 addr: addr.recipient(),
//                 lobby_id: self.room.clone(),
//                 self_id: self.id,
//             })
//             .into_actor(self)
//             .then(|res, _, ctx| {
//                 match res {
//                     Ok(_res) => (),
//                     _ => ctx.stop(),
//                 }
//                 fut::ready(())
//             })
//             .wait(ctx);
//     }
//
//     fn stopping(&mut self, _: &mut Self::Context) -> Running {
//         self.lobby_addr.do_send(Disconnect { id: self.id, room_id: self.room.clone() });
//         Running::Stop
//     }
// }
//
// impl WebSocketSession  {
//     fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
//         ctx.run_interval(Duration::from_secs(1), |act, ctx| {
//             if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
//                 println!("Disconnecting failed heartbeat");
//                 act.lobby_addr.do_send(Disconnect { id: act.id, room_id: act.room.clone() });
//                 ctx.stop();
//                 return;
//             }
//             ctx.ping(b"hi");
//         });
//     }
//
//     fn new(room: &str, addr: Addr<Lobby>) -> Self {
//         Self {
//             room: String::from(room),
//             lobby_addr: addr,
//             hb: Instant::now(),
//             id: Uuid::default()
//         }
//     }
// }
//
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession  {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => {
//                 self.hb = Instant::now();
//                 ctx.pong(&msg);
//             }
//             Ok(ws::Message::Pong(_)) => {
//                 self.hb = Instant::now();
//             }
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             Ok(ws::Message::Close(reason)) => {
//                 ctx.close(reason);
//                 ctx.stop();
//             }
//             Ok(ws::Message::Continuation(_)) => {
//                 ctx.stop();
//             }
//             Ok(ws::Message::Nop) => (),
//             Ok(Text(s)) => self.lobby_addr.do_send(BroadcastMessage {
//                 id: self.id,
//                 msg: serde_json::Value::String(s.parse().unwrap()),
//                 room_id: self.room.clone()
//             }),
//
//             Err(e) => panic!("{:?}", e),
//         }
//     }
// }
//
// impl Handler<WsMessage> for WebSocketSession  {
//     type Result = ();
//
//     fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
//         ctx.text(msg.0);
//     }
// }
//
// use actix::prelude::{Message, Recipient};
// use actix_web::{Error, HttpRequest, HttpResponse};
// use actix_web::web::{Data, Payload};
// use uuid::Uuid;
// use serde_json::{to_string, Value};
// use serde::{Deserialize, Serialize};
//
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct WsMessage(pub String);
//
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Connect {
//     pub addr: Recipient<WsMessage>,
//     pub lobby_id: String,
//     pub self_id: Uuid,
// }
//
// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct Disconnect {
//     pub id: Uuid,
//     pub room_id: String,
// }
//
// #[derive(Message, Deserialize, Serialize, Clone)]
// #[rtype(result = "()")]
// pub struct BroadcastMessage {
//     pub id: Uuid,
//     pub msg: Value,
//     pub room_id: String
// }
//
// impl BroadcastMessage {
//     pub fn new(id: Uuid, data: Value, r_id: String) -> Self {
//         Self {
//             id,
//             msg :data,
//             room_id: r_id
//         }
//     }
// }
//
// type Socket = Recipient<WsMessage>;
//
// pub struct Lobby {
//     sessions: HashMap<Uuid, Socket>, //self id to self
//     rooms: HashMap<String, HashSet<Uuid>>,      //room id  to list of users id
// }
//
// impl Default for Lobby {
//     fn default() -> Lobby {
//         Lobby {
//             sessions: HashMap::new(),
//             rooms: HashMap::new(),
//         }
//     }
// }
//
// impl Lobby {
//     fn send_message(&self, message: &str, id_to: &Uuid) {
//         if let Some(socket_recipient) = self.sessions.get(id_to) {
//             let _ = socket_recipient
//                 .do_send(WsMessage(message.to_owned()));
//         } else {
//             println!("attempting to send message but couldn't find user id.");
//         }
//     }
// }
//
// impl Actor for Lobby {
//     type Context = Context<Self>;
// }
//
// impl Handler<Disconnect> for Lobby {
//     type Result = ();
//
//     fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
//         if self.sessions.remove(&msg.id).is_some() {
//             self.rooms
//                 .get(&msg.room_id)
//                 .unwrap()
//                 .iter()
//                 .filter(|conn_id| *conn_id.to_owned() != msg.id)
//                 .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
//             if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
//                 if lobby.len() > 1 {
//                     lobby.remove(&msg.id);
//                 } else {
//                     //only one in the lobby, remove it entirely
//                     self.rooms.remove(&msg.room_id);
//                 }
//             }
//         }
//     }
// }
//
// impl Handler<Connect> for Lobby {
//     type Result = ();
//
//     fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
//         self.rooms
//             .entry(msg.lobby_id.clone())
//             .or_insert_with(HashSet::new).insert(msg.self_id);
//
//         self
//             .rooms
//             .get(&msg.lobby_id)
//             .unwrap()
//             .iter()
//             .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
//             .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));
//
//         self.sessions.insert(
//             msg.self_id,
//             msg.addr,
//         );
//
//         self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
//     }
// }
//
// impl Handler<BroadcastMessage> for Lobby {
//     type Result = ();
//
//     fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Context<Self>) -> Self::Result {
//
//         if let Some(_socket_recipient) = self.sessions.get(&msg.id) {
//             self.rooms.get(&msg.room_id).unwrap().iter().for_each(|_client| self.send_message(&to_string(&msg).unwrap(), _client));
//         } else {
//             println!("attempting to send message but couldn't find admin id.");
//         }
//     }
// }
//
// #[get("/ws")]
// pub async fn start_connection(
//     req: HttpRequest,
//     stream: Payload,
//     srv: Data<Addr<Lobby>>,
// ) -> Result<HttpResponse, Error> {
//
//     println!("client");
//
//     let ws = WebSocketSession::new(
//         "place",
//         srv.get_ref().clone(),
//     );
//     let resp = ws::start(ws, &req, stream)?;
//     Ok(resp)
// }
