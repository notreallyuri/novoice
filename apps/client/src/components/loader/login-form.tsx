import { useForm } from "@tanstack/react-form";
import { Eye, EyeClosed } from "lucide-react";
import { useState } from "react";
import { LoginRequest } from "@/types/dtos";
import { Button } from "../ui/button";
import { Field, FieldError, FieldGroup, FieldLabel } from "../ui/field";
import { Input } from "../ui/input";

export function LoginForm() {
  const [visible, setVisible] = useState(false);

  const form = useForm({
    validators: {
      onChange: LoginRequest,
      onSubmit: LoginRequest,
    },
    defaultValues: {
      email: "",
      password: "",
    },
    onSubmit: ({ value }) => {
      const _ = value;
    },
  });

  return (
    <div className="flex w-full max-w-86 flex-col gap-4">
      <FieldGroup>
        <form.Field
          children={(field) => {
            const isInvalid =
              field.state.meta.isTouched && !field.state.meta.isValid;
            return (
              <Field data-invalid={isInvalid}>
                <FieldLabel>Email</FieldLabel>
                <Input
                  aria-invalid={isInvalid}
                  autoComplete="off"
                  id={field.name}
                  name={field.name}
                  onBlur={field.handleBlur}
                  onChange={(e) => field.handleChange(e.target.value)}
                  value={field.state.value}
                />
                {isInvalid && <FieldError errors={field.state.meta.errors} />}
              </Field>
            );
          }}
          name="email"
        />
        <form.Field
          children={(field) => {
            const isInvalid =
              field.state.meta.isTouched && !field.state.meta.isValid;
            return (
              <Field>
                <FieldLabel>Password</FieldLabel>
                <div className="relative">
                  <Input
                    aria-invalid={isInvalid}
                    autoComplete="off"
                    id={field.name}
                    name={field.name}
                    onBlur={field.handleBlur}
                    onChange={(e) => field.handleChange(e.target.value)}
                    type={visible ? "text" : "password"}
                    value={field.state.value}
                  />
                  <button
                    className="absolute top-1/2 right-2 -translate-y-1/2"
                    onClick={() => setVisible(!visible)}
                    type="button"
                  >
                    {visible ? <Eye size={18} /> : <EyeClosed size={18} />}
                  </button>
                </div>
                {isInvalid && <FieldError errors={field.state.meta.errors} />}
              </Field>
            );
          }}
          name="password"
        />
      </FieldGroup>
      <Button>Login</Button>
    </div>
  );
}
