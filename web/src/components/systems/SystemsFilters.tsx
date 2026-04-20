import { SystemsFilter } from '~/lib/db/operations';
import NaturalNumberInput from '../forms/NumberInput';
import BasePrefixFilter from './filters/BasePrefixFilter';
import { createEffect } from 'solid-js';
import GnsFilter from './filters/GnsFilter';
import NameFilter from './filters/NameFilter';
import DigitTypeFilter from './filters/DigitTypeFilter';
import { DigitKind } from '~/types';

export default function SystemsFilters(props: {
  value: SystemsFilter;
  setValue: (v: SystemsFilter) => void;
}) {
  createEffect(() => {
    console.log(props.value);
  });

  return (
    <div class="flex flex-wrap items-start gap-3 p-4 border-b border-faint">
      <div class="flex flex-col gap-1">
        <NaturalNumberInput
          label={
            <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
              Dim
            </span>
          }
          name="dim"
          value={props.value.dim}
          onChange={(dim: number | undefined) => {
            const fs = structuredClone(props.value);
            fs.dim = dim;
            props.setValue(fs);
          }}
        />
      </div>

      <div class="flex flex-col gap-1" onClick={(e) => e.preventDefault()}>
        <NameFilter
          value={props.value.name}
          onChange={(name: string | undefined) => {
            const fs = structuredClone(props.value);
            fs.name = name;
            props.setValue(fs);
          }}
        />
      </div>

      <div class="flex flex-col gap-1">
        <GnsFilter
          value={props.value.gns}
          onChange={(gns: boolean | undefined) => {
            const fs = structuredClone(props.value);
            fs.gns = gns;
            props.setValue(fs);
          }}
        />
      </div>

      <div class="flex flex-col gap-1">
        <BasePrefixFilter
          value={props.value.basePrefix}
          onChange={(prefix: number[] | undefined) => {
            const fs = structuredClone(props.value);
            fs.basePrefix = prefix;
            props.setValue(fs);
          }}
        />
      </div>

      <div class="w-32">
        <DigitTypeFilter
          value={props.value.digitType}
          onChange={(digitType: DigitKind | undefined) => {
            const fs = structuredClone(props.value);
            fs.digitType = digitType;
            props.setValue(fs);
          }}
        />
      </div>

      {/* Digits search */}
      {/* <div class="flex flex-col gap-1">
        <label class="text-xs font-medium text-muted-foreground uppercase tracking-wider">
          Digits
        </label>
        <Input
          type="text"
          placeholder="e.g. [1,0] or [[1,0],[0,1]]"
          class="w-48 h-8 text-sm font-mono"
          defaultValue=""
          onBlur={(e) => update({ digits: parseDigits(e.target.value) })}
          onKeyDown={(e) => {
            if (e.key === 'Enter') {
              update({
                digits: parseDigits((e.target as HTMLInputElement).value),
              });
            }
          }}
        />
      </div> */}

      {/* Clear */}
      {/* {hasFilters && (
        <Button
          variant="ghost"
          size="sm"
          class="h-8 px-2 text-muted-foreground"
          onClick={() => {
            setFilters({});
            onFiltersChange({});
          }}
        >
          <X class="h-4 w-4 mr-1" />
          Clear
        </Button>
      )} */}
    </div>
  );
}
