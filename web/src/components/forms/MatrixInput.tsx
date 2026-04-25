import { createEffect, createSignal, JSX } from 'solid-js';
import TextAreaInput from './TextAreaInput';
import { validateStringAsFlatMatrix } from '~/lib/validators';

const toMatrix = (baseStr: string, size: number): number[][] => {
  if (!baseStr.length) {
    return [];
  }
  baseStr = baseStr.trim();
  baseStr = baseStr.replaceAll('\n', ' ');
  const base = baseStr.split(' ').map((s) => parseInt(s));
  let splitArr: number[][] = [];
  while (base.length && splitArr.length < size) {
    splitArr.push(base.splice(0, size));
  }
  return splitArr;
};

export default function MatrixInput(props: {
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

  let inputRef: HTMLInputElement | ((el: HTMLInputElement) => void) | undefined;

  function validateInput(): number[][] | undefined {
    const inputEl = inputRef as HTMLInputElement;
    try {
      const res = validateStringAsFlatMatrix(inputStr(), props.dim);
      setError(undefined);
      inputEl.setCustomValidity('');
      return res;
    } catch (e) {
      const eMessage = (e as Error).message;
      setError(eMessage);
      inputEl.setCustomValidity(eMessage);
      return undefined;
    }
  }

  createEffect(() => {
    if (inputStr().trim().length == 0) {
      props.setValue([]);
      return;
    }
    const validated = validateInput();
    if (validated) {
      props.setValue(validated);
    } else {
      props.setValue(toMatrix(inputStr(), props.dim));
    }
  });

  return (
    <TextAreaInput
      ref={inputRef}
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
