import { SystemsFilter } from '~/lib/db/operations';
import NaturalNumberInput from '../forms/NaturalNumberInput';
import BasePrefixFilter from './filters/BasePrefixFilter';
import { createEffect } from 'solid-js';
import GnsFilter from './filters/GnsFilter';
import NameFilter from './filters/NameFilter';
import DigitTypeFilter from './filters/DigitTypeFilter';
import { DigitType } from '~/types';
import FavouritesToggle from './filters/FavouritesToggle';
import OwnToggle from './filters/OwnToggle';

export default function SystemsFilters(props: {
  value: SystemsFilter;
  setValue: (v: SystemsFilter) => void;
}) {
  createEffect(() => {
    console.log(props.value);
  });

  return (
    <div class="">
      <div class="flex flex-wrap items-start p-2 gap-3 rounded-md bg-highlight ">
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
            onChange={(digitType: DigitType | undefined) => {
              const fs = structuredClone(props.value);
              fs.digitType = digitType;
              props.setValue(fs);
            }}
          />
        </div>

        <div>
          <FavouritesToggle
            value={props.value.filterFavourites ?? false}
            onChange={(b) => {
              const fs = structuredClone(props.value);
              fs.filterFavourites = b;
              props.setValue(fs);
            }}
          />
        </div>

        <div>
          <OwnToggle
            value={props.value.filterOwnedByUser ?? false}
            onChange={(b) => {
              const fs = structuredClone(props.value);
              fs.filterOwnedByUser = b;
              props.setValue(fs);
            }}
          />
        </div>

        <div class="flex flex-col gap-1 ml-auto">
          <NaturalNumberInput
            label={
              <span class="text-xs font-semibold uppercase tracking-wider text-foreground">
                PageSize
              </span>
            }
            name="pageSize"
            value={props.value.pageSize}
            onChange={(ps: number | undefined) => {
              const fs = structuredClone(props.value);
              fs.pageSize = ps ?? 1;
              props.setValue(fs);
            }}
          />
        </div>
      </div>
    </div>
  );
}
