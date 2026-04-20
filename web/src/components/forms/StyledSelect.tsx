import { Select } from '@kobalte/core/select';
import { JSX } from 'solid-js';

export default function StyledSelect<T extends string>(props: {
  label: string | JSX.Element;
  options: T[];
  name: string;
  value?: T;
  onChange?: (v: T) => void;
  placeholder?: string;
  defaultValue?: T;
}) {
  return (
    <Select
      value={props.value}
      onChange={(v: T | null) => {
        if (props.onChange && v) props.onChange(v);
      }}
      options={props.options}
      placeholder={props.placeholder}
      name={props.name}
      selectionBehavior="replace"
      disallowEmptySelection={true}
      sameWidth={true}
      defaultValue={props.defaultValue}
      itemComponent={(props) => (
        <Select.Item
          item={props.item}
          class=" text-base text-foreground flex justify-between outline-none rounded-md px-2 data-highlighted:bg-accent"
        >
          <Select.ItemLabel>{props.item.rawValue}</Select.ItemLabel>
        </Select.Item>
      )}
    >
      <Select.HiddenSelect required />
      <Select.Label>{props.label}</Select.Label>
      <Select.Trigger
        class={
          'flex items-center justify-between px-3 border-2 border-ui rounded-md w-full'
        }
      >
        <Select.Value class="text-ellipsis whitespace-nowrap overflow-hidden data-placeholder-shown:text-foreground/50 mr-2">
          {(state) => <>{state.selectedOption()}</>}
        </Select.Value>
        <Select.Icon>v</Select.Icon>
      </Select.Trigger>
      <Select.Description />
      <Select.ErrorMessage />
      <Select.Portal>
        <Select.Content class=" rounded-lg border border-ui overflow-hidden ">
          <Select.Listbox class=" overflow-y-auto p-1 bg-background " />
        </Select.Content>
      </Select.Portal>
    </Select>
  );
}
