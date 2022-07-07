use std::sync::Arc;

use anyhow::Result;
use clap::{App, AppSettings};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use webrtc::api::APIBuilder;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::Error;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::sdp_type::RTCSdpType;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp_transceiver::rtp_codec::RTPCodecType;
use webrtc::rtp_transceiver::rtp_receiver::RTCRtpReceiver;
use webrtc::track::track_local::{TrackLocal, TrackLocalWriter};
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_remote::TrackRemote;

#[derive(Debug, Serialize, Deserialize)]
struct SDP {
    // #[serde(skip)]
    // #[serde(skip_serializing)]
    // #[serde(skip_deserializing)]
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "content")]
    // #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
    pub content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new("sfu")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandsNegateReqs);

    // let mut sdp_chan_rx = signal::http_sdp_server(port).await;

    // Wait for the offer
    // let desc_data = signal::decode(line.as_str())?;
    let desc_data = "";
    let offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;
    let mut m = MediaEngine::default();

    m.register_default_codecs()?;

    let mut registry = Registry::new();

    registry = register_default_interceptors(registry, &mut m)?;

    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    let config = RTCConfiguration {
        ice_servers: vec![],
        ice_transport_policy: Default::default(),
        bundle_policy: Default::default(),
        rtcp_mux_policy: Default::default(),
        sdp_semantics: Default::default(),
        ..Default::default()
    };

    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    // Allow us to receive 1 video track
    peer_connection
        .add_transceiver_from_kind(RTPCodecType::Video, &[])
        .await?;

    let (local_track_chan_tx, mut local_track_chan_rx) =
        tokio::sync::mpsc::channel::<Arc<TrackLocalStaticRTP>>(1);

    let local_track_chan_tx = Arc::new(local_track_chan_tx);
    // Set a handler for when a new remote track starts, this handler copies inbound RTP packets,
    // replaces the SSRC and sends them back
    let pc = Arc::downgrade(&peer_connection);

    peer_connection
        .on_track(Box::new(
            move |track: Option<Arc<TrackRemote>>, _receiver: Option<Arc<RTCRtpReceiver>>| {
                if let Some(track) = track {
                    // Send a PLI on an interval so that the publisher is pushing a keyframe every rtcpPLIInterval
                    // This is a temporary fix until we implement incoming RTCP events, then we would push a PLI only when a viewer requests it
                    let media_ssrc = track.ssrc();
                    let pc2 = pc.clone();
                    tokio::spawn(async move {
                        let mut result = Result::<usize>::Ok(0);
                        while result.is_ok() {
                            let timeout = tokio::time::sleep(Duration::from_secs(3));
                            tokio::pin!(timeout);

                            tokio::select! {
                                _ = timeout.as_mut() =>{
                                    if let Some(pc) = pc2.upgrade(){
                                        result = pc.write_rtcp(&[Box::new(PictureLossIndication{
                                            sender_ssrc: 0,
                                            media_ssrc,
                                        })]).await.map_err(Into::into);
                                    }else{
                                        break;
                                    }
                                }
                            }
                            ;
                        }
                    });

                    let local_track_chan_tx2 = Arc::clone(&local_track_chan_tx);
                    tokio::spawn(async move {
                        // Create Track that we send video back to browser on
                        let local_track = Arc::new(TrackLocalStaticRTP::new(
                            track.codec().await.capability,
                            "video".to_owned(),
                            "webrtc-rs".to_owned(),
                        ));
                        let _ = local_track_chan_tx2.send(Arc::clone(&local_track)).await;

                        // Read RTP packets being sent to webrtc-rs
                        while let Ok((rtp, _)) = track.read_rtp().await {
                            if let Err(err) = local_track.write_rtp(&rtp).await {
                                if Error::ErrClosedPipe != err {
                                    println!("output track write_rtp got error: {:?} and break", err);
                                    break;
                                } else {
                                    println!("output track write_rtp got error: {:?}", err);
                                }
                            }
                        }
                    });
                }

                Box::pin(async {})
            },
        ))
        .await;

    peer_connection.set_remote_description(offer).await?;

    let answer = peer_connection.create_answer(None).await?;

    peer_connection.set_local_description(answer).await?;

    if let Some(local_track) = local_track_chan_rx.recv().await {
        loop {
            // let desc_data = signal::decode(line.as_str())?;
            let recv_only_offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;

            let mut m = MediaEngine::default();

            m.register_default_codecs()?;

            let mut registry = Registry::new();

            registry = register_default_interceptors(registry, &mut m)?;

            let api = APIBuilder::new()
                .with_media_engine(m)
                .with_interceptor_registry(registry)
                .build();

            let config = RTCConfiguration {
                ice_servers: vec![RTCIceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                    ..Default::default()
                }],
                ..Default::default()
            };

            let peer_connection = Arc::new(api.new_peer_connection(config).await?);

            let rtp_sender = peer_connection
                .add_track(Arc::clone(&local_track) as Arc<dyn TrackLocal + Send + Sync>)
                .await?;

            // Read incoming RTCP packets
            // Before these packets are returned they are processed by interceptors. For things
            // like NACK this needs to be called.
            tokio::spawn(async move {
                let mut rtcp_buf = vec![0u8; 1500];
                while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
                Result::<()>::Ok(())
            });

            peer_connection
                .set_remote_description(recv_only_offer)
                .await?;

            let answer = peer_connection.create_answer(None).await?;

            peer_connection.set_local_description(answer).await?;

            //     let json_str = serde_json::to_string(&local_desc)?;
            //     let b64 = signal::encode(&json_str);
        }
    }

    Ok(())
}
