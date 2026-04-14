import { TextField } from '@kobalte/core/text-field';
import { Accessor, JSX } from 'solid-js';

export default function TextInput(props: {
  name: string;
  label?: JSX.Element;
  placeholder?: string;
  value?: string | undefined;
  onChange?: (v: string) => void;
  error?: string;
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
    >
      <TextField.Label>{props.label}</TextField.Label>
      <TextField.Input
        placeholder={props.placeholder}
        class="px-2 border-2 rounded-md border-ui disabled:bg-faint data-invalid:border-red-700 valid:border-ui"
      />
      <div class="h-4 ">
        <TextField.ErrorMessage class="text-xs text-red-700">
          {props.error}
        </TextField.ErrorMessage>
      </div>
    </TextField>
  );
}
