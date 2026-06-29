import { RotateCcwIcon } from "lucide-react";
import {
  type ComponentProps,
  type CSSProperties,
  createContext,
  type MouseEvent,
  type ReactNode,
  type RefObject,
  type SyntheticEvent,
  useCallback,
  useContext,
  useRef,
  useState,
} from "react";
import ReactCrop, {
  centerCrop,
  makeAspectCrop,
  type PercentCrop,
  type PixelCrop,
  type ReactCropProps,
} from "react-image-crop";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import "react-image-crop/dist/ReactCrop.css";

const centerAspectCrop = (
  mediaWidth: number,
  mediaHeight: number,
  aspect: number | undefined
): PercentCrop =>
  centerCrop(
    aspect
      ? makeAspectCrop(
          { unit: "%", width: 90 },
          aspect,
          mediaWidth,
          mediaHeight
        )
      : { x: 0, y: 0, width: 90, height: 90, unit: "%" },
    mediaWidth,
    mediaHeight
  );

interface ImageCropContextType {
  applyCrop: () => void;
  completedCrop: PixelCrop | null;
  crop: PercentCrop | undefined;
  handleChange: (pixelCrop: PixelCrop, percentCrop: PercentCrop) => void;
  handleComplete: (pixelCrop: PixelCrop, percentCrop: PercentCrop) => void;
  imgRef: RefObject<HTMLImageElement | null>;
  imgSrc: string;
  onImageLoad: (e: SyntheticEvent<HTMLImageElement>) => void;
  reactCropProps: Omit<ReactCropProps, "onChange" | "onComplete" | "children">;
  resetCrop: () => void;
}

export interface CropResult {
  height: number;
  scaleX: number;
  scaleY: number;
  width: number;
  x: number;
  y: number;
}

export type ImageCropProps = {
  imageUrl: string;
  onCrop?: (result: CropResult) => void;
  children: ReactNode;
  onChange?: ReactCropProps["onChange"];
  onComplete?: ReactCropProps["onComplete"];
} & Omit<ReactCropProps, "onChange" | "onComplete" | "children">;

export interface ImageCropContentProps {
  className?: string;
  style?: CSSProperties;
}

export type ImageCropApplyProps = ComponentProps<"button"> & {
  render?: React.ReactElement;
};

const ImageCropContext = createContext<ImageCropContextType | null>(null);

const useImageCrop = () => {
  const context = useContext(ImageCropContext);
  if (!context) {
    throw new Error("ImageCrop components must be used within ImageCrop");
  }
  return context;
};

export const ImageCrop = ({
  imageUrl,
  onCrop,
  children,
  onChange,
  onComplete,
  ...reactCropProps
}: ImageCropProps) => {
  const imgRef = useRef<HTMLImageElement | null>(null);
  const [crop, setCrop] = useState<PercentCrop>();
  const [completedCrop, setCompletedCrop] = useState<PixelCrop | null>(null);
  const [initialCrop, setInitialCrop] = useState<PercentCrop>();

  const onImageLoad = useCallback(
    (e: SyntheticEvent<HTMLImageElement>) => {
      const { width, height } = e.currentTarget;
      const newCrop = centerAspectCrop(width, height, reactCropProps.aspect);
      setCrop(newCrop);
      setInitialCrop(newCrop);
    },
    [reactCropProps.aspect]
  );

  const handleChange = (pixelCrop: PixelCrop, percentCrop: PercentCrop) => {
    setCrop(percentCrop);
    onChange?.(pixelCrop, percentCrop);
  };

  const handleComplete = (pixelCrop: PixelCrop, percentCrop: PercentCrop) => {
    setCompletedCrop(pixelCrop);
    onComplete?.(pixelCrop, percentCrop);
  };

  const applyCrop = () => {
    if (!(imgRef.current && completedCrop)) {
      return;
    }

    const scaleX = imgRef.current.naturalWidth / imgRef.current.width;
    const scaleY = imgRef.current.naturalHeight / imgRef.current.height;

    onCrop?.({
      x: completedCrop.x,
      y: completedCrop.y,
      width: completedCrop.width,
      height: completedCrop.height,
      scaleX,
      scaleY,
    });
  };

  const resetCrop = () => {
    if (initialCrop) {
      setCrop(initialCrop);
      setCompletedCrop(null);
    }
  };

  return (
    <ImageCropContext.Provider
      value={{
        imgSrc: imageUrl,
        crop,
        completedCrop,
        imgRef,
        reactCropProps,
        handleChange,
        handleComplete,
        onImageLoad,
        applyCrop,
        resetCrop,
      }}
    >
      {children}
    </ImageCropContext.Provider>
  );
};

export const ImageCropContent = ({
  style,
  className,
}: ImageCropContentProps) => {
  const {
    imgSrc,
    crop,
    handleChange,
    handleComplete,
    onImageLoad,
    imgRef,
    reactCropProps,
  } = useImageCrop();

  const shadcnStyle = {
    "--rc-border-color": "var(--color-border)",
    "--rc-focus-color": "var(--color-primary)",
  } as CSSProperties;

  return (
    <ReactCrop
      className={cn("max-h-69.25 max-w-full", className)}
      crop={crop}
      onChange={handleChange}
      onComplete={handleComplete}
      style={{ ...shadcnStyle, ...style }}
      {...reactCropProps}
    >
      {imgSrc && (
        // biome-ignore lint/correctness/useImageSize: "does not need it..."
        // biome-ignore lint/a11y/noNoninteractiveElementInteractions: "does need it..."
        <img
          alt="crop"
          className="size-full"
          onLoad={onImageLoad}
          ref={imgRef}
          src={imgSrc}
        />
      )}
    </ReactCrop>
  );
};

export const ImageCropApply = ({
  render,
  children,
  onClick,
  ...props
}: ImageCropApplyProps) => {
  const { applyCrop } = useImageCrop();

  const handleClick = (e: MouseEvent<HTMLButtonElement>) => {
    applyCrop();
    onClick?.(e);
  };

  return (
    <Button onClick={handleClick} render={render} {...props}>
      {children}
    </Button>
  );
};

export type ImageCropResetProps = ComponentProps<"button"> & {
  render?: React.ReactElement;
};

export const ImageCropReset = ({
  render,
  children,
  onClick,
  ...props
}: ImageCropResetProps) => {
  const { resetCrop } = useImageCrop();

  const handleClick = (e: MouseEvent<HTMLButtonElement>) => {
    resetCrop();
    onClick?.(e);
  };

  return (
    <Button onClick={handleClick} render={render} variant="ghost" {...props}>
      {children ?? <RotateCcwIcon className="size-4" />}
    </Button>
  );
};
