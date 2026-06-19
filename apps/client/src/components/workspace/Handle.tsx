import { EllipsisVertical } from "lucide-react";
import { Separator } from "react-resizable-panels";

export function Handle() {
  return (
    <Separator className="w-2 bg-background hover:bg-primary/20 transition-colors flex flex-col items-center justify-center cursor-col-resize group data-[resize-handle-state=drag]:bg-primary/30">
      <div className="h-8 w-1 transition-colors flex flex-col gap-1 items-center justify-center">
        <EllipsisVertical />
      </div>
    </Separator>
  );
}
