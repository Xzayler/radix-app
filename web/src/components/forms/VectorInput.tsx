import { TextField } from '@kobalte/core/text-field';
import { createSignal } from 'solid-js';

export default function VectorInput(props: {
  name: string;
  label: string;
  dim: number;
  disabled: boolean;
  value?: string;
  setValue?: (s: string) => void;
  placeholder?: string;
}) {
  let [validationState, setValidationState] = createSignal<
    'valid' | 'invalid' | undefined
  >('valid');
  const [error, setError] = createSignal<string>('');

  let inputRef: HTMLInputElement | ((el: HTMLInputElement) => void) | undefined;
  const isVectorStringValid = (vs: string): boolean => {
    const vectorString = vs.trim();
    const vectorValueRegex = /^-?\d+(?: -?\d+)*$/g;
    const regmatch = vectorValueRegex.test(vectorString);
    const inputEl = inputRef as HTMLInputElement;
    if (!regmatch) {
      const msg = 'Input format is invalid';
      inputEl.setCustomValidity(msg);
      setError(msg);
      return false;
    }
    const values = vectorString.split(' ').map((s) => parseInt(s));
    if (values.length != props.dim) {
      const msg = "The point's dimensions doesn't match the system's";
      inputEl.setCustomValidity(msg);
      setError(msg);
      return false;
    }
    if (!values.every((v) => !isNaN(v))) {
      const msg = 'The vector elements should be integers';
      inputEl.setCustomValidity(msg);
      setError(msg);
      return false;
    }
    inputEl.setCustomValidity('');
    setError('');
    return true;
  };

  const validateInput = (e: InputEvent) => {
    const element = e.target as HTMLInputElement;
    const isValid = isVectorStringValid(element.value);
    if (props.disabled || isValid) {
      setValidationState('valid');
    } else {
      setValidationState('invalid');
    }
  };

  return (
    <TextField
      name={props.name}
      value={props.value}
      onChange={props.setValue}
      validationState={validationState()}
      onInput={validateInput}
    >
      <TextField.Label class="block">{props.label}</TextField.Label>
      <TextField.Input
        ref={inputRef}
        disabled={props.disabled}
        placeholder={props.placeholder}
        class="px-2 border-2 rounded-md border-ui disabled:bg-faint data-invalid:border-red-700 valid:border-ui"
      />
      <TextField.ErrorMessage class="text-xs text-red-700">
        {error()}
      </TextField.ErrorMessage>
    </TextField>
  );
}
