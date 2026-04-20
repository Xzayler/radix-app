import StyledCheckbox from '~/components/forms/StyledCheckbox';

export default function FavouritesToggle(props: {
  value: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <StyledCheckbox
      value={props.value}
      onChange={props.onChange}
      label={
        <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
          Favourited
        </span>
      }
    />
  );
}
