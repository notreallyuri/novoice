import { useForm } from "@tanstack/react-form";
import { invoke } from "@tauri-apps/api/core";
import { Camera, Eye, EyeClosed, Loader2 } from "lucide-react";
import { useState } from "react";
import { DialogCropper } from "@/components/dialogs/dialog-cropper";
import type { CropResult } from "@/components/image-cropper";
import { Textarea } from "@/components/ui/textarea";
import {
  generateCroppedImage,
  useImageSelection,
} from "@/hooks/use-image-selection";
import { cn } from "@/lib/utils";
import {
  RegisterRequest,
  RegisterStepAccount,
  RegisterStepProfile,
} from "@/types/dtos";
import { Button } from "../../ui/button";
import { Field, FieldError, FieldGroup, FieldLabel } from "../../ui/field";
import { Input } from "../../ui/input";

export function RegisterForm() {
  const [step, setStep] = useState<"account" | "profile">("account");
  const [passwordVisible, setPasswordVisible] = useState(false);

  const avatarSelection = useImageSelection("Pick Avatar");
  const bannerSelection = useImageSelection("Select a banner");

  const [activeCropper, setActiveCropper] = useState<
    "avatar" | "banner" | null
  >(null);
  const [avatarCrop, setAvatarCrop] = useState<CropResult | null>(null);
  const [bannerCrop, setBannerCrop] = useState<CropResult | null>(null);

  const form = useForm({
    defaultValues: {
      account: { username: "", email: "", password: "" },
      profile: {
        display_name: "",
        bio: "",
        avatar_url: null,
        banner_url: null,
      },
    } as RegisterRequest,
    validators: {
      onSubmit: RegisterRequest,
    },
    onSubmit: async ({ value }) => {
      try {
        const payload = {
          ...value,
          profile: {
            ...value.profile,
            bio: value.profile.bio ?? null,
            avatar_url: null,
            banner_url: null,
          },
        };

        console.log("Submitting:", payload, "Crop:", avatarCrop);

        await invoke("register_user", {
          payload,
          avatarPath: avatarSelection.originalPath,
          avatarCrop,
          bannerPath: bannerSelection.originalPath,
          bannerCrop,
        });
      } catch (err) {
        console.error(err);
      }
    },
  });

  return (
    <div className="flex w-full max-w-86 flex-col gap-4">
      {step === "account" && (
        <form.FormGroup
          name="account"
          onGroupSubmit={() => setStep("profile")}
          validators={{ onChange: RegisterStepAccount }}
        >
          {(group) => (
            <form
              className="fade-in slide-in-from-right-4 flex animate-in flex-col gap-4"
              onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                group.handleSubmit();
              }}
            >
              <FieldGroup>
                <form.Field name="account.email">
                  {(field) => {
                    const isInvalid =
                      field.state.meta.isTouched &&
                      field.state.meta.errors.length > 0;
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel>Email</FieldLabel>
                        <Input
                          autoComplete="off"
                          onBlur={field.handleBlur}
                          onChange={(e) => field.handleChange(e.target.value)}
                          type="email"
                          value={field.state.value}
                        />
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    );
                  }}
                </form.Field>

                <form.Field name="account.username">
                  {(field) => {
                    const isInvalid =
                      field.state.meta.isTouched &&
                      field.state.meta.errors.length > 0;
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel>Username</FieldLabel>
                        <Input
                          autoComplete="off"
                          onBlur={field.handleBlur}
                          onChange={(e) => field.handleChange(e.target.value)}
                          value={field.state.value}
                        />
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    );
                  }}
                </form.Field>

                <form.Field name="account.password">
                  {(field) => {
                    const isInvalid =
                      field.state.meta.isTouched &&
                      field.state.meta.errors.length > 0;
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel>Password</FieldLabel>
                        <div className="relative">
                          <Input
                            autoComplete="off"
                            className="pr-10"
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
                            type={passwordVisible ? "text" : "password"}
                            value={field.state.value}
                          />
                          <button
                            className="absolute top-1/2 right-2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                            onClick={() => setPasswordVisible(!passwordVisible)}
                            tabIndex={-1}
                            type="button"
                          >
                            {passwordVisible ? (
                              <Eye size={18} />
                            ) : (
                              <EyeClosed size={18} />
                            )}
                          </button>
                        </div>
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    );
                  }}
                </form.Field>
              </FieldGroup>

              <Button type="submit">Next: Setup Profile</Button>
            </form>
          )}
        </form.FormGroup>
      )}

      {step === "profile" && (
        <form.FormGroup
          name="profile"
          onGroupSubmit={() => form.handleSubmit()}
          validators={{ onChange: RegisterStepProfile }}
        >
          {(group) => (
            <form
              className="fade-in slide-in-from-right-4 flex animate-in flex-col gap-4"
              onSubmit={(e) => {
                e.preventDefault();
                e.stopPropagation();
                group.handleSubmit();
              }}
            >
              <FieldGroup>
                <div className="relative aspect-16/6 w-full">
                  <form.Field name="profile.banner_url">
                    {(field) => (
                      <div className="size-full">
                        <button
                          className={cn(
                            "group size-full cursor-pointer border border-dashed bg-muted/25",
                            field.state.meta.errors.length > 0
                              ? ""
                              : "border-border bg-muted/25 hover:bg-muted"
                          )}
                          onClick={async () => {
                            await bannerSelection.handleSelectImage();
                            setActiveCropper("banner");
                          }}
                          type="button"
                        >
                          {field.state.value && (
                            <>
                              <div className="pointer-events-none absolute size-full transition-colors group-hover:bg-muted/25" />
                              {/*biome-ignore lint/correctness/useImageSize: "ignore"*/}
                              <img
                                alt="Banner"
                                className="size-full object-cover"
                                src={field.state.value}
                              />
                            </>
                          )}
                        </button>
                      </div>
                    )}
                  </form.Field>
                  <form.Field name="profile.avatar_url">
                    {(field) => (
                      <div className="flex flex-col items-center gap-2">
                        <button
                          className={cn(
                            "group absolute bottom-2 left-2 flex size-20 shrink-0 cursor-pointer items-center justify-center overflow-hidden rounded-full border border-dashed transition-all",
                            field.state.meta.errors.length > 0
                              ? "border-destructive bg-destructive/10"
                              : "border-border bg-muted/25 hover:border-primary/50 hover:bg-muted"
                          )}
                          onClick={async () => {
                            await avatarSelection.handleSelectImage();
                            setActiveCropper("avatar");
                          }}
                          type="button"
                        >
                          {field.state.value ? (
                            <>
                              <div className="pointer-events-none absolute size-full transition-colors group-hover:bg-muted/25" />
                              {/* biome-ignore lint/correctness/useImageSize: "ignore" */}
                              <img
                                alt="Avatar"
                                className="size-full object-cover"
                                src={field.state.value}
                              />
                            </>
                          ) : (
                            <Camera className="size-6 text-muted-foreground transition-colors group-hover:text-foreground" />
                          )}
                        </button>
                        {field.state.meta.errors.length > 0 && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </div>
                    )}
                  </form.Field>
                </div>

                <form.Field name="profile.display_name">
                  {(field) => {
                    const isInvalid =
                      field.state.meta.isTouched &&
                      field.state.meta.errors.length > 0;
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel>Display Name</FieldLabel>
                        <Input
                          autoComplete="off"
                          onBlur={field.handleBlur}
                          onChange={(e) => field.handleChange(e.target.value)}
                          value={field.state.value}
                        />
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    );
                  }}
                </form.Field>

                <form.Field name="profile.bio">
                  {(field) => {
                    const isInvalid =
                      field.state.meta.isTouched &&
                      field.state.meta.errors.length > 0;
                    return (
                      <Field data-invalid={isInvalid}>
                        <FieldLabel>Bio (Optional)</FieldLabel>
                        <Textarea
                          autoComplete="off"
                          onBlur={field.handleBlur}
                          onChange={(e) => field.handleChange(e.target.value)}
                          value={field.state.value || ""}
                        />
                        {isInvalid && (
                          <FieldError errors={field.state.meta.errors} />
                        )}
                      </Field>
                    );
                  }}
                </form.Field>
              </FieldGroup>

              <div className="mt-2 flex gap-2">
                <Button
                  className="w-1/3"
                  onClick={() => setStep("account")}
                  type="button"
                  variant="outline"
                >
                  Back
                </Button>
                <form.Subscribe selector={(state) => [state.isSubmitting]}>
                  {([isSubmitting]) => (
                    <Button
                      className="w-2/3"
                      disabled={isSubmitting}
                      type="submit"
                    >
                      {isSubmitting ? (
                        <Loader2 className="animate-spin" />
                      ) : (
                        "Complete"
                      )}
                    </Button>
                  )}
                </form.Subscribe>
              </div>
            </form>
          )}
        </form.FormGroup>
      )}

      <DialogCropper
        aspect={activeCropper === "avatar" ? 1 : 16 / 6}
        circular={activeCropper === "avatar"}
        isOpen={
          (activeCropper === "avatar" && !!avatarSelection.previewUrl) ||
          (activeCropper === "banner" && !!bannerSelection.previewUrl)
        }
        onClose={() => {
          if (
            activeCropper === "avatar" &&
            !form.getFieldValue("profile.avatar_url")
          ) {
            avatarSelection.clearSelection();
          } else if (
            activeCropper === "banner" &&
            !form.getFieldValue("profile.banner_url")
          ) {
            bannerSelection.clearSelection();
          }
          setActiveCropper(null);
        }}
        onSuccess={async (crop) => {
          const isAvatar = activeCropper === "avatar";
          const selection = isAvatar ? avatarSelection : bannerSelection;
          const fieldName = isAvatar
            ? "profile.avatar_url"
            : "profile.banner_url";

          if (isAvatar) {
            setAvatarCrop(crop);
          } else {
            setBannerCrop(crop);
          }

          setActiveCropper(null);

          if (selection.previewUrl) {
            const croppedUrl = await generateCroppedImage(
              selection.previewUrl,
              crop
            );
            form.setFieldValue(fieldName, croppedUrl);
          }
        }}
        previewUrl={
          activeCropper === "avatar"
            ? avatarSelection.previewUrl
            : bannerSelection.previewUrl
        }
        title={
          activeCropper === "avatar"
            ? "Position your Avatar"
            : "Position your Banner"
        }
      />
    </div>
  );
}
