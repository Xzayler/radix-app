import { For } from 'solid-js';
import SystemEntry from './SystemEntry';
import { columns } from './systemTableColumns';
import { getSystems } from '~/api/server';
import { createAsync } from '@solidjs/router';

export default function SystemsTable() {
  const systems = createAsync(() => getSystems({ digits: undefined }));

  return (
    <div class="rounded-lg border border-faint bg-card">
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
            <For each={systems()}>
              {(system) => <SystemEntry system={system} />}
            </For>
          </tbody>
        </table>
      </div>
    </div>
  );
}
