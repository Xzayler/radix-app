import { createEffect, createSignal, JSX } from 'solid-js';
import TextAreaInput from './TextAreaInput';
import { validateStringAsExplicitDigits } from '~/lib/utils/validators';

export default function VectorSetInput(props: {
  label: JSX.Element;
  name: string;
  setValue: (m: number[][]) => void;
  dim: number;
  placeholder?: string;
  disabled?: boolean;
  required?: boolean;
}) {
  const [error, setError] = createSignal<string>();
  const [inputStr, setInputStr] = createSignal<string>('');

  function validateInput(): number[][] | undefined {
    try {
      const res = validateStringAsExplicitDigits(inputStr(), props.dim);
      setError(undefined);
      return res;
    } catch (e) {
      setError((e as Error).message);
      return undefined;
    }
  }

  createEffect(() => {
    const validatedDigits = validateInput();
    if (validatedDigits) {
      if (inputStr().trim().length == 0) {
        props.setValue([]);
      } else {
        props.setValue(validatedDigits);
      }
    }
  });

  return (
    <TextAreaInput
      label={props.label}
      name={props.name}
      placeholder={props.placeholder}
      value={inputStr()}
      onChange={setInputStr}
      error={error()}
      disabled={props.disabled}
      required={props.required}
    />
  );
}
