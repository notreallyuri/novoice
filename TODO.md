# TODO

## Phase 1: The Contextual Guild (REST Engine)

This phase locks down your Role-Based Access Control (RBAC) so the client only
ever receives sanitized data they are explicitly allowed to see.

- [x] Implement get_guild Service: Fetch the base guild profile and all associated
categories and channels concurrently.
- [x] Calculate Base Permissions: Fetch the requesting user's GuildMember record
and compute their base permissions from their assigned roles.
- [x] Fetch Overrides: Query both category_overrides and channel_overrides for
the specific guild.
- [x] Apply the Bitwise Math: Route the base permissions through the
apply_override logic for each category and channel.
- [x] Filter the Payload: Strip out any channels where the resulting permission
mask lacks Permissions::VIEW_CHANNEL before returning the DTO.

## Phase 2: Access & Onboarding (REST Flow)

Before WebSockets can be useful, users need a way to populate their UI by
interacting with others and joining servers.

- [x] Create Invite Endpoint (POST /guilds/{id}/invites): Generate short-lived or
limited-use UUID invite codes.
- [] Join Guild Endpoint (POST /invites/{code}): Validate the invite, insert a
new guild_members record, and automatically assign the default @everyone role link.
- [] Leave Guild Endpoint (DELETE /guilds/{id}/members/@me): Allow users to exit
a server, handling cascading deletes cleanly.

## Phase 3: The Real-Time Gateway (WebSocket Auth)

This is where we safely bridge the stateless HTTP world with the stateful TCP
world, avoiding browser-based WebSocket header limitations.

- [] Ticket Generation (POST /auth/ticket): Create a secure, short-lived
(e.g., 30-second) token stored in Redis specifically for WebSocket upgrades.
- [] Axum Upgrade Route: Build the HTTP-to-WebSocket protocol upgrade handler in
apps/api/src/features/ws/listener.rs.
- [] The Identify Handshake: Require the client to send their short-lived ticket
as the first WebSocket frame.
- [] Session Management: Validate the ticket, map the active socket to the UserId
in the global DashMap, and disconnect unauthenticated peers.
- [] The Bootstrap Push: Upon successful identification, immediately fire the
UserServerEvents::InitialState payload (reusing the optimized get_me logic) down
the wire.

## Phase 4: Real-Time Routing (Redis Pub/Sub)

With authenticated sockets open, we route the background Redis events directly
into the user's active connection.

- [] Wire the Redis Listener: Update the background thread in ws/listener.rs to
look up active UserId connections in the DashMap and forward incoming channel:*,
guild:*, and user:* events.
- [] Presence Lifecycle: Emit Online status when a socket connects, and
automatically broadcast Offline when the socket drops or the heartbeat times out.
