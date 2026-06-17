# TODO

## Phase 1: The Contextual Guild (REST Engine)

*This phase locks down your Role-Based Access Control (RBAC) so the client only
ever receives sanitized data they are explicitly allowed to see.*

- [x] Implement `get_guild` Service
- [x] Calculate Base Permissions
- [x] Fetch Overrides
- [x] Apply the Bitwise Math
- [x] Filter the Payload

## Phase 2: Access & Onboarding (REST Flow)

*Before WebSockets can be useful, users need a way to populate their UI by
interacting with others and joining servers.*

- [x] Create Invite Endpoint (`POST /guilds/{id}/invite`)
- [x] Join Guild Endpoint (`POST /users/@me/g/join`)
- [x] Leave Guild Endpoint (`DELETE /guilds/{id}/members/@me`)

## Phase 3: The Real-Time Gateway (WebSocket Auth)

*Bridging the stateless HTTP world with the stateful TCP world.*

- [x] Ticket Generation (`POST /auth/ticket`)
- [x] Axum Upgrade Route (`ws/handler.rs`)
- [x] The Identify Handshake
- [x] Session Management via `DashMap`
- [x] The Bootstrap Push (`Ready` & `PresenceUpdate` events)

## Phase 4: Real-Time Routing (Redis Pub/Sub)

*Routing background events directly into the user's active connection.*

- [x] Wire the Redis Listener (`ws/listener.rs`)
- [x] Presence Lifecycle (Online/Offline broadcasts)
- [x] 30-Second Heartbeat Engine

## Phase 5: The SFU Media Engine (WebRTC)

*The custom Selective Forwarding Unit for Voice Channels.*

- [x] Media Engine Initialization & Opus Codec Registration
- [x] SDP Offer/Answer Negotiation Switchboard
- [x] Interactive Connectivity Establishment (ICE) Traversal
- [x] Lock-Free `VoiceRoom` isolation using `DashMap`
- [x] RTP Packet Router (Microphone to Virtual Speakers)
- [x] Zombie Connection & Memory Leak Cleanup

## Phase 6: Distributed Messaging (ScyllaDB)

*High-throughput, highly available message storage.*

- [x] YYYYMM Time-Bucket Partitioning strategy
- [x] `thread_messages` Materialized Views (Eliminated `ALLOW FILTERING`)
- [x] Cross-bucket Pagination Engine
- [x] Soft Deletion & Message Editing

---

## Phase 7: Direct Messaging (DMs)

*Expanding the channel architecture to support private 1-on-1 conversations.*

- [x] Create DM Channel initialization endpoints (`POST /users/@me/channels`).
- [x] Implement DM list fetching and caching.
- [x] Route the existing WebRTC `JoinVoice` logic to seamlessly handle DM Call states.

## Phase 8: Server Administration (Audit & Webhooks)

*Advanced guild management capabilities.*

- [ ] Create an `audit_logs` table in PostgreSQL to track administrative actions.
- [ ] Implement middleware/hooks on destructive endpoints
(Kick, Ban, Delete Channel) to write to the audit log.
- [ ] Build the Webhook execution engine for external integrations.

## Phase 9: The Native Client (Front End)

*Building the cross-platform desktop client to consume the Axum/Scylla backend.*

- [ ] **Core Setup & IPC:**
  - [ ] Initialize Tauri application shell.
  - [ ] Configure strict Content Security Policies (CSP) for WebRTC and WebSocket
  connections.
- [ ] **Design System & UI Architecture:**
  - [ ] Establish global CSS variables/themes (System/Dark/Light).
  - [ ] Implement a strict, squared-off component library
  (enforcing `border-radius: 0` across all containers, modals, and buttons).
  - [ ] Build the layout shell
  (Server Sidebar, Channel List, Main Content Area, Member List).
- [ ] **State Management & Networking:**
  - [ ] Build the WebSocket hook/store to handle the `Identify` handshake using
  the short-lived ticket.
  - [ ] Implement presence tracking (Online/Offline) and the 30-second
  heartbeat loop.
  - [ ] Create the WebRTC state machine to manage `RTCPeerConnection`, catch
  incoming audio tracks, and handle SDP/ICE signaling via the WS switchboard.
- [ ] **Core Views:**
  - [ ] Authentication (Login/Register/Recovery).
  - [ ] Guild View (Real-time chat scrolling with ScyllaDB pagination).
  - [ ] Direct Message Hub.
  - [ ] Active Voice Room overlay (showing speaking indicators based on audio levels).

## Phase 10: Asset Storage & Media Pipeline

*Handling user uploads, avatars, and message attachments via Cloudflare R2.*

- [ ] Implement Pre-Signed URL generation for direct-to-S3 client uploads.
- [ ] Create `features/storage` endpoints to finalize and link uploaded assets to
User Profiles.
- [ ] Add attachment metadata arrays to the ScyllaDB `messages` table schema.
