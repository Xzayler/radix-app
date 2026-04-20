import { NumberField } from '@kobalte/core/number-field';
import { JSX } from 'solid-js';

function parseInput(v: string): number | undefined {
  if (v.length == 0) {
    return undefined;
  }
  const n = parseInt(v);
  if (Number.isNaN(n) || n == 0) {
    return undefined;
  }
  return parseInt(v);
}

function filterNumbers(e: InputEvent) {
  const element = e.target as HTMLInputElement;
  element.value = element.value.replace(/[^[0-9-]]/g, '');
}

export default function NaturalNumberInput(props: {
  label: string | JSX.Element;
  name: string;
  value: number | undefined;
  onChange: (v: number | undefined) => void;
}) {
  return (
    <NumberField
      class="flex flex-col"
      value={props.value ?? ''}
      onChange={(v: string) => {
        props.onChange(parseInput(v));
      }}
      minValue={0}
    >
      <NumberField.Label>{props.label}</NumberField.Label>
      <NumberField.Input
        class="flex no-spinner items-center justify-between px-2 border-2 border-ui rounded-md w-10"
        type="number"
        name={props.name}
        onInput={filterNumbers}
      />
      <NumberField.ErrorMessage />
    </NumberField>
  );
}
