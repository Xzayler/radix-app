import { Select } from '@kobalte/core/select';
import { Accessor } from 'solid-js';

export default function StyledSelect<T extends string>(props: {
  label: string;
  options: T[];
  name: string;
  value?: Accessor<T | null | undefined>;
  onChange?: (v: T | null) => void;
  defaultValue?: T;
  placeholder?: string;
}) {
  return (
    <Select
      value={props.value ? props.value() : undefined}
      onChange={props.onChange}
      options={props.options}
      defaultValue={props.defaultValue}
      placeholder={props.placeholder}
      name={props.name}
      selectionBehavior="replace"
      disallowEmptySelection
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
      <Select.Trigger class="flex items-center justify-between px-3 border border-ui rounded-md w-28 outline-none ">
        <Select.Value class=" text-ellipsis whitespace-nowrap overflow-hidden data-placeholder-shown:text-foreground/50 ">
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
