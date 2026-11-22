// Module: engine/crates/signal-graph/src/ephemeral_presence.rs — 60 Hz Collab Presence & Follow-Me Viewport Lock (FEAT-094)
// Invariants:Volatile RAM-only presence state; enforces 60 Hz frame pacing & presenter Follow-Me camera sync.
// Spec: architecture/04-kubernetes-runtime-and-collaboration.md, architecture/17-extended-feature-specifications-ui-and-rust.md
use signal_geom::Point2Nm;
use signal_proto::v1::{BroadcastFollowPresenceRequest, BroadcastFollowPresenceResponse};
use std::collections::{BTreeMap, BTreeSet};

pub const PRESENCE_MIN_INTERVAL_US: u64 = 16_000; // ~60 Hz cap

#[derive(Debug, Clone, PartialEq)]
pub struct CollaboratorPresenceState {
    pub session_id: String,
    pub camera_center_nm: Point2Nm,
    pub zoom_scale: f64,
    pub laser_pointer_nm: Option<Point2Nm>,
    pub last_broadcast_us: u64,
}

/// Manages 60 Hz ephemeral camera/cursor/laser broadcasts and Follow-Me presenter lock (`FEAT-094`).
#[derive(Debug, Default)]
pub struct EphemeralPresenceHub {
    collaborators: BTreeMap<String, CollaboratorPresenceState>,
    followers_by_presenter: BTreeMap<String, BTreeSet<String>>,
    active_locked_presenter: Option<String>,
}

impl EphemeralPresenceHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe_follower(&mut self, follower_session_id: &str, presenter_session_id: &str) {
        self.followers_by_presenter
            .entry(presenter_session_id.to_string())
            .or_default()
            .insert(follower_session_id.to_string());
    }

    pub fn unsubscribe_session(&mut self, session_id: &str) {
        self.collaborators.remove(session_id);
        self.followers_by_presenter.remove(session_id);
        for set in self.followers_by_presenter.values_mut() {
            set.remove(session_id);
        }
        if self.active_locked_presenter.as_deref() == Some(session_id) {
            self.active_locked_presenter = None;
        }
    }

    pub fn active_locked_presenter(&self) -> Option<&str> {
        self.active_locked_presenter.as_deref()
    }

    pub fn session_state(&self, session_id: &str) -> Option<&CollaboratorPresenceState> {
        self.collaborators.get(session_id)
    }

    /// Updates presenter camera/laser state at `<= 60 Hz` and propagates viewport to locked followers (`FEAT-094`).
    pub fn broadcast_presence(
        &mut self,
        req: &BroadcastFollowPresenceRequest,
        now_us: u64,
    ) -> BroadcastFollowPresenceResponse {
        let cam = req
            .camera_center_nm
            .as_ref()
            .map_or(Point2Nm::ORIGIN, |p| Point2Nm::new(p.x_nm, p.y_nm));
        let laser = req
            .laser_pointer_nm
            .as_ref()
            .map(|p| Point2Nm::new(p.x_nm, p.y_nm));
        let zoom = req.zoom_scale.clamp(0.01, 1000.0);

        if req.follow_me_locked {
            self.active_locked_presenter = Some(req.presenter_session_id.clone());
        } else if self.active_locked_presenter.as_deref() == Some(&req.presenter_session_id) {
            self.active_locked_presenter = None;
        }

        let throttled = self
            .collaborators
            .get(&req.presenter_session_id)
            .is_some_and(|prev| {
                now_us.saturating_sub(prev.last_broadcast_us) < PRESENCE_MIN_INTERVAL_US
            });

        if !throttled {
            self.collaborators.insert(
                req.presenter_session_id.clone(),
                CollaboratorPresenceState {
                    session_id: req.presenter_session_id.clone(),
                    camera_center_nm: cam,
                    zoom_scale: zoom,
                    laser_pointer_nm: laser,
                    last_broadcast_us: now_us,
                },
            );

            if let Some(followers) = self
                .followers_by_presenter
                .get(&req.presenter_session_id)
                .cloned()
            {
                for fid in followers {
                    self.collaborators.insert(
                        fid.clone(),
                        CollaboratorPresenceState {
                            session_id: fid,
                            camera_center_nm: cam,
                            zoom_scale: zoom,
                            laser_pointer_nm: None,
                            last_broadcast_us: now_us,
                        },
                    );
                }
            }
        }

        let follower_count = self
            .followers_by_presenter
            .get(&req.presenter_session_id)
            .map_or(0, |s| s.len() as u32);
        BroadcastFollowPresenceResponse { follower_count }
    }
}
