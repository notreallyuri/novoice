import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { useState } from "react";
import type { CropResult } from "@/components/image-cropper";

// biome-ignore lint/suspicious/useAwait: we need to use an await since image loading ain't instant
export async function generateCroppedImage(
  imageUrl: string,
  crop: CropResult
): Promise<string> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.src = imageUrl;
    image.onload = () => {
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");
      if (!ctx) {
        return reject(new Error("No 2d context"));
      }

      const targetX = crop.x * crop.scaleX;
      const targetY = crop.y * crop.scaleY;
      const targetWidth = crop.width * crop.scaleX;
      const targetHeight = crop.height * crop.scaleY;

      canvas.width = targetWidth;
      canvas.height = targetHeight;

      ctx.drawImage(
        image,
        targetX,
        targetY,
        targetWidth,
        targetHeight,
        0,
        0,
        targetWidth,
        targetHeight
      );

      resolve(canvas.toDataURL("image/png", 1.0));
    };
    image.onerror = reject;
  });
}

export function useImageSelection(title?: string) {
  const [originalPath, setOriginalPath] = useState<string | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [isSelecting, setIsSelecting] = useState<boolean>(false);

  const handleSelectImage = async () => {
    setIsSelecting(true);
    try {
      const selected = await open({
        title: title ?? "Pick File",
        multiple: false,
        pickerMode: "image",
        filters: [
          { name: "Image", extensions: ["png", "jpg", "jpeg", "gif", "webp"] },
        ],
      });

      if (selected && typeof selected === "string") {
        setOriginalPath(selected);

        const contents = await readFile(selected);
        const ext = selected.split(".").pop()?.toLowerCase();
        const mimeMap: Record<string, string> = {
          png: "image/png",
          jpg: "image/jpeg",
          jpeg: "image/jpeg",
          gif: "image/gif",
          webp: "image/webp",
        };

        const mime = mimeMap[ext ?? ""] ?? "image/png";
        const blob = new Blob([contents], { type: mime });
        const blobURL = URL.createObjectURL(blob);
        setPreviewUrl(blobURL);
      }
    } catch (err) {
      console.error(err);
    } finally {
      setIsSelecting(false);
    }
  };

  const clearSelection = () => {
    setOriginalPath(null);
    setPreviewUrl(null);
  };

  return {
    originalPath,
    previewUrl,
    isSelecting,
    handleSelectImage,
    clearSelection,
  };
}
