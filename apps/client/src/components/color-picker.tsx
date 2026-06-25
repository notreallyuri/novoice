import { Slider } from "@base-ui/react/slider";
import Color from "color";
import { PipetteIcon } from "lucide-react";
import {
  type ComponentProps,
  createContext,
  type HTMLAttributes,
  memo,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { cn } from "@/lib/utils";

interface ColorPickerContextValue {
  alpha: number;
  hue: number;
  lightness: number;
  mode: string;
  saturation: number;
  setAlpha: (alpha: number) => void;
  setHue: (hue: number) => void;
  setLightness: (lightness: number) => void;
  setMode: (mode: string) => void;
  setSaturation: (saturation: number) => void;
}

const ColorPickerContext = createContext<ColorPickerContextValue | undefined>(
  undefined
);

export const useColorPicker = () => {
  const context = useContext(ColorPickerContext);
  if (!context) {
    throw new Error("useColorPicker must be used within a ColorPicker");
  }
  return context;
};

export type ColorPickerProps = Omit<
  HTMLAttributes<HTMLDivElement>,
  "onChange"
> & {
  value?: Parameters<typeof Color>[0];
  defaultValue?: Parameters<typeof Color>[0];
  onChange?: (value: InstanceType<typeof Color>) => void;
};

export function ColorPicker({
  value,
  defaultValue = "#000000",
  onChange,
  className,
  ...props
}: ColorPickerProps) {
  function parse(v: Parameters<typeof Color>[0]) {
    try {
      return Color(v).hsl();
    } catch {
      return Color(defaultValue).hsl();
    }
  }

  const initial = parse(value ?? defaultValue);

  const [hue, setHue] = useState(initial.hue());
  const [saturation, setSaturation] = useState(initial.saturationl());
  const [lightness, setLightness] = useState(initial.lightness());
  const [alpha, setAlpha] = useState(initial.alpha() * 100);
  const [mode, setMode] = useState("hex");

  useEffect(() => {
    if (value === undefined) {
      return;
    }
    try {
      const c = Color(value).hsl();
      setHue(c.hue());
      setSaturation(c.saturationl());
      setLightness(c.lightness());
      setAlpha(c.alpha() * 100);
    } catch {
      /* ignore */
    }
  }, [value]);

  useEffect(() => {
    onChange?.(Color.hsl(hue, saturation, lightness).alpha(alpha / 100));
  }, [hue, saturation, lightness, alpha, onChange]);

  return (
    <ColorPickerContext.Provider
      value={{
        hue,
        saturation,
        lightness,
        alpha,
        mode,
        setHue,
        setSaturation,
        setLightness,
        setAlpha,
        setMode,
      }}
    >
      <div
        className={cn("flex size-full flex-col gap-4", className)}
        {...props}
      />
    </ColorPickerContext.Provider>
  );
}

export type ColorPickerSelectionProps = HTMLAttributes<HTMLDivElement>;

export const ColorPickerSelection = memo(
  ({ className, ...props }: ColorPickerSelectionProps) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const [isDragging, setIsDragging] = useState(false);
    const [positionX, setPositionX] = useState(0);
    const [positionY, setPositionY] = useState(0);
    const { hue, setSaturation, setLightness } = useColorPicker();

    const background = useMemo(
      () =>
        "linear-gradient(0deg,rgba(0,0,0,1),rgba(0,0,0,0))," +
        "linear-gradient(90deg,rgba(255,255,255,1),rgba(255,255,255,0))," +
        `hsl(${hue},100%,50%)`,
      [hue]
    );

    const handleMove = useCallback(
      (event: PointerEvent) => {
        if (!(isDragging && containerRef.current)) {
          return;
        }
        const rect = containerRef.current.getBoundingClientRect();
        const x = Math.max(
          0,
          Math.min(1, (event.clientX - rect.left) / rect.width)
        );
        const y = Math.max(
          0,
          Math.min(1, (event.clientY - rect.top) / rect.height)
        );
        setPositionX(x);
        setPositionY(y);
        setSaturation(x * 100);
        setLightness((x < 0.01 ? 100 : 50 + 50 * (1 - x)) * (1 - y));
      },
      [isDragging, setSaturation, setLightness]
    );

    useEffect(() => {
      const up = () => setIsDragging(false);
      if (isDragging) {
        window.addEventListener("pointermove", handleMove);
        window.addEventListener("pointerup", up);
      }
      return () => {
        window.removeEventListener("pointermove", handleMove);
        window.removeEventListener("pointerup", up);
      };
    }, [isDragging, handleMove]);

    return (
      <div
        className={cn("relative size-full cursor-crosshair rounded", className)}
        onPointerDown={(e) => {
          e.preventDefault();
          setIsDragging(true);
          handleMove(e.nativeEvent);
        }}
        ref={containerRef}
        style={{ background }}
        {...props}
      >
        <div
          className="pointer-events-none absolute size-4 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white"
          style={{
            left: `${positionX * 100}%`,
            top: `${positionY * 100}%`,
            boxShadow: "0 0 0 1px rgba(0,0,0,0.5)",
          }}
        />
      </div>
    );
  }
);
ColorPickerSelection.displayName = "ColorPickerSelection";

export type ColorPickerHueProps = ComponentProps<typeof Slider.Root>;

export function ColorPickerHue({ className, ...props }: ColorPickerHueProps) {
  const { hue, setHue } = useColorPicker();
  return (
    <Slider.Root
      className={cn("relative flex h-4 w-full touch-none", className)}
      max={360}
      onValueChange={(val) => setHue(typeof val === "number" ? val : val[0])}
      step={1}
      value={[hue]}
      {...props}
    >
      <Slider.Track className="relative my-0.5 h-3 w-full grow rounded-full bg-[linear-gradient(90deg,#FF0000,#FFFF00,#00FF00,#00FFFF,#0000FF,#FF00FF,#FF0000)]">
        <Slider.Indicator className="absolute h-full" />
      </Slider.Track>
      <Slider.Thumb className="block h-4 w-4 rounded-full border border-primary/50 bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50" />
    </Slider.Root>
  );
}

export type ColorPickerAlphaProps = ComponentProps<typeof Slider.Root>;

export function ColorPickerAlpha({
  className,
  ...props
}: ColorPickerAlphaProps) {
  const { alpha, setAlpha } = useColorPicker();
  return (
    <Slider.Root
      className={cn("relative flex h-4 w-full touch-none", className)}
      max={100}
      onValueChange={(val) => setAlpha(typeof val === "number" ? val : val[0])}
      step={1}
      value={[alpha]}
      {...props}
    >
      <Slider.Track className="relative my-0.5 h-3 w-full grow overflow-hidden rounded-full bg-[url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAMUlEQVQ4T2NkYGAQYcAP3uCTZhw1gGGYhAGBZIA/nYDCgBDAm9BGDWAAJyRCgLaBCAAgXwixzAS0pgAAAABJRU5ErkJggg==')] bg-repeat">
        <div className="absolute inset-0 rounded-full bg-linear-to-r from-transparent to-black/50 dark:to-white/50" />
        <Slider.Indicator className="absolute h-full bg-transparent" />
      </Slider.Track>
      <Slider.Thumb className="block h-4 w-4 rounded-full border border-primary/50 bg-background shadow transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50" />
    </Slider.Root>
  );
}

export type ColorPickerEyeDropperProps = ComponentProps<typeof Button>;

export function ColorPickerEyeDropper({
  className,
  ...props
}: ColorPickerEyeDropperProps) {
  const { setHue, setSaturation, setLightness, setAlpha } = useColorPicker();

  const handleEyeDropper = async () => {
    try {
      // @ts-expect-error — EyeDropper API is experimental
      const eyeDropper = new EyeDropper();
      const result = await eyeDropper.open();
      const c = Color(result.sRGBHex).hsl();
      setHue(c.hue());
      setSaturation(c.saturationl());
      setLightness(c.lightness());
      setAlpha(100);
    } catch {
      /* cancelled or unsupported */
    }
  };

  return (
    <Button
      className={cn("shrink-0 text-muted-foreground", className)}
      onClick={handleEyeDropper}
      size="icon"
      type="button"
      variant="outline"
      {...props}
    >
      <PipetteIcon size={16} />
    </Button>
  );
}

export type ColorPickerOutputProps = ComponentProps<typeof SelectTrigger>;

const FORMATS = ["hex", "rgb", "css", "hsl"] as const;

export function ColorPickerOutput({
  className,
  ...props
}: ColorPickerOutputProps) {
  const { mode, setMode } = useColorPicker();
  return (
    <Select
      onValueChange={(v) => {
        if (v) {
          setMode(v);
        }
      }}
      value={mode}
    >
      <SelectTrigger
        className={cn("h-8 w-20 shrink-0 text-xs", className)}
        {...props}
      >
        <SelectValue placeholder="Mode" />
      </SelectTrigger>
      <SelectContent>
        {FORMATS.map((f) => (
          <SelectItem className="text-xs" key={f} value={f}>
            {f.toUpperCase()}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

type PercentageInputProps = ComponentProps<typeof Input>;

function PercentageInput({ className, ...props }: PercentageInputProps) {
  return (
    <div className="relative">
      <Input
        className={cn(
          "h-8 w-13 rounded-l-none bg-secondary px-2 text-xs shadow-none",
          className
        )}
        readOnly
        type="text"
        {...props}
      />
      <span className="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground text-xs">
        %
      </span>
    </div>
  );
}

export type ColorPickerFormatProps = HTMLAttributes<HTMLDivElement>;

export function ColorPickerFormat({
  className,
  ...props
}: ColorPickerFormatProps) {
  const { hue, saturation, lightness, alpha, mode } = useColorPicker();
  const color = Color.hsl(hue, saturation, lightness).alpha(alpha / 100);

  if (mode === "hex") {
    return (
      <div
        className={cn(
          "relative flex w-full items-center -space-x-px rounded-md shadow-sm",
          className
        )}
        {...props}
      >
        <Input
          className="h-8 rounded-r-none bg-secondary px-2 text-xs shadow-none"
          readOnly
          type="text"
          value={color.hex()}
        />
        <PercentageInput value={Math.round(alpha)} />
      </div>
    );
  }

  if (mode === "rgb") {
    const rgb = color.rgb().array().map(Math.round);
    return (
      <div
        className={cn(
          "flex items-center -space-x-px rounded-md shadow-sm",
          className
        )}
        {...props}
      >
        {rgb.map((v, i) => (
          <Input
            className={cn(
              "h-8 bg-secondary px-2 text-xs shadow-none",
              i === 0 ? "rounded-r-none" : "rounded-none"
            )}
            key={i}
            readOnly
            type="text"
            value={v}
          />
        ))}
        <PercentageInput value={Math.round(alpha)} />
      </div>
    );
  }

  if (mode === "css") {
    const [r, g, b] = color.rgb().array().map(Math.round);
    return (
      <div className={cn("w-full rounded-md shadow-sm", className)} {...props}>
        <Input
          className="h-8 w-full bg-secondary px-2 text-xs shadow-none"
          readOnly
          type="text"
          value={`rgba(${r}, ${g}, ${b}, ${Math.round(alpha)}%)`}
        />
      </div>
    );
  }

  if (mode === "hsl") {
    const hsl = color.hsl().array().map(Math.round);
    return (
      <div
        className={cn(
          "flex items-center -space-x-px rounded-md shadow-sm",
          className
        )}
        {...props}
      >
        {hsl.map((v, i) => (
          <Input
            className={cn(
              "h-8 bg-secondary px-2 text-xs shadow-none",
              i === 0 ? "rounded-r-none" : "rounded-none"
            )}
            key={i}
            readOnly
            type="text"
            value={v}
          />
        ))}
        <PercentageInput value={Math.round(alpha)} />
      </div>
    );
  }

  return null;
}
