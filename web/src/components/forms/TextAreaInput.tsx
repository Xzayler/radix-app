import { TextField } from '@kobalte/core/text-field';
import { JSX } from 'solid-js';

export default function TextAreaInput(props: {
  name: string;
  label: JSX.Element;
  placeholder?: string;
  value?: string | undefined;
  onChange?: (v: string) => void;
  error?: string;
  maxLength?: number;
  minLength?: number;
  required?: boolean;
  disabled?: boolean;
  ref?: HTMLInputElement | ((el: HTMLInputElement) => void) | undefined;
}) {
  return (
    <TextField
      class="flex flex-col"
      value={props.value}
      onChange={props.onChange}
      name={props.name}
      validationState={
        props.error && props.error.length != 0 ? 'invalid' : 'valid'
      }
      required={props.required}
      disabled={props.disabled}
    >
      <TextField.Label>{props.label}</TextField.Label>
      <TextField.TextArea
        ref={props.ref}
        autoResize
        placeholder={props.placeholder}
        maxLength={props.maxLength}
        minLength={props.minLength}
        class="px-2 border-2 disabled:bg-ui/30 rounded-md border-ui data-invalid:border-red-700 valid:border-ui"
      />
      <div class="h-4">
        <TextField.ErrorMessage class="text-sm text-red-700 ">
          {props.error}
        </TextField.ErrorMessage>
      </div>
    </TextField>
  );
}
