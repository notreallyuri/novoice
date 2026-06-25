import * as React from "react";
import { cn } from "@/lib/utils";
import TextAreaAutoResize, {
  TextareaAutosizeProps,
} from "react-textarea-autosize";

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaAutosizeProps>(
  ({ className, ...props }, ref) => {
    return (
      <TextAreaAutoResize
        ref={ref}
        minRows={1}
        maxRows={6}
        data-slot="textarea"
        className={cn(className)}
        {...props}
      />
    );
  },
);
Textarea.displayName = "Textarea";

export { Textarea };
