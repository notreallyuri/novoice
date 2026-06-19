import { useState } from "react";
import { motion, useDragControls } from "motion/react";
import {
  MicOff,
  Mic,
  Settings,
  PhoneOff,
  Headphones,
  HeadphoneOff,
  GripVertical,
  Group,
  ChevronDown,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";

export function FloatingCallManager() {
  const [open, _setOpen] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [isDeafen, setIsDeafen] = useState(false);

  const dragControls = useDragControls();

  return (
    <motion.div
      drag
      dragControls={dragControls}
      dragListener={false}
      onDragStart={() => {
        document.body.style.userSelect = "none";
        document.body.style.setProperty("-webkit-user-select", "none");
      }}
      onDragEnd={() => {
        document.body.style.userSelect = "";
        document.body.style.removeProperty("-webkit-user-select");
      }}
      dragConstraints={{
        top: -40,
        left: -(window.innerWidth - 300),
        right: 40,
        bottom: window.innerHeight - 250,
      }}
      dragMomentum={false}
      className={cn(
        "absolute top-10 right-10 z-50 flex bg-black border border-border shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] cursor-grab active:cursor-grabbing",
        !open && "hidden",
      )}
    >
      <div
        onPointerDown={(e) => dragControls.start(e)}
        style={{ touchAction: "none" }}
        className="flex justify-between items-center cursor-grab active:cursor-grabbing hover:bg-muted/50 transition-colors"
      >
        <GripVertical />
      </div>

      <div className="flex justify-between p-2 gap-2">
        <Popover>
          <PopoverTrigger
            render={
              <button className="p-2 border h-10 flex items-center justify-between gap-3 border-border hover:bg-muted transition-colors outline-none focus:ring-1 focus:ring-primary">
                <ChevronDown size={16} className="text-muted-foreground" />
                <div className="flex -space-x-2">
                  <div className="size-5 rounded-full bg-[#86efac] border border-[oklch(0.1344_0_0)] z-30" />
                  <div className="size-5 rounded-full bg-[#93c5fd] border border-[oklch(0.1344_0_0)] z-20" />
                  <div className="size-5 rounded-full bg-[#fca5a5] border border-[oklch(0.1344_0_0)] z-10" />
                </div>
              </button>
            }
          ></PopoverTrigger>
          <PopoverContent
            side="bottom"
            align="start"
            sideOffset={12}
            className="w-48 p-0 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] rounded-none bg-[oklch(0.1344_0_0)]"
          >
            <div className="flex flex-col">
              <div className="p-2 border-b border-border bg-muted/20">
                <span className="text-xs font-bold uppercase tracking-widest text-muted-foreground">
                  In Call (3)
                </span>
              </div>
              <div className="flex flex-col p-2 gap-1">
                <div className="flex items-center gap-2 p-1 hover:bg-muted/50 cursor-pointer transition-colors">
                  <div className="size-4 rounded-full bg-[#86efac]" />
                  <span className="text-sm font-medium">Yuri (You)</span>
                </div>
                <div className="flex items-center gap-2 p-1 hover:bg-muted/50 cursor-pointer transition-colors">
                  <div className="size-4 rounded-full bg-[#93c5fd]" />
                  <span className="text-sm">Alex</span>
                </div>
                <div className="flex items-center gap-2 p-1 hover:bg-muted/50 cursor-pointer transition-colors">
                  <div className="size-4 rounded-full bg-[#fca5a5]" />
                  <span className="text-sm">Sarah</span>
                </div>
              </div>
            </div>
          </PopoverContent>
        </Popover>

        <button
          onClick={() => setIsMuted(!isMuted)}
          className={cn(
            "p-2 border size-10 flex items-center justify-center border-border hover:bg-muted transition-colors",
            isMuted && "bg-destructive/20 text-destructive border-destructive",
          )}
        >
          {isMuted ? <MicOff size={18} /> : <Mic size={18} />}
        </button>
        <button
          onClick={() => setIsDeafen(!isDeafen)}
          className={cn(
            "p-2 border size-10 flex items-center justify-center border-border hover:bg-muted transition-colors",
            isDeafen && "bg-destructive/20 text-destructive border-destructive",
          )}
        >
          {isDeafen ? <HeadphoneOff size={18} /> : <Headphones size={18} />}
        </button>
        <button
          className={cn(
            "p-2 border h-10 border-border flex items-center justify-center hover:bg-muted transition-colors flex-col",
          )}
        >
          <Group size={18} />
          <ChevronDown size={18} />
        </button>
        <button className="p-2 flex items-center justify-center size-10 border border-border hover:bg-muted transition-colors">
          <Settings size={18} />
        </button>
        <button className="p-2 border flex items-center justify-center size-10 border-destructive bg-destructive/10 text-destructive hover:bg-destructive hover:text-white transition-colors">
          <PhoneOff size={18} />
        </button>
      </div>
    </motion.div>
  );
}
