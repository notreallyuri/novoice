import {
  ChevronDown,
  GripVertical,
  Group,
  HeadphoneOff,
  Headphones,
  Mic,
  MicOff,
  PhoneOff,
  Settings,
} from "lucide-react";
import { motion, useDragControls } from "motion/react";
import { cn } from "@/lib/utils";
import { useCallStore } from "@/store/call-store";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";

export function FloatingCallManager() {
  const { isActive, isMuted, isDeafened, toggleMute, toggleDeafen, leaveCall } =
    useCallStore();

  const dragControls = useDragControls();

  if (!isActive) {
    return null;
  }

  return (
    <motion.div
      className={cn(
        "absolute top-10 right-10 z-50 flex cursor-grab border border-border bg-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] active:cursor-grabbing"
      )}
      drag
      dragConstraints={{
        top: -40,
        left: -(window.innerWidth - 300),
        right: 40,
        bottom: window.innerHeight - 250,
      }}
      dragControls={dragControls}
      dragListener={false}
      dragMomentum={false}
      onDragEnd={() => {
        document.body.style.userSelect = "";
        document.body.style.removeProperty("-webkit-user-select");
      }}
      onDragStart={() => {
        document.body.style.userSelect = "none";
        document.body.style.setProperty("-webkit-user-select", "none");
      }}
    >
      <div
        className="flex cursor-grab items-center justify-between transition-colors hover:bg-muted/50 active:cursor-grabbing"
        onPointerDown={(e) => dragControls.start(e)}
        style={{ touchAction: "none" }}
      >
        <GripVertical />
      </div>

      <div className="flex justify-between gap-2 p-2">
        <Popover>
          <PopoverTrigger
            render={
              <button className="flex h-10 items-center justify-between gap-3 border border-border p-2 outline-none transition-colors hover:bg-muted focus:ring-1 focus:ring-primary">
                <ChevronDown className="text-muted-foreground" size={16} />
                <div className="flex -space-x-2">
                  <div className="z-30 size-5 rounded-full border border-[oklch(0.1344_0_0)] bg-[#86efac]" />
                  <div className="z-20 size-5 rounded-full border border-[oklch(0.1344_0_0)] bg-[#93c5fd]" />
                  <div className="z-10 size-5 rounded-full border border-[oklch(0.1344_0_0)] bg-[#fca5a5]" />
                </div>
              </button>
            }
          />
          <PopoverContent
            align="start"
            className="w-48 rounded-none bg-[oklch(0.1344_0_0)] p-0 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]"
            side="bottom"
            sideOffset={12}
          >
            <div className="flex flex-col">
              <div className="border-border border-b bg-muted/20 p-2">
                <span className="font-bold text-muted-foreground text-xs uppercase tracking-widest">
                  In Call (3)
                </span>
              </div>
              <div className="flex flex-col gap-1 p-2">
                <div className="flex cursor-pointer items-center gap-2 p-1 transition-colors hover:bg-muted/50">
                  <div className="size-4 rounded-full bg-[#86efac]" />
                  <span className="font-medium text-sm">Yuri (You)</span>
                </div>
                <div className="flex cursor-pointer items-center gap-2 p-1 transition-colors hover:bg-muted/50">
                  <div className="size-4 rounded-full bg-[#93c5fd]" />
                  <span className="text-sm">Alex</span>
                </div>
                <div className="flex cursor-pointer items-center gap-2 p-1 transition-colors hover:bg-muted/50">
                  <div className="size-4 rounded-full bg-[#fca5a5]" />
                  <span className="text-sm">Sarah</span>
                </div>
              </div>
            </div>
          </PopoverContent>
        </Popover>

        <button
          className={cn(
            "flex size-10 items-center justify-center border border-border p-2 transition-colors hover:bg-muted",
            isMuted && "border-destructive bg-destructive/20 text-destructive"
          )}
          onClick={toggleMute}
        >
          {isMuted ? <MicOff size={18} /> : <Mic size={18} />}
        </button>
        <button
          className={cn(
            "flex size-10 items-center justify-center border border-border p-2 transition-colors hover:bg-muted",
            isDeafened &&
              "border-destructive bg-destructive/20 text-destructive"
          )}
          onClick={toggleDeafen}
        >
          {isDeafened ? <HeadphoneOff size={18} /> : <Headphones size={18} />}
        </button>
        <button
          className={cn(
            "flex h-10 flex-col items-center justify-center border border-border p-2 transition-colors hover:bg-muted"
          )}
        >
          <Group size={18} />
          <ChevronDown size={18} />
        </button>
        <button className="flex size-10 items-center justify-center border border-border p-2 transition-colors hover:bg-muted">
          <Settings size={18} />
        </button>
        <button
          className="flex size-10 items-center justify-center border border-destructive bg-destructive/10 p-2 text-destructive transition-colors hover:bg-destructive hover:text-white"
          onClick={leaveCall}
        >
          <PhoneOff size={18} />
        </button>
      </div>
    </motion.div>
  );
}
