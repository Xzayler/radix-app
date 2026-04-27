import StyledSelect from '~/components/forms/StyledSelect';
import { DigitType } from '~/types';

type FilterOption = DigitType | 'Any';
type FilterValue = DigitType | undefined;

export default function DigitTypeFilter(props: {
  value: FilterValue;
  onChange: (v: FilterValue) => void;
}) {
  function optionToValue(option: FilterOption): FilterValue {
    if (option == 'Any') return undefined;
    return option;
  }

  function valueToOption(value: FilterValue): FilterOption {
    if (value === undefined) return 'Any';
    return value;
  }

  return (
    <StyledSelect<FilterOption>
      label={
        <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
          Digit Type
        </span>
      }
      name="gns"
      options={[
        'Any',
        'Explicit',
        'Canonical',
        'JCanonical',
        'Symmetric',
        'JSymmetric',
        'Shifted',
        'Adjoint',
      ]}
      value={valueToOption(props.value)}
      onChange={(selected: FilterOption) => {
        props.onChange(optionToValue(selected));
      }}
    />
  );
}
