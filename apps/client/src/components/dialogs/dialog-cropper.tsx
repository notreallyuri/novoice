import {
  type CropResult,
  ImageCrop,
  ImageCropApply,
  ImageCropContent,
  ImageCropReset,
} from "../image-cropper";
import { Button } from "../ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "../ui/dialog";

interface Props {
  aspect?: number;
  circular?: boolean;
  isOpen?: boolean;
  onClose: () => void;
  onSuccess: (crop: CropResult) => void;
  previewUrl: string | null;
  title?: string;
}

export function DialogCropper({
  isOpen: controlledOpen,
  previewUrl,
  onClose,
  onSuccess,
  aspect = 1,
  circular = false,
  title = "Crop Image",
}: Props) {
  const isOpen = controlledOpen ?? !!previewUrl;

  return (
    <Dialog onOpenChange={(open) => !open && onClose()} open={isOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col items-center justify-center py-4">
          {previewUrl && (
            <ImageCrop
              aspect={aspect}
              circularCrop={circular}
              imageUrl={previewUrl}
              onCrop={onSuccess}
            >
              <ImageCropContent className="max-w-md" />
              <div className="mt-4 flex items-center justify-end gap-2">
                <ImageCropReset
                  render={
                    <Button size="sm" variant="ghost">
                      Reset Zoom
                    </Button>
                  }
                />
                <Button onClick={onClose} size="sm" variant="ghost">
                  Cancel
                </Button>
                <ImageCropApply
                  render={
                    <Button size="sm" variant="ghost">
                      Apply Crop
                    </Button>
                  }
                />
              </div>
            </ImageCrop>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
