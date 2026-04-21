import { For, Suspense } from 'solid-js';
import SystemEntry from './SystemEntry';
import { columns } from './systemTableColumns';
import { System } from '~/types';

export default function SystemsTable(props: { systems: System[] }) {
  return (
    <div class="w-full overflow-x-auto">
      <table class="w-full border-collapse bg-highlight rounded-md p-2 overflow-hidden">
        <thead>
          <tr class="">
            {columns.map((col) => (
              <th class="px-2 py-3 text-left text-xs font-semibold uppercase tracking-wider text-foreground">
                {col.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          <Suspense fallback={<></>}>
            <For each={props.systems}>
              {(system) => <SystemEntry system={system} />}
            </For>
          </Suspense>
        </tbody>
      </table>
    </div>
  );
}
