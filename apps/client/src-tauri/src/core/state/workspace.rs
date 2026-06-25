use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceData {
    pub id: String,
    #[serde(rename = "type")]
    pub space_type: String,
    pub target_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "nodeType", rename_all = "camelCase")]
pub enum WorkspaceNode {
    Group {
        id: String,
        direction: String,
        children: Vec<WorkspaceNode>,
    },
    Panel {
        data: SpaceData,
    },
}

impl WorkspaceNode {
    pub fn remove_node(&mut self, id_to_remove: &str) -> bool {
        match self {
            WorkspaceNode::Group { children, .. } => {
                let mut found = false;
                let initial_len = children.len();

                children.retain(|child| {
                    if let WorkspaceNode::Panel { data } = child {
                        data.id != id_to_remove
                    } else {
                        true
                    }
                });

                if children.len() < initial_len {
                    found = true;
                } else {
                    for child in children.iter_mut() {
                        if child.remove_node(id_to_remove) {
                            found = true;
                            break;
                        }
                    }
                }

                if found {
                    children.retain(|child| {
                        if let WorkspaceNode::Group {
                            children: inner, ..
                        } = child
                        {
                            !inner.is_empty()
                        } else {
                            true
                        }
                    });
                }
                found
            }
            WorkspaceNode::Panel { .. } => false,
        }
    }

    pub fn replace_node(&mut self, target_id: &str, new_data: SpaceData) -> bool {
        match self {
            WorkspaceNode::Group { children, .. } => {
                for child in children.iter_mut() {
                    if let WorkspaceNode::Panel { data } = child {
                        if data.id == target_id {
                            *data = new_data;
                            return true;
                        }
                    }
                    if child.replace_node(target_id, new_data.clone()) {
                        return true;
                    }
                }
                false
            }
            WorkspaceNode::Panel { .. } => false,
        }
    }

    pub fn split_node(
        &mut self,
        target_id: &str,
        new_panel: WorkspaceNode,
        new_group_id: String,
        split_direction: String,
    ) -> bool {
        match self {
            WorkspaceNode::Group { children, .. } => {
                for child in children.iter_mut() {
                    if let WorkspaceNode::Panel { data } = child {
                        if data.id == target_id {
                            let original_panel = child.clone();

                            *child = WorkspaceNode::Group {
                                id: new_group_id,
                                direction: split_direction,
                                children: vec![original_panel, new_panel],
                            };
                            return true;
                        }
                    }

                    if child.split_node(
                        target_id,
                        new_panel.clone(),
                        new_group_id.clone(),
                        split_direction.clone(),
                    ) {
                        return true;
                    }
                }
                false
            }
            WorkspaceNode::Panel { .. } => false,
        }
    }

    pub fn add_to_root(&mut self, new_node: WorkspaceNode) {
        if let WorkspaceNode::Group { children, .. } = self {
            children.push(new_node);
        }
    }

    pub fn set_root_direction(&mut self, new_direction: String) {
        if let WorkspaceNode::Group { direction, .. } = self {
            *direction = new_direction;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceData {
    pub root: WorkspaceNode,
}

pub struct WorkspaceState {
    pub inner: Arc<RwLock<WorkspaceData>>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(WorkspaceData {
                root: WorkspaceNode::Group {
                    id: "root-group".to_string(),
                    direction: "horizontal".to_string(),
                    children: vec![
                        WorkspaceNode::Panel {
                            data: SpaceData {
                                id: "space-1".to_string(),
                                space_type: "channel".to_string(),
                                target_id: "c-123".to_string(),
                                title: "general".to_string(),
                            },
                        },
                        WorkspaceNode::Panel {
                            data: SpaceData {
                                id: "space-2".to_string(),
                                space_type: "channel".to_string(),
                                target_id: "c-456".to_string(),
                                title: "dev-updates".to_string(),
                            },
                        },
                    ],
                },
            })),
        }
    }
}
