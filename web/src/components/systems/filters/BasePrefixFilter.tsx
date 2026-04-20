import { createEffect, createSignal } from 'solid-js';
import TextInput from '~/components/forms/TextInput';

function stringifyArr(arr: number[] | undefined): string | undefined {
  return arr ? arr.join(' ') : undefined;
}

export default function BasePrefixFilter(props: {
  value: number[] | undefined;
  onChange: (p: number[] | undefined) => void;
}) {
  const [input, setInput] = createSignal<string>('');
  const [error, setError] = createSignal<string | undefined>();

  const isVectorStringValid = (vs: string) => {
    const vectorString = vs.trim();
    if (vectorString.length == 0) {
      setError(undefined);
      props.onChange(undefined);
      return;
    }
    const vectorValueRegex = /^-?\d+(?: -?\d+)*$/g;
    const regmatch = vectorValueRegex.test(vectorString);
    if (!regmatch) {
      setError('Input format is invalid');
      props.onChange(undefined);
      return;
    }
    const values = vectorString.split(' ').map((s) => parseInt(s));
    if (!values.every((v) => !isNaN(v))) {
      setError('The vector elements should be integers');
      props.onChange(undefined);
      return;
    }
    setError(undefined);
    props.onChange(values);
  };

  return (
    <TextInput
      label={
        <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
          Base Prefix
        </span>
      }
      name="base-prefix"
      placeholder="1 0 2..."
      value={stringifyArr(props.value)}
      onChange={isVectorStringValid}
      error={error()}
    />
  );
}
