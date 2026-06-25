import { Hash, Loader2, PanelBottom, PanelRight, X } from "lucide-react";
import { Fragment, useEffect, useRef } from "react";
import { Group, Panel } from "react-resizable-panels";
import { cn } from "@/lib/utils";
import {
  type SpaceData,
  useWorkspaceStore,
  type WorkspaceNode,
} from "@/store/workspace-store";
import { Textarea } from "../ui/textarea";
import { Handle } from "./handle";

function countTotalPanels(node: WorkspaceNode): number {
  if (node.nodeType === "panel") {
    return 1;
  }
  if (node.nodeType === "group") {
    return node.children.reduce(
      (acc, child) => acc + countTotalPanels(child),
      0
    );
  }
  return 0;
}

function Space({
  data,
  totalSpaces,
}: {
  data: SpaceData;
  totalSpaces: number;
}) {
  const closePanel = useWorkspaceStore((state) => state.closePanel);
  const splitPanel = useWorkspaceStore((state) => state.splitPanel);

  const activePanelId = useWorkspaceStore((state) => state.activePanelId);
  const setActivePanel = useWorkspaceStore((state) => state.setActivePanel);

  const isActive = activePanelId === data.id && totalSpaces > 1;
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const isTrulyActive = activePanelId === data.id;

  useEffect(() => {
    if (isTrulyActive && textareaRef.current) {
      textareaRef.current.focus();
    }
  }, [isTrulyActive]);

  const handleSplit = (direction: "horizontal" | "vertical") => {
    splitPanel(data.id, direction, {
      targetId: data.targetId,
      title: data.title,
      type: data.type,
    });
  };

  return (
    <div
      className={cn(
        "flex h-full w-full flex-col border bg-[oklch(0.1344_0_0)] transition-colors",
        isActive
          ? "border-primary"
          : "border-transparent border-r-border border-b-border"
      )}
      data-panel-id={data.id}
      onClick={() => setActivePanel(data.id)}
    >
      <div className="group flex h-12 items-center justify-between border-border border-b bg-muted/10 px-4">
        <div className="flex items-center gap-2 font-bold">
          {data.type === "channel" && (
            <Hash className="text-muted-foreground" size={18} />
          )}
          <span>{data.title}</span>
        </div>

        <div className="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
          <button
            className="rounded p-1 text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground"
            onClick={() => handleSplit("horizontal")}
            title="Split Right"
          >
            <PanelRight size={16} />
          </button>
          <button
            className="mr-2 rounded p-1 text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground"
            onClick={() => handleSplit("vertical")}
            title="Split Down"
          >
            <PanelBottom size={16} />
          </button>
          <button
            className="rounded border border-transparent p-1 transition-all hover:border-destructive hover:bg-destructive hover:text-white"
            onClick={() => closePanel(data.id)}
            title="Close Panel"
          >
            <X size={16} />
          </button>
        </div>
      </div>

      <div className="flex flex-1 flex-col justify-end overflow-y-auto p-4">
        <div className="mb-4 border-border border-b-2 pb-4 text-muted-foreground text-sm">
          Welcome to the start of #{data.title}.
        </div>
      </div>

      <div className="border-border border-t bg-background p-4">
        <Textarea
          className="w-full resize-none border border-border bg-[oklch(0.1344_0_0)] p-3 font-medium outline-none transition-colors placeholder:text-muted-foreground/50 focus:border-primary"
          placeholder={`Message #${data.title}`}
          ref={textareaRef}
        />
      </div>
    </div>
  );
}

function WorkspaceNodeRenderer({
  node,
  totalSpaces,
}: {
  node: WorkspaceNode;
  totalSpaces: number;
}) {
  if (node.nodeType === "panel") {
    return <Space data={node.data} totalSpaces={totalSpaces} />;
  }

  if (node.nodeType === "group") {
    if (node.children.length === 0) {
      return (
        <div className="flex h-full w-full items-center justify-center bg-background font-bold text-muted-foreground text-sm uppercase tracking-widest">
          Empty Group
        </div>
      );
    }

    return (
      <Group orientation={node.direction}>
        {node.children.map((child, index) => {
          const key = child.nodeType === "group" ? child.id : child.data.id;

          return (
            <Fragment key={key}>
              <Panel defaultSize={100 / node.children.length} minSize={15}>
                <WorkspaceNodeRenderer node={child} totalSpaces={totalSpaces} />
              </Panel>
              {index < node.children.length - 1 && (
                <Handle orientation={node.direction} />
              )}
            </Fragment>
          );
        })}
      </Group>
    );
  }

  return null;
}

export function WorkspaceGrid() {
  const root = useWorkspaceStore((state) => state.root);
  const isBooting = useWorkspaceStore((state) => state.isBooting);

  if (isBooting) {
    return (
      <div className="flex h-full w-full animate-pulse flex-col items-center justify-center bg-background font-semibold text-muted-foreground">
        <Loader2 className="animate-spin" />
        <span>Loading Workspace...</span>
      </div>
    );
  }

  if (!root || (root.nodeType === "group" && root.children.length === 0)) {
    return (
      <div className="flex h-full w-full items-center justify-center bg-background font-bold text-muted-foreground text-sm uppercase tracking-widest">
        No Active Workspaces
      </div>
    );
  }

  const totalSpaces = countTotalPanels(root);

  return (
    <div className="h-full w-full overflow-hidden bg-background">
      <WorkspaceNodeRenderer node={root} totalSpaces={totalSpaces} />
    </div>
  );
}
