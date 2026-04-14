import StyledSelect from '~/components/forms/StyledSelect';

type GnsOptionsType = 'Yes' | 'No' | 'Any';

export default function GnsFilter(props: {
  value: boolean | undefined;
  onChange: (v: boolean | undefined) => void;
}) {
  function gnsOptionToBool(input: GnsOptionsType | null): boolean | undefined {
    switch (input) {
      case 'Yes':
        return true;
      case 'No':
        return false;
      case 'Any':
      case null:
        return undefined;
    }
  }

  function boolToGnsOption(input: boolean | undefined) {
    switch (input) {
      case true:
        return 'Yes';
      case false:
        return 'No';
      case undefined:
        return 'Any';
    }
  }

  return (
    <StyledSelect<GnsOptionsType>
      label={
        <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
          Gns
        </span>
      }
      name="gns"
      options={['Yes', 'No', 'Any']}
      defaultValue="Any"
      value={boolToGnsOption(props.value)}
      onChange={(newVal: GnsOptionsType | null) => {
        console.log('Updating gns to ', newVal);
        props.onChange(gnsOptionToBool(newVal));
      }}
    />
  );
}
