import { Checkbox } from '@kobalte/core/checkbox';
import { JSX } from 'solid-js';
import CheckIcon from '../shared/CheckIcon';

export default function StyledCheckbox(props: {
  value: boolean;
  onChange: (v: boolean) => void;
  label: JSX.Element;
}) {
  return (
    <Checkbox checked={props.value} onChange={props.onChange}>
      <Checkbox.Label class="">{props.label}</Checkbox.Label>
      <Checkbox.Control class="h-7 w-7 aspect-square rounded-md bg-highlight border-2 border-ui cursor-pointer overflow-hidden hover:bg-accent">
        <Checkbox.Indicator>
          <div class="h-full w-full bg-accent text-foreground">
            <CheckIcon />
          </div>
        </Checkbox.Indicator>
      </Checkbox.Control>
      <Checkbox.Input class="hidden" />
    </Checkbox>
  );
}
