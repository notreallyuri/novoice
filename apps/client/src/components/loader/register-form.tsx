import { useForm } from "@tanstack/react-form";
import { invoke } from "@tauri-apps/api/core";
import { Eye, EyeClosed, Loader2 } from "lucide-react";
import { useState } from "react";
import {
  RegisterRequest,
  RegisterStepAccount,
  RegisterStepProfile,
} from "@/types/dtos";
import { Button } from "../ui/button";
import { Field, FieldError, FieldGroup, FieldLabel } from "../ui/field";
import { Input } from "../ui/input";

export function RegisterForm() {
  const [step, setStep] = useState<"account" | "profile">("account");
  const [passwordVisible, setPasswordVisible] = useState(false);

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
        console.log("Submitting to Rust:", value);
        await invoke("register_user", { payload: value });
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
          onGroupSubmit={() => {
            setStep("profile");
          }}
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
          onGroupSubmit={() => {
            form.handleSubmit();
          }}
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
                        <Input
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
    </div>
  );
}
