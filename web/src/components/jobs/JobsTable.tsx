import { For } from 'solid-js';
import JobEntry from './JobEntry';
import { columns } from './jobTableColumn';
import { getJobs, getSystems } from '~/api/server';
import { createAsync } from '@solidjs/router';

export default function JobsTable() {
  const jobs = createAsync(() => getJobs({}));

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
            <For each={jobs()}>{(job) => <JobEntry job={job} />}</For>
          </tbody>
        </table>
      </div>
    </div>
  );
}
