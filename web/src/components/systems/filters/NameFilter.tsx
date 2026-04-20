import { createEffect, createSignal } from 'solid-js';
import TextInput from '~/components/forms/TextInput';

// function stringifyArr(arr: number[] | undefined): string | undefined {
//   return arr ? arr.join(' ') : undefined;
// }

export default function NameFilter(props: {
  value: string | undefined;
  onChange: (p: string | undefined) => void;
}) {
  return (
    <TextInput
      label={
        <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
          Name
        </span>
      }
      name="name"
      placeholder="My System"
      value={props.value}
      onChange={props.onChange}
    />
  );
}
