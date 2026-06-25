import { Ellipsis, EllipsisVertical } from "lucide-react";
import { Separator } from "react-resizable-panels";
import { cn } from "@/lib/utils";

export function Handle({
  orientation,
}: {
  orientation?: "horizontal" | "vertical";
}) {
  return (
    <Separator
      className={cn(
        "group flex items-center justify-center bg-background transition-colors hover:bg-border data-[resize-handle-state=drag]:bg-primary/30",
        orientation === "horizontal"
          ? "w-2 cursor-col-resize flex-col"
          : "h-2 cursor-row-resize flex-row"
      )}
    >
      <div
        className={cn(
          "flex items-center justify-center transition-colors",
          orientation === "horizontal"
            ? "h-8 w-1 flex-col gap-1"
            : "h-1 w-8 flex-row gap-1"
        )}
      >
        {orientation === "horizontal" ? (
          <EllipsisVertical size={20} />
        ) : (
          <Ellipsis size={20} />
        )}
      </div>
    </Separator>
  );
}
