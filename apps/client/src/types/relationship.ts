import type { UserId } from ".";
import type { UserPublic } from "./user";

export type RelationshipStatus =
  | "None"
  | "Friend"
  | "Blocked"
  | "PendingIncoming"
  | "PendingOutgoing";

export interface UserRelationship {
  id: UserId;
  since: string;
  status: RelationshipStatus;
  user: UserPublic;
}
