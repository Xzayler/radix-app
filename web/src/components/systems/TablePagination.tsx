import { Button } from '@kobalte/core/button';

export default function TablePagination(props: {
  value: number;
  onChange: (v: number) => void;
  hasNext: boolean;
}) {
  return (
    <div class="flex items-center justify-between">
      <Button
        disabled={props.value == 1}
        class="border border-faint rounded-md w-22 py-1 cursor-pointer transition-colors hover:border-ui text-lg disabled:text-faint"
        onClick={() => {
          props.onChange(props.value - 1);
        }}
      >
        Previous
      </Button>
      <div
        class=""
        onClick={(e) => {
          console.log('V: ' + props.hasNext);
        }}
      >
        <span>{`Page ${props.value}`}</span>
      </div>
      <Button
        disabled={!props.hasNext}
        class="border border-faint rounded-md w-22 py-1 cursor-pointer transition-colors hover:border-ui text-lg disabled:text-faint"
        onClick={() => {
          props.onChange(props.value + 1);
        }}
      >
        Next
      </Button>
    </div>
  );
}
