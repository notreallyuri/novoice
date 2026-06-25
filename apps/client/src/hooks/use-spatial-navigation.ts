import { useEffect } from "react";
import { useWorkspaceStore } from "@/store/workspace-store";

type Direction = "left" | "down" | "up" | "right";

const VIM_KEYS: Record<string, Direction> = {
  h: "left",
  j: "down",
  k: "up",
  l: "right",
};

const getCenter = (el: Element) => {
  const rect = el.getBoundingClientRect();
  return {
    x: rect.left + rect.width / 2,
    y: rect.top + rect.height / 2,
  };
};

const isTargetValid = (
  dir: Direction,
  curX: number,
  curY: number,
  targetX: number,
  targetY: number
) => {
  switch (dir) {
    case "left":
      return targetX < curX;
    case "right":
      return targetX > curX;
    case "up":
      return targetY < curY;
    case "down":
      return targetY > curY;
    default:
      return false;
  }
};

const findNextPanel = (
  dir: Direction,
  currentEl: Element,
  allPanels: Element[]
): string | null => {
  const currentCenter = getCenter(currentEl);
  let closestId: string | null = null;
  let minDistance = Number.POSITIVE_INFINITY;

  for (const panel of allPanels) {
    if (panel === currentEl) {
      continue;
    }

    const targetCenter = getCenter(panel);

    if (
      isTargetValid(
        dir,
        currentCenter.x,
        currentCenter.y,
        targetCenter.x,
        targetCenter.y
      )
    ) {
      const distance = Math.hypot(
        targetCenter.x - currentCenter.x,
        targetCenter.y - currentCenter.y
      );

      if (distance < minDistance) {
        minDistance = distance;
        closestId = panel.getAttribute("data-panel-id");
      }
    }
  }

  return closestId;
};

export function useSpatialNavigation() {
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!(e.ctrlKey || e.metaKey)) {
        return;
      }

      const dir = VIM_KEYS[e.key];
      if (!dir) {
        return;
      }

      const allPanels = Array.from(
        document.querySelectorAll("[data-panel-id]")
      );
      if (allPanels.length <= 1) {
        return;
      }

      const { activePanelId, setActivePanel } = useWorkspaceStore.getState();
      if (!activePanelId) {
        return;
      }

      const currentEl = document.querySelector(
        `[data-panel-id="${activePanelId}"]`
      );
      if (!currentEl) {
        return;
      }

      e.preventDefault();

      const closestId = findNextPanel(dir, currentEl, allPanels);
      if (closestId) {
        setActivePanel(closestId);
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);
}
