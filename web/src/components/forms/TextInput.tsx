import { TextField } from '@kobalte/core/text-field';
import { JSX } from 'solid-js';

export default function TextInput(props: {
  name: string;
  label?: JSX.Element;
  placeholder?: string;
  value?: string | undefined;
  onChange?: (v: string) => void;
  error?: string;
  maxLength?: number;
  minLength?: number;
  required?: boolean;
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
      required={props.required ?? false}
    >
      <TextField.Label>{props.label}</TextField.Label>
      <TextField.Input
        placeholder={props.placeholder}
        maxLength={props.maxLength}
        minLength={props.minLength}
        class="px-2 border-2 rounded-md border-ui disabled:bg-faint data-invalid:border-red-700 valid:border-ui"
      />
      {/* <div class="h-4 ">
        <TextField.ErrorMessage class="text-xs text-red-700">
          {props.error}
        </TextField.ErrorMessage>
      </div> */}
    </TextField>
  );
}
