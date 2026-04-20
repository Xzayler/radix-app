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
  // const [value, setValue] = createSignal<string>('');
  let [validationState, setValidationState] = createSignal<
    'valid' | 'invalid' | undefined
  >('valid');
  const [error, setError] = createSignal<string>('');

  const isVectorStringValid = (vs: string): boolean => {
    const vectorString = vs.trim();
    const vectorValueRegex = /^-?\d+(?: -?\d+)*$/g;
    const regmatch = vectorValueRegex.test(vectorString);
    if (!regmatch) {
      setError('Input format is invalid');
      return false;
    }
    const values = vectorString.split(' ').map((s) => parseInt(s));
    if (values.length != props.dim) {
      setError("The point's dimensions doesn't match the system's");
      return false;
    }
    if (!values.every((v) => !isNaN(v))) {
      setError('The vector elements should be integers');
      return false;
    }
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
