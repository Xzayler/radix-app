import { createEffect, createSignal, For, Suspense } from 'solid-js';
import SystemEntry from './SystemEntry';
import { columns } from './systemTableColumns';
import { getSystems } from '~/api/server';
import { createAsync } from '@solidjs/router';
import SystemsFilters from './SystemsFilters';
import { SystemsFilter } from '~/lib/db/operations';

export default function SystemsTable() {
  const [filters, setFilters] = createSignal<SystemsFilter>({});
  const systems = createAsync(() => {
    return getSystems(filters());
  });

  return (
    <div class="rounded-lg border border-faint bg-card">
      <SystemsFilters value={filters()} setValue={setFilters} />
      <button onclick={() => console.log('clicked: ', filters())}>
        CLICK ME
      </button>
      <div class="w-full overflow-x-auto">
        <table class="w-full border-collapse">
          <thead>
            <tr class="border-b border-faint">
              {columns.map((col) => (
                <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-foreground">
                  {col.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            <Suspense fallback={<></>}>
              <For each={systems()}>
                {(system) => <SystemEntry system={system} />}
              </For>
            </Suspense>
          </tbody>
        </table>
      </div>
    </div>
  );
}
