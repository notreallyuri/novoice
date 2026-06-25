import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import { useState } from "react";

export function useImageSelection() {
  const [originalPath, setOriginalPath] = useState<string | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [isSelecting, setIsSelecting] = useState<boolean>(false);

  const handleSelectImage = async () => {
    setIsSelecting(true);
    try {
      const selected = await open({
        multiple: false,
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
